<#
.SYNOPSIS
    Comprehensive System Report Generator
.DESCRIPTION
    Generates a detailed HTML system report with hardware, software, network,
    security, performance, and configuration information.
.NOTES
    Run as Administrator for complete information gathering.
#>

#Requires -Version 5.1
Set-StrictMode -Version Latest
$ErrorActionPreference = 'SilentlyContinue'

# ── Configuration ──────────────────────────────────────────────────────────────
$ReportTitle = "System Report"
$ComputerName = $env:COMPUTERNAME
$ReportDate = Get-Date -Format "yyyy-MM-dd HH:mm"
$OutputPath = ".\SystemReport_${ComputerName}_$(Get-Date -Format 'yyyyMMdd_HHmm').html"

# ── Data Collection Functions ──────────────────────────────────────────────────

function Get-SystemOverview {
    $os = Get-CimInstance Win32_OperatingSystem
    $cs = Get-CimInstance Win32_ComputerSystem
    $bios = Get-CimInstance Win32_BIOS
    $tz = Get-CimInstance Win32_TimeZone
    $bb = Get-CimInstance Win32_BaseBoard

    $bootTime = $os.LastBootUpTime
    $uptime = (Get-Date) - $bootTime
    $uptimeStr = "{0}d {1}h {2}m" -f $uptime.Days, $uptime.Hours, $uptime.Minutes

    $installDate = try {
        $raw = (Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion').InstallDate
        ([DateTime]'1970-01-01').AddSeconds($raw).ToLocalTime().ToString('yyyy-MM-dd HH:mm')
    }
    catch { $os.InstallDate.ToString('yyyy-MM-dd HH:mm') }

    $winVer = try {
        $nt = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'
        $nt.DisplayVersion
    }
    catch { "N/A" }

    $buildLab = try {
        (Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion').BuildLabEx
    }
    catch { "N/A" }

    @{
        ComputerName   = $cs.Name
        Domain         = if ($cs.PartOfDomain) { $cs.Domain } else { $cs.Workgroup + " (Workgroup)" }
        Manufacturer   = $cs.Manufacturer
        Model          = $cs.Model
        SystemType     = $cs.SystemType
        Motherboard    = "$($bb.Manufacturer) $($bb.Product)" -replace '\s+', ' '
        MotherboardSN  = $bb.SerialNumber
        BIOSVendor     = $bios.Manufacturer
        BIOSVersion    = $bios.SMBIOSBIOSVersion
        BIOSDate       = $bios.ReleaseDate.ToString('yyyy-MM-dd')
        SerialNumber   = $bios.SerialNumber
        OSName         = $os.Caption
        OSVersion      = $os.Version
        OSBuild        = $os.BuildNumber
        WinVersion     = $winVer
        BuildLab       = $buildLab
        OSArchitecture = $os.OSArchitecture
        InstallDate    = $installDate
        LastBoot       = $bootTime.ToString('yyyy-MM-dd HH:mm:ss')
        Uptime         = $uptimeStr
        TimeZone       = $tz.Caption
        SystemLocale   = (Get-Culture).DisplayName
        PageFileSize   = [math]::Round($os.TotalVirtualMemorySize / 1MB, 2)
        TotalPhysMemGB = [math]::Round($cs.TotalPhysicalMemory / 1GB, 2)
        FreePhysMemGB  = [math]::Round($os.FreePhysicalMemory / 1MB, 2)
        MemoryUsagePct = [math]::Round((1 - ($os.FreePhysicalMemory * 1KB / $cs.TotalPhysicalMemory)) * 100, 1)
    }
}

function Get-SecurityInfo {
    $secureBoot = try {
        $sb = Confirm-SecureBootUEFI
        if ($sb) { "Enabled" } else { "Disabled" }
    } catch { "Not Supported / Unknown" }

    $tpm = try {
        $tpmInfo = Get-Tpm -ErrorAction Stop
        
        $tpmVersion = try {
            $tpmWmi = Get-CimInstance -Namespace 'root\cimv2\security\microsofttpm' -ClassName Win32_Tpm -ErrorAction Stop
            $tpmWmi.SpecVersion.Split(',')[0].Trim()
        } catch { "Present (version unknown)" }
        
        $tpmManufacturer = try {
            $tpmWmi = Get-CimInstance -Namespace 'root\cimv2\security\microsofttpm' -ClassName Win32_Tpm -ErrorAction Stop
            $tpmWmi.ManufacturerIdTxt
        } catch { "Unknown" }

        @{
            Present      = $tpmInfo.TpmPresent
            Ready        = $tpmInfo.TpmReady
            Enabled      = $tpmInfo.TpmEnabled
            Activated    = $tpmInfo.TpmActivated
            Version      = $tpmVersion
            Manufacturer = $tpmManufacturer
        }
    } catch {
        try {
            $tpmWmi = Get-CimInstance -Namespace 'root\cimv2\security\microsofttpm' -ClassName Win32_Tpm -ErrorAction Stop
            $tpmVer = $tpmWmi.SpecVersion.Split(',')[0].Trim()
            $tpmMfr = $tpmWmi.ManufacturerIdTxt

            @{
                Present      = [bool]$tpmWmi
                Ready        = $tpmWmi.IsReady_InitialValue
                Enabled      = $tpmWmi.IsEnabled_InitialValue
                Activated    = $tpmWmi.IsActivated_InitialValue
                Version      = $tpmVer
                Manufacturer = $tpmMfr
            }
        } catch {
            try {
                $regPath = 'HKLM:\SYSTEM\CurrentControlSet\Services\TPM'
                $exists = Test-Path $regPath

                @{
                    Present      = $exists
                    Ready        = "Unknown (run as admin)"
                    Enabled      = "Unknown (run as admin)"
                    Activated    = "Unknown (run as admin)"
                    Version      = "Unknown (run as admin)"
                    Manufacturer = "Unknown (run as admin)"
                }
            } catch {
                @{
                    Present      = $false
                    Ready        = $false
                    Enabled      = $false
                    Activated    = $false
                    Version      = "N/A"
                    Manufacturer = "N/A"
                }
            }
        }
    }

    $av = Get-CimInstance -Namespace 'root\SecurityCenter2' -ClassName AntiVirusProduct |
        Select-Object displayName, productState, pathToSignedProductExe, timestamp

    $fw = try {
        Get-NetFirewallProfile | Select-Object Name, Enabled, DefaultInboundAction, DefaultOutboundAction
    } catch { @() }

    $uac = try {
        $reg = Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System'
        @{
            Enabled        = $reg.EnableLUA -eq 1
            ConsentPrompt  = $reg.ConsentPromptBehaviorAdmin
            PromptOnSecure = $reg.PromptOnSecureDesktop -eq 1
        }
    } catch {
        @{ Enabled = "Unknown"; ConsentPrompt = "Unknown"; PromptOnSecure = "Unknown" }
    }

    $rdp = try {
        $reg = Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Terminal Server'
        $reg.fDenyTSConnections -eq 0
    } catch { "Unknown" }

    $bitlocker = try {
        Get-BitLockerVolume | Select-Object MountPoint, VolumeStatus, EncryptionMethod, ProtectionStatus, LockStatus
    } catch { @() }

    $pendingUpdates = try {
        $searcher = (New-Object -ComObject Microsoft.Update.Session).CreateUpdateSearcher()
        $results  = $searcher.Search("IsInstalled=0")
        $results.Updates | ForEach-Object {
            @{ Title = $_.Title; Severity = $_.MsrcSeverity; KB = ($_.KBArticleIDs -join ', ') }
        }
    } catch { @() }

    @{
        SecureBoot     = $secureBoot
        TPM            = $tpm
        Antivirus      = $av
        Firewall       = $fw
        UAC            = $uac
        RDPEnabled     = $rdp
        BitLocker      = $bitlocker
        PendingUpdates = $pendingUpdates
    }
}

function Get-CPUInfo {
    Get-CimInstance Win32_Processor | ForEach-Object {
        @{
            Name              = $_.Name.Trim()
            DeviceID          = $_.DeviceID
            Manufacturer      = $_.Manufacturer
            Family            = $_.Family
            Cores             = $_.NumberOfCores
            LogicalProcessors = $_.NumberOfLogicalProcessors
            MaxClockMHz       = $_.MaxClockSpeed
            CurrentClockMHz   = $_.CurrentClockSpeed
            Socket            = $_.SocketDesignation
            L2CacheKB         = $_.L2CacheSize
            L3CacheKB         = $_.L3CacheSize
            Architecture      = switch ($_.Architecture) { 0 { "x86" } 5 { "ARM" } 9 { "x64" } 12 { "ARM64" } default { "Unknown" } }
            VTEnabled         = $_.VirtualizationFirmwareEnabled
            Status            = $_.Status
            LoadPct           = $_.LoadPercentage
            Voltage           = $_.CurrentVoltage
        }
    }
}

function Get-MemoryInfo {
    $slots = Get-CimInstance Win32_PhysicalMemory | ForEach-Object {
        @{
            Slot         = $_.DeviceLocator
            BankLabel    = $_.BankLabel
            SizeGB       = [math]::Round($_.Capacity / 1GB, 2)
            SpeedMHz     = $_.ConfiguredClockSpeed
            DataWidthBit = $_.DataWidth
            Type         = switch ($_.SMBIOSMemoryType) {
                20 { "DDR" } 21 { "DDR2" } 22 { "DDR2 FB-DIMM" } 24 { "DDR3" } 26 { "DDR4" } 34 { "DDR5" } default { "Type $($_.SMBIOSMemoryType)" }
            }
            FormFactor   = switch ($_.FormFactor) {
                8 { "DIMM" } 12 { "SODIMM" } default { "Other ($($_.FormFactor))" }
            }
            Manufacturer = $_.Manufacturer.Trim()
            PartNumber   = $_.PartNumber.Trim()
            SerialNumber = $_.SerialNumber.Trim()
        }
    }

    $array = Get-CimInstance Win32_PhysicalMemoryArray | Select-Object -First 1
    $totalSlots = $array.MemoryDevices
    $maxCapacityGB = [math]::Round($array.MaxCapacityEx / 1MB, 0)

    @{
        Slots      = $slots
        TotalSlots = $totalSlots
        UsedSlots  = ($slots | Measure-Object).Count
        MaxCapGB   = $maxCapacityGB
    }
}

function Get-GPUInfo {
    Get-CimInstance Win32_VideoController | ForEach-Object {
        $vramGB = if ($_.AdapterRAM -and $_.AdapterRAM -gt 0) {
            [math]::Round($_.AdapterRAM / 1GB, 2)
        }
        else { "Unknown" }

        @{
            Name           = $_.Name
            DeviceID       = $_.DeviceID
            DriverVersion  = $_.DriverVersion
            DriverDate     = if ($_.DriverDate) { $_.DriverDate.ToString('yyyy-MM-dd') } else { "N/A" }
            VideoProcessor = $_.VideoProcessor
            VRAM           = $vramGB
            Resolution     = "$($_.CurrentHorizontalResolution) x $($_.CurrentVerticalResolution)"
            RefreshRate    = "$($_.CurrentRefreshRate) Hz"
            BitsPerPixel   = $_.CurrentBitsPerPixel
            Status         = $_.Status
            Availability   = switch ($_.Availability) { 3 { "Running" } 8 { "Off-line" } default { "Other ($($_.Availability))" } }
            AdapterDACType = $_.AdapterDACType
        }
    }
}

function Get-MonitorInfo {
    Get-CimInstance -Namespace root\wmi -ClassName WmiMonitorID | ForEach-Object {
        $decode = { param($arr) if ($arr) { -join ($arr | Where-Object { $_ -ne 0 } | ForEach-Object { [char]$_ }) } else { "N/A" } }
        @{
            Manufacturer = & $decode $_.ManufacturerName
            Model        = & $decode $_.UserFriendlyName
            SerialNumber = & $decode $_.SerialNumberID
            YearMade     = $_.YearOfManufacture
            WeekMade     = $_.WeekOfManufacture
        }
    }
}

function Get-DiskPhysical {
    Get-CimInstance Win32_DiskDrive | Sort-Object Index | ForEach-Object {
        $sizeGB = [math]::Round($_.Size / 1GB, 2)

        $health = try {
            $msft = Get-PhysicalDisk | Where-Object {
                $_.FriendlyName -like "*$($_.Model.Trim().Substring(0, [Math]::Min(15, $_.Model.Trim().Length)))*"
            } | Select-Object -First 1
            if ($msft) { $msft.HealthStatus } else { "N/A" }
        } catch { "N/A" }

        $mediaType = try {
            $msft = Get-PhysicalDisk | Where-Object {
                $_.FriendlyName -like "*$($_.Model.Trim().Substring(0, [Math]::Min(15, $_.Model.Trim().Length)))*"
            } | Select-Object -First 1
            if ($msft) { $msft.MediaType } else { $_.MediaType }
        } catch { $_.MediaType }

        $serialNo = try { $_.SerialNumber } catch { "" }
        $fwVersion = try { $_.FirmwareRevision } catch { "" }

        @{
            Index       = $_.Index
            Model       = $_.Model.Trim()
            Interface   = $_.InterfaceType
            MediaType   = $mediaType
            SizeGB      = $sizeGB
            Partitions  = $_.Partitions
            Status      = $_.Status
            Health      = $health
            SerialNo    = $serialNo
            FWVersion   = $fwVersion
            BytesSector = $_.BytesPerSector
        }
    }
}

function Get-DiskLogical {
    Get-CimInstance Win32_LogicalDisk -Filter "DriveType=3" | ForEach-Object {
        $totalGB = [math]::Round($_.Size / 1GB, 2)
        $freeGB = [math]::Round($_.FreeSpace / 1GB, 2)
        $usedGB = $totalGB - $freeGB
        $usePct = if ($totalGB -gt 0) { [math]::Round(($usedGB / $totalGB) * 100, 1) } else { 0 }

        @{
            Drive      = $_.DeviceID
            Label      = $_.VolumeName
            FileSystem = $_.FileSystem
            TotalGB    = $totalGB
            FreeGB     = $freeGB
            UsedGB     = $usedGB
            UsagePct   = $usePct
            Compressed = $_.Compressed
        }
    }
}

function Get-NetworkAdapters {
    $configs = Get-CimInstance Win32_NetworkAdapterConfiguration -Filter "IPEnabled=True"
    $adapters = Get-CimInstance Win32_NetworkAdapter -Filter "NetEnabled=True"

    foreach ($cfg in $configs) {
        $adapter = $adapters | Where-Object { $_.Index -eq $cfg.Index }
        $speed = if ($adapter.Speed) {
            $s = $adapter.Speed
            if ($s -ge 1e9) { "$([math]::Round($s/1e9,1)) Gbps" }
            elseif ($s -ge 1e6) { "$([math]::Round($s/1e6,0)) Mbps" }
            else { "$s bps" }
        }
        else { "N/A" }

        @{
            Name            = $cfg.Description
            AdapterType     = $adapter.AdapterType
            NetConnectionID = $adapter.NetConnectionID
            Status          = if ($adapter.NetConnectionStatus -eq 2) { "Connected" } else { "Disconnected" }
            IPv4            = ($cfg.IPAddress | Where-Object { $_ -match '^\d+\.\d+\.\d+\.\d+$' }) -join ', '
            IPv6            = ($cfg.IPAddress | Where-Object { $_ -match ':' }) -join ', '
            Subnet          = ($cfg.IPSubnet | Select-Object -First 1)
            Gateway         = ($cfg.DefaultIPGateway -join ', ')
            DNS             = ($cfg.DNSServerSearchOrder -join ', ')
            DHCP            = $cfg.DHCPEnabled
            DHCPServer      = $cfg.DHCPServer
            DHCPLeaseObt    = if ($cfg.DHCPLeaseObtained) { $cfg.DHCPLeaseObtained.ToString('yyyy-MM-dd HH:mm') } else { "N/A" }
            DHCPLeaseExp    = if ($cfg.DHCPLeaseExpires) { $cfg.DHCPLeaseExpires.ToString('yyyy-MM-dd HH:mm') } else { "N/A" }
            MAC             = $cfg.MACAddress
            Speed           = $speed
        }
    }
}

function Get-InstalledSoftware {
    $regPaths = @(
        'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*',
        'HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*',
        'HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*'
    )

    $regPaths | ForEach-Object { Get-ItemProperty $_ } |
    Where-Object { $_.DisplayName -and $_.DisplayName.Trim() -ne '' -and -not $_.SystemComponent -and -not $_.ParentKeyName } |
    Sort-Object DisplayName -Unique |
    ForEach-Object {
        @{
            Name        = $_.DisplayName
            Version     = $_.DisplayVersion
            Publisher   = $_.Publisher
            InstallDate = $_.InstallDate
            Size        = if ($_.EstimatedSize) { "$([math]::Round($_.EstimatedSize / 1024, 1)) MB" } else { "" }
            Location    = $_.InstallLocation
        }
    }
}

function Get-StartupApps {
    Get-CimInstance Win32_StartupCommand | ForEach-Object {
        @{
            Name     = $_.Name
            Command  = $_.Command
            Location = $_.Location
            User     = $_.User
        }
    }
}

function Get-TopProcesses {
    param([int]$Top = 30)

    Get-Process | Sort-Object WorkingSet64 -Descending | Select-Object -First $Top | ForEach-Object {
        $owner = try { $_.GetOwner().User } catch { "N/A" }
        $cmdLine = try {
            (Get-CimInstance Win32_Process -Filter "ProcessId=$($_.Id)").CommandLine
        } catch { "" }

        $cpuTime = try { $_.TotalProcessorTime.TotalSeconds } catch { 0 }

        # Calculate start time BEFORE the hashtable
        $startTime = try { $_.StartTime.ToString('yyyy-MM-dd HH:mm:ss') } catch { "N/A" }
        $procPath = try { $_.Path } catch { "" }

        @{
            Name      = $_.ProcessName
            PID       = $_.Id
            Owner     = $owner
            CPUS      = [math]::Round($cpuTime, 2)
            MemMB     = [math]::Round($_.WorkingSet64 / 1MB, 2)
            PrivateMB = [math]::Round($_.PrivateMemorySize64 / 1MB, 2)
            Handles   = $_.HandleCount
            Threads   = $_.Threads.Count
            StartTime = $startTime
            Path      = $procPath
            CmdLine   = $cmdLine
        }
    }
}

function Get-ServicesList {
    Get-CimInstance Win32_Service | Sort-Object @{Expression = { $_.State -eq 'Running' }; Descending = $true }, DisplayName | ForEach-Object {
        @{
            Name        = $_.Name
            DisplayName = $_.DisplayName
            State       = $_.State
            StartMode   = $_.StartMode
            Account     = $_.StartName
            PathName    = $_.PathName
            Description = $_.Description
        }
    }
}

function Get-ScheduledTasksSummary {
    Get-ScheduledTask |
        Where-Object { $_.State -ne 'Disabled' -and $_.TaskPath -notlike '\Microsoft\*' } |
        Sort-Object TaskName | ForEach-Object {
            $info = $_ | Get-ScheduledTaskInfo -ErrorAction SilentlyContinue

            $lastRun = if ($info.LastRunTime -and $info.LastRunTime.Year -gt 1999) {
                $info.LastRunTime.ToString('yyyy-MM-dd HH:mm')
            } else { "Never" }

            $nextRun = if ($info.NextRunTime -and $info.NextRunTime.Year -gt 1999) {
                $info.NextRunTime.ToString('yyyy-MM-dd HH:mm')
            } else { "N/A" }

            @{
                Name    = $_.TaskName
                Path    = $_.TaskPath
                State   = $_.State.ToString()
                LastRun = $lastRun
                NextRun = $nextRun
                Result  = $info.LastTaskResult
                Author  = $_.Author
            }
        }
}

function Get-EventLogSummary {
    param([int]$Hours = 24, [int]$MaxPerLog = 15)

    $cutoff = (Get-Date).AddHours(-$Hours)
    $logs = @('System', 'Application')
    $result = @{}

    foreach ($log in $logs) {
        $events = Get-WinEvent -FilterHashtable @{
            LogName   = $log
            Level     = @(1, 2, 3)  # Critical, Error, Warning
            StartTime = $cutoff
        } -MaxEvents ($MaxPerLog * 3) -ErrorAction SilentlyContinue |
        Select-Object -First $MaxPerLog |
        ForEach-Object {
            $levelName = switch ($_.Level) { 1 { "Critical" } 2 { "Error" } 3 { "Warning" } }
            @{
                Time    = $_.TimeCreated.ToString('yyyy-MM-dd HH:mm:ss')
                Level   = $levelName
                Source  = $_.ProviderName
                EventID = $_.Id
                Message = ($_.Message -split "`n")[0].Substring(0, [Math]::Min(200, ($_.Message -split "`n")[0].Length))
            }
        }
        $result[$log] = $events
    }
    $result
}

function Get-UsersAndGroups {
    $localUsers = Get-LocalUser | ForEach-Object {
        @{
            Name            = $_.Name
            Enabled         = $_.Enabled
            LastLogon       = if ($_.LastLogon) { $_.LastLogon.ToString('yyyy-MM-dd HH:mm') } else { "Never" }
            PasswordExpires = if ($_.PasswordExpires) { $_.PasswordExpires.ToString('yyyy-MM-dd') } else { "Never" }
            Description     = $_.Description
        }
    }

    $localGroups = Get-LocalGroup | ForEach-Object {
        $members = try {
            (Get-LocalGroupMember $_.Name | ForEach-Object { $_.Name }) -join ', '
        }
        catch { "Access Denied" }
        @{
            GroupName   = $_.Name
            Description = $_.Description
            Members     = $members
        }
    }

    @{ Users = $localUsers; Groups = $localGroups }
}

function Get-PowerConfig {
    $plan = try {
        $activePlan = powercfg /getactivescheme 2>$null
        if ($activePlan -match 'Power Scheme GUID:\s+\S+\s+\((.+)\)') { $Matches[1] } else { "Unknown" }
    }
    catch { "Unknown" }

    $battery = Get-CimInstance Win32_Battery | ForEach-Object {
        @{
            Name               = $_.Name
            Status             = $_.Status
            ChargePct          = $_.EstimatedChargeRemaining
            RunTimeMins        = $_.EstimatedRunTime
            DesignCapacity     = $_.DesignCapacity
            FullChargeCapacity = $_.FullChargeCapacity
            Chemistry          = switch ($_.Chemistry) { 1 { "Other" } 2 { "Unknown" } 3 { "Lead Acid" } 4 { "Nickel Cadmium" } 5 { "Nickel Metal Hydride" } 6 { "Lithium-ion" } default { "Other" } }
        }
    }

    @{ ActivePlan = $plan; Battery = $battery }
}

function Get-EnvironmentVars {
    [System.Environment]::GetEnvironmentVariables('Machine').GetEnumerator() |
    Sort-Object Name | ForEach-Object {
        @{ Name = $_.Name; Value = $_.Value; Scope = "Machine" }
    }
}

function Get-HotfixList {
    Get-HotFix | Sort-Object InstalledOn -Descending | Select-Object -First 25 | ForEach-Object {
        @{
            HotFixID    = $_.HotFixID
            Description = $_.Description
            InstalledBy = $_.InstalledBy
            InstalledOn = if ($_.InstalledOn) { $_.InstalledOn.ToString('yyyy-MM-dd') } else { "Unknown" }
        }
    }
}

function Get-USBDevices {
    # Method 1: PnP entities (most reliable)
    $devices = try {
        Get-CimInstance Win32_PnPEntity | Where-Object {
            $_.PNPDeviceID -match '^USB\\' -and
            $_.Name -and
            $_.Name -notmatch 'Root Hub|Generic Hub|USB Composite Device'
        } | Sort-Object Name -Unique | ForEach-Object {
            $statusText = if ($_.Status -eq 'OK') { 'OK' }
                          elseif ($_.Status) { $_.Status }
                          else { 'Unknown' }
            @{
                Name         = $_.Name
                Manufacturer = $_.Manufacturer
                DeviceID     = $_.PNPDeviceID
                Status       = $statusText
                Class        = $_.PNPClass
            }
        }
    } catch { @() }

    # Method 2: Fallback to USBControllerDevice if Method 1 finds nothing
    if (-not $devices -or $devices.Count -eq 0) {
        $devices = try {
            Get-CimInstance Win32_USBControllerDevice | ForEach-Object {
                $depPath = $_.Dependent.ToString()
                $dep = Get-CimInstance -Query "SELECT * FROM Win32_PnPEntity WHERE DeviceID='$($depPath -replace '\\','\\')'" -ErrorAction SilentlyContinue
                if ($dep -and $dep.Name -notmatch 'Root Hub|Generic Hub|USB Composite Device|Input Device') {
                    $statusText = if ($dep.Status -eq 'OK') { 'OK' }
                                  elseif ($dep.Status) { $dep.Status }
                                  else { 'Unknown' }
                    @{
                        Name         = $dep.Name
                        Manufacturer = $dep.Manufacturer
                        DeviceID     = $dep.PNPDeviceID
                        Status       = $statusText
                        Class        = $dep.PNPClass
                    }
                }
            } | Where-Object { $_ }
        } catch { @() }
    }

    $devices
}

function Get-AudioDevices {
    Get-CimInstance Win32_SoundDevice | ForEach-Object {
        @{
            Name         = $_.Name
            Manufacturer = $_.Manufacturer
            Status       = $_.Status
            DeviceID     = $_.DeviceID
        }
    }
}

# ── Collect All Data ───────────────────────────────────────────────────────────

Write-Host "Collecting system information..." -ForegroundColor Cyan

Write-Host "  [1/16] System overview..."
$sysOverview = Get-SystemOverview

Write-Host "  [2/16] Security information..."
$security = Get-SecurityInfo

Write-Host "  [3/16] CPU information..."
$cpuInfo = @(Get-CPUInfo)

Write-Host "  [4/16] Memory information..."
$memInfo = Get-MemoryInfo

Write-Host "  [5/16] GPU information..."
$gpuInfo = @(Get-GPUInfo)

Write-Host "  [6/16] Monitor information..."
$monitorInfo = @(Get-MonitorInfo)

Write-Host "  [7/16] Physical disks..."
$diskPhysical = @(Get-DiskPhysical)

Write-Host "  [8/16] Logical volumes..."
$diskLogical = @(Get-DiskLogical)

Write-Host "  [9/16] Network adapters..."
$netAdapters = @(Get-NetworkAdapters)

Write-Host "  [10/16] Installed software..."
$software = @(Get-InstalledSoftware)

Write-Host "  [11/16] Startup applications..."
$startup = @(Get-StartupApps)

Write-Host "  [12/16] Top processes..."
$processes = @(Get-TopProcesses -Top 30)

Write-Host "  [13/16] Services..."
$services = @(Get-ServicesList)

Write-Host "  [14/16] Event log summary..."
$eventLogs = Get-EventLogSummary -Hours 24

Write-Host "  [15/16] Users and groups..."
$usersGroups = Get-UsersAndGroups

Write-Host "  [16/16] Additional info..."
$power = Get-PowerConfig
$hotfixes = @(Get-HotfixList)
$usbDevs = @(Get-USBDevices)
$audio = @(Get-AudioDevices)
$schedTasks = @(Get-ScheduledTasksSummary)

# ── HTML Helper Functions ──────────────────────────────────────────────────────

function HtmlEncode([string]$text) {
    if (-not $text) { return '' }
    [System.Web.HttpUtility]::HtmlEncode($text)
}
Add-Type -AssemblyName System.Web

function MakeProgressBar([double]$pct) {
    $color = if ($pct -gt 90) { 'var(--danger)' }
    elseif ($pct -gt 75) { 'var(--warning)' }
    else { 'var(--primary)' }
    "<div class='progress-bar'><div class='progress-fill' style='width:${pct}%;background:${color}'></div></div>"
}

function MakeStatusBadge([string]$status) {
    switch -Wildcard ($status.ToLower()) {
        'running' { "<span class='badge badge-success'>Running</span>" }
        'connected' { "<span class='badge badge-success'>Connected</span>" }
        'enabled' { "<span class='badge badge-success'>Enabled</span>" }
        'ok' { "<span class='badge badge-success'>OK</span>" }
        'healthy' { "<span class='badge badge-success'>Healthy</span>" }
        'ready' { "<span class='badge badge-success'>Ready</span>" }
        'stopped' { "<span class='badge badge-danger'>Stopped</span>" }
        'disconnected' { "<span class='badge badge-danger'>Disconnected</span>" }
        'disabled' { "<span class='badge badge-muted'>Disabled</span>" }
        'warning' { "<span class='badge badge-warning'>Warning</span>" }
        'error' { "<span class='badge badge-danger'>Error</span>" }
        'critical' { "<span class='badge badge-danger'>Critical</span>" }
        default { "<span class='badge'>$status</span>" }
    }
}

function MakeSection([string]$id, [string]$icon, [string]$title, [string]$content, [int]$count = -1) {
    $countBadge = if ($count -ge 0) { "<span class='section-count'>$count</span>" } else { '' }
    @"
<section class="card" id="section-$id">
    <div class="card-header" onclick="toggleSection('$id')">
        <h2><span class="section-icon">$icon</span> $title $countBadge</h2>
        <span class="collapse-indicator" id="indicator-$id">▼</span>
    </div>
    <div class="section-content" id="content-$id">
        $content
    </div>
</section>
"@
}

# ── Build HTML Sections ────────────────────────────────────────────────────────

# -- Overview Cards --
$overviewHtml = @"
<div class="info-grid">
    <div class="info-card">
        <h4>💻 Computer</h4>
        <div class="info-row"><span class="info-label">Name</span><span class="info-value">$($sysOverview.ComputerName)</span></div>
        <div class="info-row"><span class="info-label">Domain</span><span class="info-value">$(HtmlEncode $sysOverview.Domain)</span></div>
        <div class="info-row"><span class="info-label">Manufacturer</span><span class="info-value">$(HtmlEncode $sysOverview.Manufacturer)</span></div>
        <div class="info-row"><span class="info-label">Model</span><span class="info-value">$(HtmlEncode $sysOverview.Model)</span></div>
        <div class="info-row"><span class="info-label">Motherboard</span><span class="info-value">$(HtmlEncode $sysOverview.Motherboard)</span></div>
        <div class="info-row"><span class="info-label">System Type</span><span class="info-value">$($sysOverview.SystemType)</span></div>
        <div class="info-row"><span class="info-label">Serial Number</span><span class="info-value">$(HtmlEncode $sysOverview.SerialNumber)</span></div>
    </div>
    <div class="info-card">
        <h4>🖥️ Operating System</h4>
        <div class="info-row"><span class="info-label">OS</span><span class="info-value">$(HtmlEncode $sysOverview.OSName)</span></div>
        <div class="info-row"><span class="info-label">Version</span><span class="info-value">$($sysOverview.WinVersion) (Build $($sysOverview.OSBuild))</span></div>
        <div class="info-row"><span class="info-label">Architecture</span><span class="info-value">$($sysOverview.OSArchitecture)</span></div>
        <div class="info-row"><span class="info-label">Install Date</span><span class="info-value">$($sysOverview.InstallDate)</span></div>
        <div class="info-row"><span class="info-label">Last Boot</span><span class="info-value">$($sysOverview.LastBoot)</span></div>
        <div class="info-row"><span class="info-label">Uptime</span><span class="info-value">$($sysOverview.Uptime)</span></div>
        <div class="info-row"><span class="info-label">Time Zone</span><span class="info-value">$(HtmlEncode $sysOverview.TimeZone)</span></div>
    </div>
    <div class="info-card">
        <h4>📊 Memory Overview</h4>
        <div class="info-row"><span class="info-label">Total Physical</span><span class="info-value">$($sysOverview.TotalPhysMemGB) GB</span></div>
        <div class="info-row"><span class="info-label">Available</span><span class="info-value">$($sysOverview.FreePhysMemGB) GB</span></div>
        <div class="info-row"><span class="info-label">Usage</span><span class="info-value">$($sysOverview.MemoryUsagePct)%</span></div>
        $(MakeProgressBar $sysOverview.MemoryUsagePct)
    </div>
    <div class="info-card">
        <h4>🔐 Security</h4>
        <div class="info-row"><span class="info-label">Secure Boot</span><span class="info-value">$($security.SecureBoot)</span></div>
        <div class="info-row"><span class="info-label">TPM Present</span><span class="info-value">$($security.TPM.Present)</span></div>
        <div class="info-row"><span class="info-label">TPM Version</span><span class="info-value">$($security.TPM.Version)</span></div>
        <div class="info-row"><span class="info-label">UAC Enabled</span><span class="info-value">$($security.UAC.Enabled)</span></div>
        <div class="info-row"><span class="info-label">RDP Enabled</span><span class="info-value">$($security.RDPEnabled)</span></div>
        <div class="info-row"><span class="info-label">Antivirus</span><span class="info-value">$(($security.Antivirus | ForEach-Object { HtmlEncode $_.displayName }) -join ', ')</span></div>
    </div>
    <div class="info-card">
        <h4>🔋 BIOS / Firmware</h4>
        <div class="info-row"><span class="info-label">Vendor</span><span class="info-value">$(HtmlEncode $sysOverview.BIOSVendor)</span></div>
        <div class="info-row"><span class="info-label">Version</span><span class="info-value">$(HtmlEncode $sysOverview.BIOSVersion)</span></div>
        <div class="info-row"><span class="info-label">Date</span><span class="info-value">$($sysOverview.BIOSDate)</span></div>
    </div>
    <div class="info-card">
        <h4>⚡ Power</h4>
        <div class="info-row"><span class="info-label">Active Plan</span><span class="info-value">$(HtmlEncode $power.ActivePlan)</span></div>
        $(if ($power.Battery) {
            $power.Battery | ForEach-Object {
                "<div class='info-row'><span class='info-label'>Battery</span><span class='info-value'>$($_.Name) - $($_.ChargePct)%</span></div>"
            }
        } else {
            "<div class='info-row'><span class='info-label'>Battery</span><span class='info-value'>Not present (Desktop)</span></div>"
        })
    </div>
</div>
"@

# -- CPU Section --
$cpuHtml = ($cpuInfo | ForEach-Object {
        @"
<div class="detail-block">
    <div class="info-grid cols-3">
        <div class="info-row"><span class="info-label">Name</span><span class="info-value"><strong>$(HtmlEncode $_.Name)</strong></span></div>
        <div class="info-row"><span class="info-label">Socket</span><span class="info-value">$($_.Socket)</span></div>
        <div class="info-row"><span class="info-label">Architecture</span><span class="info-value">$($_.Architecture)</span></div>
        <div class="info-row"><span class="info-label">Cores</span><span class="info-value">$($_.Cores)</span></div>
        <div class="info-row"><span class="info-label">Logical Processors</span><span class="info-value">$($_.LogicalProcessors)</span></div>
        <div class="info-row"><span class="info-label">Max Clock</span><span class="info-value">$($_.MaxClockMHz) MHz</span></div>
        <div class="info-row"><span class="info-label">Current Clock</span><span class="info-value">$($_.CurrentClockMHz) MHz</span></div>
        <div class="info-row"><span class="info-label">L2 Cache</span><span class="info-value">$([math]::Round($_.L2CacheKB / 1024, 2)) MB</span></div>
        <div class="info-row"><span class="info-label">L3 Cache</span><span class="info-value">$([math]::Round($_.L3CacheKB / 1024, 2)) MB</span></div>
        <div class="info-row"><span class="info-label">Virtualization</span><span class="info-value">$($_.VTEnabled)</span></div>
        <div class="info-row"><span class="info-label">Load</span><span class="info-value">$(if ($_.LoadPct) { "$($_.LoadPct)%" } else { "N/A" })</span></div>
        <div class="info-row"><span class="info-label">Status</span><span class="info-value">$(MakeStatusBadge $_.Status)</span></div>
    </div>
</div>
"@
    }) -join "`n"

# -- Memory Section --
$memTableRows = ($memInfo.Slots | ForEach-Object {
        "<tr><td>$($_.Slot)</td><td>$($_.BankLabel)</td><td>$($_.SizeGB) GB</td><td>$($_.SpeedMHz) MHz</td><td>$($_.Type)</td><td>$($_.FormFactor)</td><td>$(HtmlEncode $_.Manufacturer)</td><td>$(HtmlEncode $_.PartNumber)</td><td>$($_.SerialNumber)</td></tr>"
    }) -join "`n"

$memHtml = @"
<div class="summary-bar">
    <span>Total Installed: <strong>$(($memInfo.Slots | Measure-Object -Property SizeGB -Sum).Sum) GB</strong></span>
    <span>Slots Used: <strong>$($memInfo.UsedSlots) / $($memInfo.TotalSlots)</strong></span>
    <span>Max Capacity: <strong>$($memInfo.MaxCapGB) GB</strong></span>
</div>
<div class="table-wrapper">
<table>
    <thead><tr><th>Slot</th><th>Bank</th><th>Size</th><th>Speed</th><th>Type</th><th>Form</th><th>Manufacturer</th><th>Part Number</th><th>Serial</th></tr></thead>
    <tbody>$memTableRows</tbody>
</table>
</div>
"@

# -- GPU Section --
$gpuHtml = ($gpuInfo | ForEach-Object {
        @"
<div class="detail-block">
    <div class="info-grid cols-3">
        <div class="info-row"><span class="info-label">Name</span><span class="info-value"><strong>$(HtmlEncode $_.Name)</strong></span></div>
        <div class="info-row"><span class="info-label">VRAM</span><span class="info-value">$(if ($_.VRAM -ne 'Unknown') { "$($_.VRAM) GB" } else { 'Unknown' })</span></div>
        <div class="info-row"><span class="info-label">Driver</span><span class="info-value">$($_.DriverVersion)</span></div>
        <div class="info-row"><span class="info-label">Driver Date</span><span class="info-value">$($_.DriverDate)</span></div>
        <div class="info-row"><span class="info-label">Resolution</span><span class="info-value">$($_.Resolution)</span></div>
        <div class="info-row"><span class="info-label">Refresh Rate</span><span class="info-value">$($_.RefreshRate)</span></div>
        <div class="info-row"><span class="info-label">DAC Type</span><span class="info-value">$($_.AdapterDACType)</span></div>
        <div class="info-row"><span class="info-label">Status</span><span class="info-value">$(MakeStatusBadge $_.Status)</span></div>
    </div>
</div>
"@
    }) -join "`n"

# -- Monitors --
$monitorHtml = if ($monitorInfo.Count -gt 0) {
    $rows = ($monitorInfo | ForEach-Object {
            "<tr><td>$(HtmlEncode $_.Manufacturer)</td><td>$(HtmlEncode $_.Model)</td><td>$($_.SerialNumber)</td><td>$($_.YearMade)</td><td>Week $($_.WeekMade)</td></tr>"
        }) -join "`n"
    "<div class='table-wrapper'><table><thead><tr><th>Manufacturer</th><th>Model</th><th>Serial</th><th>Year</th><th>Week</th></tr></thead><tbody>$rows</tbody></table></div>"
}
else { "<p class='no-data'>No monitor information available via WMI.</p>" }

# -- Physical Disks --
$diskPhysRows = ($diskPhysical | ForEach-Object {
        "<tr><td>$($_.Index)</td><td><strong>$(HtmlEncode $_.Model)</strong></td><td>$($_.Interface)</td><td>$($_.MediaType)</td><td>$($_.SizeGB) GB</td><td>$($_.Partitions)</td><td>$(MakeStatusBadge $_.Status)</td><td>$($_.SerialNo)</td><td>$($_.FWVersion)</td></tr>"
    }) -join "`n"

$diskPhysHtml = @"
<div class="table-wrapper">
<table>
    <thead><tr><th>#</th><th>Model</th><th>Interface</th><th>Media</th><th>Size</th><th>Parts</th><th>Status</th><th>Serial</th><th>Firmware</th></tr></thead>
    <tbody>$diskPhysRows</tbody>
</table>
</div>
"@

# -- Logical Volumes --
$diskLogRows = ($diskLogical | ForEach-Object {
        $barColor = if ($_.UsagePct -gt 90) { 'var(--danger)' } elseif ($_.UsagePct -gt 75) { 'var(--warning)' } else { 'var(--primary)' }
        "<tr><td><strong>$($_.Drive)</strong></td><td>$(HtmlEncode $_.Label)</td><td>$($_.FileSystem)</td><td>$($_.TotalGB) GB</td><td>$($_.FreeGB) GB</td><td>$($_.UsedGB) GB</td><td>$($_.UsagePct)%<div class='progress-bar'><div class='progress-fill' style='width:$($_.UsagePct)%;background:$barColor'></div></div></td></tr>"
    }) -join "`n"

$diskLogHtml = @"
<div class="table-wrapper">
<table>
    <thead><tr><th>Drive</th><th>Label</th><th>FS</th><th>Total</th><th>Free</th><th>Used</th><th>Usage</th></tr></thead>
    <tbody>$diskLogRows</tbody>
</table>
</div>
"@

# -- Network --
$netHtml = ($netAdapters | ForEach-Object {
        @"
<div class="detail-block">
    <div class="info-grid cols-3">
        <div class="info-row"><span class="info-label">Adapter</span><span class="info-value"><strong>$(HtmlEncode $_.Name)</strong></span></div>
        <div class="info-row"><span class="info-label">Connection</span><span class="info-value">$(HtmlEncode $_.NetConnectionID)</span></div>
        <div class="info-row"><span class="info-label">Status</span><span class="info-value">$(MakeStatusBadge $_.Status)</span></div>
        <div class="info-row"><span class="info-label">IPv4</span><span class="info-value">$($_.IPv4)</span></div>
        <div class="info-row"><span class="info-label">IPv6</span><span class="info-value monospace">$($_.IPv6)</span></div>
        <div class="info-row"><span class="info-label">Subnet</span><span class="info-value">$($_.Subnet)</span></div>
        <div class="info-row"><span class="info-label">Gateway</span><span class="info-value">$($_.Gateway)</span></div>
        <div class="info-row"><span class="info-label">DNS</span><span class="info-value">$($_.DNS)</span></div>
        <div class="info-row"><span class="info-label">DHCP</span><span class="info-value">$(if ($_.DHCP) { "Yes (Server: $($_.DHCPServer))" } else { "No (Static)" })</span></div>
        <div class="info-row"><span class="info-label">MAC</span><span class="info-value monospace">$($_.MAC)</span></div>
        <div class="info-row"><span class="info-label">Speed</span><span class="info-value">$($_.Speed)</span></div>
        <div class="info-row"><span class="info-label">Type</span><span class="info-value">$($_.AdapterType)</span></div>
    </div>
</div>
"@
    }) -join "`n"

# -- Firewall --
$fwHtml = if ($security.Firewall.Count -gt 0) {
    $fwRows = ($security.Firewall | ForEach-Object {
            $st = if ($_.Enabled) { "Enabled" } else { "Disabled" }
            "<tr><td>$($_.Name)</td><td>$(MakeStatusBadge $st)</td><td>$($_.DefaultInboundAction)</td><td>$($_.DefaultOutboundAction)</td></tr>"
        }) -join "`n"
    "<div class='table-wrapper'><table><thead><tr><th>Profile</th><th>Status</th><th>Inbound Default</th><th>Outbound Default</th></tr></thead><tbody>$fwRows</tbody></table></div>"
}
else { "<p class='no-data'>Firewall information unavailable.</p>" }

# -- Users & Groups --
$userRows = ($usersGroups.Users | ForEach-Object {
        $en = if ($_.Enabled) { "Enabled" } else { "Disabled" }
        "<tr><td>$(HtmlEncode $_.Name)</td><td>$(MakeStatusBadge $en)</td><td>$($_.LastLogon)</td><td>$($_.PasswordExpires)</td><td>$(HtmlEncode $_.Description)</td></tr>"
    }) -join "`n"

$groupRows = ($usersGroups.Groups | ForEach-Object {
        "<tr><td><strong>$(HtmlEncode $_.GroupName)</strong></td><td>$(HtmlEncode $_.Description)</td><td class='wrap-cell'>$(HtmlEncode $_.Members)</td></tr>"
    }) -join "`n"

$usersHtml = @"
<h3>Local Users</h3>
<div class="table-wrapper">
<table><thead><tr><th>Username</th><th>Status</th><th>Last Logon</th><th>Password Expires</th><th>Description</th></tr></thead>
<tbody>$userRows</tbody></table>
</div>
<h3>Local Groups</h3>
<div class="table-wrapper">
<table><thead><tr><th>Group</th><th>Description</th><th>Members</th></tr></thead>
<tbody>$groupRows</tbody></table>
</div>
"@

# -- Installed Software --
$swRows = ($software | ForEach-Object {
        "<tr><td>$(HtmlEncode $_.Name)</td><td>$(HtmlEncode $_.Version)</td><td>$(HtmlEncode $_.Publisher)</td><td>$($_.InstallDate)</td><td>$($_.Size)</td></tr>"
    }) -join "`n"

$swHtml = @"
<div class="table-controls">
    <input type="text" id="softwareFilter" class="filter-input" placeholder="🔍 Filter software..." onkeyup="filterTable('softwareFilter', 'softwareTable')">
</div>
<div class="table-wrapper">
<table id="softwareTable">
    <thead><tr><th>Name</th><th>Version</th><th>Publisher</th><th>Install Date</th><th>Size</th></tr></thead>
    <tbody>$swRows</tbody>
</table>
</div>
"@

# -- Startup Apps --
$startupRows = ($startup | ForEach-Object {
        "<tr><td>$(HtmlEncode $_.Name)</td><td class='monospace wrap-cell'>$(HtmlEncode $_.Command)</td><td class='monospace wrap-cell'>$(HtmlEncode $_.Location)</td><td>$(HtmlEncode $_.User)</td></tr>"
    }) -join "`n"

$startupHtml = @"
<div class="table-wrapper">
<table><thead><tr><th>Name</th><th>Command</th><th>Location</th><th>User</th></tr></thead>
<tbody>$startupRows</tbody></table>
</div>
"@

# -- Processes --
$procRows = ($processes | ForEach-Object {
        $cmdEncoded = HtmlEncode $_.CmdLine
        $pathEncoded = HtmlEncode $_.Path
        $rowId = "proc-$($_.PID)"
        @"
<tr class="expandable" onclick="toggleRow('$rowId')">
    <td><span class="expand-icon">▶</span> <strong>$($_.Name)</strong></td>
    <td>$($_.PID)</td>
    <td>$(HtmlEncode $_.Owner)</td>
    <td>$($_.CPUS)s</td>
    <td>$($_.MemMB)</td>
    <td>$($_.PrivateMB)</td>
    <td>$($_.Threads)</td>
    <td>$($_.Handles)</td>
    <td class="monospace">$($_.StartTime)</td>
</tr>
<tr class="details-row" id="$rowId">
    <td colspan="9">
        <div class="details-content">
            <div><strong>Path:</strong> <span class="monospace">$pathEncoded</span></div>
            <div><strong>Command Line:</strong> <span class="monospace">$cmdEncoded</span></div>
        </div>
    </td>
</tr>
"@
    }) -join "`n"

$procHtml = @"
<div class="table-controls">
    <input type="text" id="processFilter" class="filter-input" placeholder="🔍 Filter processes..." onkeyup="filterTable('processFilter', 'processTable')">
</div>
<div class="table-wrapper">
<table id="processTable">
    <thead><tr><th>Process</th><th>PID</th><th>Owner</th><th>CPU Time</th><th>Working (MB)</th><th>Private (MB)</th><th>Threads</th><th>Handles</th><th>Started</th></tr></thead>
    <tbody>$procRows</tbody>
</table>
</div>
"@

# -- Services --
$svcRunning = ($services | Where-Object { $_.State -eq 'Running' } | Measure-Object).Count
$svcStopped = ($services | Where-Object { $_.State -eq 'Stopped' } | Measure-Object).Count

$svcRows = ($services | ForEach-Object {
        $svcId = "svc-$(HtmlEncode $_.Name)"
        @"
<tr class="expandable" onclick="toggleRow('$svcId')">
    <td><span class="expand-icon">▶</span> $(HtmlEncode $_.Name)</td>
    <td>$(HtmlEncode $_.DisplayName)</td>
    <td>$(MakeStatusBadge $_.State)</td>
    <td>$($_.StartMode)</td>
    <td>$(HtmlEncode $_.Account)</td>
</tr>
<tr class="details-row" id="$svcId">
    <td colspan="5">
        <div class="details-content">
            <div><strong>Path:</strong> <span class="monospace">$(HtmlEncode $_.PathName)</span></div>
            <div><strong>Description:</strong> $(HtmlEncode $_.Description)</div>
        </div>
    </td>
</tr>
"@
    }) -join "`n"

$svcHtml = @"
<div class="summary-bar">
    <span class="badge badge-success">Running: $svcRunning</span>
    <span class="badge badge-danger">Stopped: $svcStopped</span>
</div>
<div class="table-controls">
    <input type="text" id="serviceFilter" class="filter-input" placeholder="🔍 Filter services..." onkeyup="filterTable('serviceFilter', 'serviceTable')">
    <div class="filter-buttons">
        <button class="btn btn-sm active" onclick="filterServiceState(this, 'all')">All</button>
        <button class="btn btn-sm" onclick="filterServiceState(this, 'Running')">Running</button>
        <button class="btn btn-sm" onclick="filterServiceState(this, 'Stopped')">Stopped</button>
    </div>
</div>
<div class="table-wrapper">
<table id="serviceTable">
    <thead><tr><th>Name</th><th>Display Name</th><th>Status</th><th>Start Type</th><th>Account</th></tr></thead>
    <tbody>$svcRows</tbody>
</table>
</div>
"@

# -- Event Logs --
$eventHtml = ""
foreach ($logName in @('System', 'Application')) {
    $events = $eventLogs[$logName]
    if ($events -and $events.Count -gt 0) {
        $evtRows = ($events | ForEach-Object {
                "<tr><td class='monospace'>$($_.Time)</td><td>$(MakeStatusBadge $_.Level)</td><td>$(HtmlEncode $_.Source)</td><td>$($_.EventID)</td><td class='wrap-cell'>$(HtmlEncode $_.Message)</td></tr>"
            }) -join "`n"
        $eventHtml += @"
<h3>$logName Log (Last 24h)</h3>
<div class="table-wrapper">
<table><thead><tr><th>Time</th><th>Level</th><th>Source</th><th>Event ID</th><th>Message</th></tr></thead>
<tbody>$evtRows</tbody></table>
</div>
"@
    }
    else {
        $eventHtml += "<h3>$logName Log</h3><p class='no-data'>No critical/error/warning events in the last 24 hours.</p>"
    }
}

# -- Hotfixes --
$hfRows = ($hotfixes | ForEach-Object {
        "<tr><td><strong>$($_.HotFixID)</strong></td><td>$($_.Description)</td><td>$(HtmlEncode $_.InstalledBy)</td><td>$($_.InstalledOn)</td></tr>"
    }) -join "`n"

$hfHtml = @"
<div class="table-wrapper">
<table><thead><tr><th>KB ID</th><th>Description</th><th>Installed By</th><th>Date</th></tr></thead>
<tbody>$hfRows</tbody></table>
</div>
"@

# -- USB Devices --
$usbHtml = if ($usbDevs -and $usbDevs.Count -gt 0) {
    $usbRows = ($usbDevs | ForEach-Object {
        "<tr><td>$(HtmlEncode $_.Name)</td><td>$(HtmlEncode $_.Manufacturer)</td><td>$(HtmlEncode $_.Class)</td><td>$(MakeStatusBadge $_.Status)</td></tr>"
    }) -join "`n"
    "<div class='table-wrapper'><table><thead><tr><th>Device</th><th>Manufacturer</th><th>Class</th><th>Status</th></tr></thead><tbody>$usbRows</tbody></table></div>"
} else { "<p class='no-data'>No USB devices detected.</p>" }

# -- Audio --
$audioHtml = if ($audio.Count -gt 0) {
    $audioRows = ($audio | ForEach-Object {
            "<tr><td>$(HtmlEncode $_.Name)</td><td>$(HtmlEncode $_.Manufacturer)</td><td>$(MakeStatusBadge $_.Status)</td></tr>"
        }) -join "`n"
    "<div class='table-wrapper'><table><thead><tr><th>Device</th><th>Manufacturer</th><th>Status</th></tr></thead><tbody>$audioRows</tbody></table></div>"
}
else { "<p class='no-data'>No audio devices detected.</p>" }

# -- Scheduled Tasks --
$taskHtml = if ($schedTasks.Count -gt 0) {
    $taskRows = ($schedTasks | ForEach-Object {
            $resultClass = if ($_.Result -eq 0) { 'badge-success' } elseif ($_.Result) { 'badge-warning' } else { '' }
            $resultText = if ($_.Result -eq 0) { "Success" } elseif ($_.Result) { "0x$("{0:X}" -f $_.Result)" } else { "N/A" }
            "<tr><td>$(HtmlEncode $_.Name)</td><td class='monospace'>$(HtmlEncode $_.Path)</td><td>$(MakeStatusBadge $_.State)</td><td>$($_.LastRun)</td><td>$($_.NextRun)</td><td><span class='badge $resultClass'>$resultText</span></td></tr>"
        }) -join "`n"
    @"
<div class="table-controls">
    <input type="text" id="taskFilter" class="filter-input" placeholder="🔍 Filter tasks..." onkeyup="filterTable('taskFilter', 'taskTable')">
</div>
<div class="table-wrapper">
<table id="taskTable"><thead><tr><th>Name</th><th>Path</th><th>State</th><th>Last Run</th><th>Next Run</th><th>Result</th></tr></thead>
<tbody>$taskRows</tbody></table>
</div>
"@
}
else { "<p class='no-data'>No custom scheduled tasks found.</p>" }

# -- BitLocker --
$bitlockerHtml = if ($security.BitLocker -and $security.BitLocker.Count -gt 0) {
    $blRows = ($security.BitLocker | ForEach-Object {
            "<tr><td>$($_.MountPoint)</td><td>$($_.VolumeStatus)</td><td>$($_.EncryptionMethod)</td><td>$($_.ProtectionStatus)</td><td>$($_.LockStatus)</td></tr>"
        }) -join "`n"
    "<div class='table-wrapper'><table><thead><tr><th>Volume</th><th>Status</th><th>Method</th><th>Protection</th><th>Lock</th></tr></thead><tbody>$blRows</tbody></table></div>"
}
else { "<p class='no-data'>BitLocker not configured or no encrypted volumes.</p>" }

# -- Pending Updates --
$updatesHtml = if ($security.PendingUpdates -and $security.PendingUpdates.Count -gt 0) {
    $updRows = ($security.PendingUpdates | ForEach-Object {
            "<tr><td>$(HtmlEncode $_.Title)</td><td>$($_.Severity)</td><td>$($_.KB)</td></tr>"
        }) -join "`n"
    "<div class='table-wrapper'><table><thead><tr><th>Update</th><th>Severity</th><th>KB</th></tr></thead><tbody>$updRows</tbody></table></div>"
}
else { "<p class='no-data'>No pending updates found.</p>" }

# ── Assemble Final HTML ───────────────────────────────────────────────────────

$html = @"
<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>$ReportTitle - $ComputerName</title>
    <style>
        /* ── CSS Variables ── */
        :root[data-theme="dark"] {
            --primary: #667eea;
            --primary-hover: #7b93ff;
            --secondary: #764ba2;
            --bg: #0f0f0f;
            --surface: #1a1a2e;
            --surface-alt: #16213e;
            --surface-hover: #1f2b47;
            --text: #e4e6eb;
            --text-muted: #8b8fa3;
            --border: #2a2d3e;
            --border-focus: #667eea;
            --success: #00c853;
            --danger: #ff3d3d;
            --warning: #ffab00;
            --shadow: rgba(0, 0, 0, 0.4);
            --code-bg: #0d1117;
        }
        :root[data-theme="light"] {
            --primary: #5c6bc0;
            --primary-hover: #3f51b5;
            --secondary: #7c4dff;
            --bg: #f0f2f5;
            --surface: #ffffff;
            --surface-alt: #f8f9fa;
            --surface-hover: #eef0f4;
            --text: #1a1a2e;
            --text-muted: #6c757d;
            --border: #dee2e6;
            --border-focus: #5c6bc0;
            --success: #2e7d32;
            --danger: #c62828;
            --warning: #f57f17;
            --shadow: rgba(0, 0, 0, 0.08);
            --code-bg: #f6f8fa;
        }

        /* ── Reset & Base ── */
        *, *::before, *::after { box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', sans-serif;
            background: var(--bg);
            color: var(--text);
            margin: 0;
            padding: 0;
            line-height: 1.6;
            transition: background 0.3s, color 0.3s;
            -webkit-font-smoothing: antialiased;
        }

        /* ── Header ── */
        .header {
            background: linear-gradient(135deg, var(--primary) 0%, var(--secondary) 100%);
            color: #fff;
            padding: 48px 24px;
            text-align: center;
            position: relative;
            overflow: hidden;
        }
        .header::before {
            content: '';
            position: absolute;
            top: -50%;
            left: -50%;
            width: 200%;
            height: 200%;
            background: radial-gradient(circle, rgba(255,255,255,0.05) 0%, transparent 70%);
            animation: headerPulse 15s ease-in-out infinite;
        }
        @keyframes headerPulse {
            0%, 100% { transform: translate(0, 0) scale(1); }
            50% { transform: translate(-5%, 5%) scale(1.1); }
        }
        .header h1 {
            margin: 0;
            font-size: 2.4rem;
            font-weight: 700;
            letter-spacing: -0.5px;
            position: relative;
        }
        .header .subtitle {
            opacity: 0.85;
            margin-top: 8px;
            font-size: 1.05rem;
            position: relative;
        }
        .header .meta {
            display: flex;
            justify-content: center;
            gap: 24px;
            margin-top: 16px;
            flex-wrap: wrap;
            position: relative;
        }
        .header .meta span {
            background: rgba(255,255,255,0.15);
            padding: 6px 16px;
            border-radius: 20px;
            font-size: 0.85rem;
            backdrop-filter: blur(4px);
        }

        /* ── Navigation ── */
        .nav-bar {
            background: var(--surface);
            border-bottom: 1px solid var(--border);
            padding: 0 24px;
            position: sticky;
            top: 0;
            z-index: 100;
            overflow-x: auto;
            white-space: nowrap;
            box-shadow: 0 2px 8px var(--shadow);
        }
        .nav-bar a {
            display: inline-block;
            padding: 14px 16px;
            color: var(--text-muted);
            text-decoration: none;
            font-size: 0.85rem;
            font-weight: 500;
            border-bottom: 2px solid transparent;
            transition: all 0.2s;
        }
        .nav-bar a:hover {
            color: var(--primary);
            border-bottom-color: var(--primary);
            background: var(--surface-hover);
        }

        /* ── Container ── */
        .container { max-width: 1400px; margin: 0 auto; padding: 24px; }

        /* ── Cards ── */
        .card {
            background: var(--surface);
            border-radius: 12px;
            margin-bottom: 20px;
            border: 1px solid var(--border);
            box-shadow: 0 2px 12px var(--shadow);
            overflow: hidden;
            transition: box-shadow 0.2s;
        }
        .card:hover { box-shadow: 0 4px 20px var(--shadow); }
        .card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px 24px;
            cursor: pointer;
            user-select: none;
            border-bottom: 1px solid var(--border);
            background: var(--surface-alt);
            transition: background 0.2s;
        }
        .card-header:hover { background: var(--surface-hover); }
        .card-header h2 {
            margin: 0;
            font-size: 1.15rem;
            font-weight: 600;
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .section-icon { font-size: 1.2rem; }
        .section-count {
            background: var(--primary);
            color: #fff;
            padding: 2px 10px;
            border-radius: 12px;
            font-size: 0.75rem;
            font-weight: 600;
        }
        .collapse-indicator {
            font-size: 0.8rem;
            color: var(--text-muted);
            transition: transform 0.3s;
        }
        .collapsed .collapse-indicator { transform: rotate(-90deg); }
        .section-content {
            padding: 20px 24px;
            overflow: hidden;
            transition: max-height 0.4s ease-out, padding 0.3s;
        }
        .card.collapsed .section-content {
            max-height: 0 !important;
            padding-top: 0;
            padding-bottom: 0;
        }

        /* ── Info Grid ── */
        .info-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
            gap: 20px;
            margin-bottom: 20px;
        }
        .info-grid.cols-3 {
            grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
        }
        .info-card {
            background: var(--surface);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 20px;
            border-left: 4px solid var(--primary);
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .info-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px var(--shadow);
        }
        .info-card h4 {
            margin: 0 0 14px 0;
            font-size: 1rem;
            color: var(--primary);
        }
        .info-row {
            display: flex;
            justify-content: space-between;
            padding: 5px 0;
            font-size: 0.88rem;
            border-bottom: 1px solid var(--border);
        }
        .info-row:last-child { border-bottom: none; }
        .info-label { color: var(--text-muted); font-weight: 500; }
        .info-value { text-align: right; max-width: 60%; word-break: break-word; }

        /* ── Tables ── */
        .table-wrapper { overflow-x: auto; }
        table {
            width: 100%;
            border-collapse: collapse;
            font-size: 0.85rem;
        }
        thead th {
            text-align: left;
            padding: 12px 14px;
            background: var(--surface-alt);
            color: var(--text);
            font-weight: 600;
            border-bottom: 2px solid var(--border);
            position: sticky;
            top: 0;
            white-space: nowrap;
        }
        tbody td {
            padding: 10px 14px;
            border-bottom: 1px solid var(--border);
            vertical-align: top;
        }
        tbody tr:hover { background: var(--surface-hover); }
        tbody tr.expandable { cursor: pointer; }
        tbody tr.expandable:hover td:first-child { color: var(--primary); }
        .expand-icon {
            display: inline-block;
            transition: transform 0.2s;
            font-size: 0.7rem;
            margin-right: 4px;
        }
        .details-row { display: none; }
        .details-row.show { display: table-row; }
        .details-row.show .expand-icon { transform: rotate(90deg); }
        .details-content {
            padding: 12px 16px;
            background: var(--code-bg);
            border-radius: 6px;
            font-size: 0.82rem;
            line-height: 1.7;
        }
        .details-content .monospace {
            word-break: break-all;
        }
        .wrap-cell { word-break: break-all; max-width: 400px; }

        /* ── Progress Bars ── */
        .progress-bar {
            height: 6px;
            background: var(--border);
            border-radius: 3px;
            overflow: hidden;
            margin-top: 6px;
        }
        .progress-fill {
            height: 100%;
            background: var(--primary);
            border-radius: 3px;
            transition: width 0.5s ease;
        }

        /* ── Badges ── */
        .badge {
            display: inline-block;
            padding: 3px 10px;
            border-radius: 12px;
            font-size: 0.75rem;
            font-weight: 600;
            letter-spacing: 0.3px;
        }
        .badge-success { background: rgba(0,200,83,0.15); color: var(--success); }
        .badge-danger { background: rgba(255,61,61,0.15); color: var(--danger); }
        .badge-warning { background: rgba(255,171,0,0.15); color: var(--warning); }
        .badge-muted { background: var(--surface-hover); color: var(--text-muted); }

        /* ── Controls ── */
        .table-controls {
            display: flex;
            gap: 12px;
            align-items: center;
            margin-bottom: 14px;
            flex-wrap: wrap;
        }
        .filter-input {
            padding: 8px 16px;
            border: 1px solid var(--border);
            border-radius: 8px;
            background: var(--surface-alt);
            color: var(--text);
            font-size: 0.88rem;
            outline: none;
            min-width: 260px;
            transition: border-color 0.2s;
        }
        .filter-input:focus { border-color: var(--border-focus); }
        .filter-buttons { display: flex; gap: 6px; }
        .btn {
            padding: 6px 14px;
            border: 1px solid var(--border);
            border-radius: 6px;
            background: var(--surface);
            color: var(--text-muted);
            cursor: pointer;
            font-size: 0.8rem;
            transition: all 0.2s;
        }
        .btn:hover { background: var(--surface-hover); color: var(--text); }
        .btn.active { background: var(--primary); color: #fff; border-color: var(--primary); }

        .summary-bar {
            display: flex;
            gap: 16px;
            padding: 12px 0;
            margin-bottom: 14px;
            flex-wrap: wrap;
            align-items: center;
            font-size: 0.9rem;
        }

        .detail-block {
            background: var(--surface-alt);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 16px;
            margin-bottom: 14px;
        }

        .monospace { font-family: 'Cascadia Code', 'Fira Code', 'JetBrains Mono', 'Consolas', monospace; }

        .no-data {
            color: var(--text-muted);
            font-style: italic;
            padding: 20px;
            text-align: center;
        }

        h3 {
            color: var(--primary);
            font-size: 1rem;
            margin: 20px 0 10px 0;
            padding-bottom: 6px;
            border-bottom: 1px solid var(--border);
        }

        /* ── Theme Toggle ── */
        .theme-toggle {
            position: fixed;
            bottom: 24px;
            right: 24px;
            width: 48px;
            height: 48px;
            border-radius: 50%;
            border: 1px solid var(--border);
            background: var(--surface);
            color: var(--text);
            cursor: pointer;
            box-shadow: 0 4px 16px var(--shadow);
            z-index: 200;
            font-size: 1.3rem;
            display: flex;
            align-items: center;
            justify-content: center;
            transition: all 0.3s;
        }
        .theme-toggle:hover {
            transform: scale(1.1);
            box-shadow: 0 6px 24px var(--shadow);
        }

        /* ── Scroll to Top ── */
        .scroll-top {
            position: fixed;
            bottom: 80px;
            right: 24px;
            width: 42px;
            height: 42px;
            border-radius: 50%;
            border: 1px solid var(--border);
            background: var(--surface);
            color: var(--text-muted);
            cursor: pointer;
            box-shadow: 0 2px 10px var(--shadow);
            z-index: 200;
            font-size: 1.1rem;
            display: none;
            align-items: center;
            justify-content: center;
            transition: all 0.3s;
        }
        .scroll-top:hover { color: var(--primary); transform: translateY(-2px); }
        .scroll-top.visible { display: flex; }

        /* ── Footer ── */
        .footer {
            text-align: center;
            padding: 30px;
            color: var(--text-muted);
            font-size: 0.8rem;
            border-top: 1px solid var(--border);
            margin-top: 40px;
        }

        /* ── Print ── */
        @media print {
            .nav-bar, .theme-toggle, .scroll-top, .table-controls, .filter-buttons { display: none !important; }
            .card.collapsed .section-content { max-height: none !important; padding: 20px 24px !important; }
            body { background: #fff; color: #000; }
        }

        /* ── Responsive ── */
        @media (max-width: 768px) {
            .header h1 { font-size: 1.6rem; }
            .info-grid { grid-template-columns: 1fr; }
            .container { padding: 12px; }
            .section-content { padding: 14px; }
            .filter-input { min-width: 100%; }
        }
    </style>
</head>
<body>

<button class="theme-toggle" onclick="toggleTheme()" title="Toggle theme">🌓</button>
<button class="scroll-top" onclick="scrollToTop()" title="Scroll to top">↑</button>

<div class="header">
    <h1>📊 $ReportTitle</h1>
    <div class="subtitle">Comprehensive system analysis and configuration report</div>
    <div class="meta">
        <span>🖥️ $ComputerName</span>
        <span>📅 $ReportDate</span>
        <span>⏱️ Uptime: $($sysOverview.Uptime)</span>
        <span>🔧 $($sysOverview.OSName)</span>
    </div>
</div>

<nav class="nav-bar">
    <a href="#section-overview">Overview</a>
    <a href="#section-cpu">CPU</a>
    <a href="#section-gpu">GPU</a>
    <a href="#section-memory">Memory</a>
    <a href="#section-disk-physical">Disks</a>
    <a href="#section-disk-logical">Volumes</a>
    <a href="#section-network">Network</a>
    <a href="#section-firewall">Firewall</a>
    <a href="#section-users">Users</a>
    <a href="#section-security">Security</a>
    <a href="#section-software">Software</a>
    <a href="#section-startup">Startup</a>
    <a href="#section-processes">Processes</a>
    <a href="#section-services">Services</a>
    <a href="#section-tasks">Tasks</a>
    <a href="#section-events">Events</a>
    <a href="#section-hotfixes">Hotfixes</a>
    <a href="#section-devices">Devices</a>
</nav>

<div class="container">
    
    <section id="section-overview" style="margin-bottom:20px;">
        $overviewHtml
    </section>

    $(MakeSection 'cpu' '🧠' 'CPU Information' $cpuHtml $cpuInfo.Count)
    $(MakeSection 'gpu' '🎮' 'GPU & Display' "$gpuHtml<h3>Monitors</h3>$monitorHtml" $gpuInfo.Count)
    $(MakeSection 'memory' '🧩' 'Memory' $memHtml $memInfo.Slots.Count)
    $(MakeSection 'disk-physical' '💾' 'Physical Disks' $diskPhysHtml $diskPhysical.Count)
    $(MakeSection 'disk-logical' '📁' 'Logical Volumes' $diskLogHtml $diskLogical.Count)
    $(MakeSection 'network' '🌐' 'Network Adapters' $netHtml $netAdapters.Count)
    $(MakeSection 'firewall' '🛡️' 'Firewall' $fwHtml)
    $(MakeSection 'users' '👥' 'Users & Groups' $usersHtml)
    $(MakeSection 'security' '🔒' 'Security Details' "$bitlockerHtml<h3>Pending Updates</h3>$updatesHtml")
    $(MakeSection 'software' '📦' 'Installed Software' $swHtml $software.Count)
    $(MakeSection 'startup' '🚀' 'Startup Applications' $startupHtml $startup.Count)
    $(MakeSection 'processes' '⚙️' 'Top Processes (by Memory)' $procHtml $processes.Count)
    $(MakeSection 'services' '🔧' 'Services' $svcHtml $services.Count)
    $(MakeSection 'tasks' '📋' 'Scheduled Tasks (Non-Microsoft)' $taskHtml $schedTasks.Count)
    $(MakeSection 'events' '📰' 'Event Log Summary (24h)' $eventHtml)
    $(MakeSection 'hotfixes' '🩹' 'Recent Hotfixes' $hfHtml $hotfixes.Count)
    $(MakeSection 'devices' '🔌' 'Peripheral Devices' "$('<h3>USB Devices</h3>')$usbHtml$('<h3>Audio Devices</h3>')$audioHtml" ($usbDevs.Count + $audio.Count))

</div>

<div class="footer">
    Generated on <strong>$ReportDate</strong> | Machine: <strong>$ComputerName</strong> | 
    PowerShell $($PSVersionTable.PSVersion) | Report Generator v2.0
</div>

<script>
    // ── Theme ──
    function toggleTheme() {
        const html = document.documentElement;
        const next = html.getAttribute('data-theme') === 'dark' ? 'light' : 'dark';
        html.setAttribute('data-theme', next);
        localStorage.setItem('sysreport-theme', next);
    }
    (function() {
        const saved = localStorage.getItem('sysreport-theme');
        if (saved) document.documentElement.setAttribute('data-theme', saved);
    })();

    // ── Section Collapse ──
    function toggleSection(id) {
        const card = document.getElementById('section-' + id);
        const indicator = document.getElementById('indicator-' + id);
        card.classList.toggle('collapsed');
    }

    // ── Row Expand ──
    function toggleRow(id) {
        const row = document.getElementById(id);
        if (!row) return;
        const isShown = row.classList.toggle('show');
        const prev = row.previousElementSibling;
        if (prev) {
            const icon = prev.querySelector('.expand-icon');
            if (icon) icon.style.transform = isShown ? 'rotate(90deg)' : 'rotate(0deg)';
        }
    }

    // ── Table Filter ──
    function filterTable(inputId, tableId) {
        const filter = document.getElementById(inputId).value.toUpperCase();
        const table = document.getElementById(tableId);
        if (!table) return;
        const rows = table.querySelectorAll('tbody tr');
        rows.forEach(row => {
            if (row.classList.contains('details-row')) return;
            const text = row.textContent.toUpperCase();
            const match = text.includes(filter);
            row.style.display = match ? '' : 'none';
            // Also hide associated details row
            const next = row.nextElementSibling;
            if (next && next.classList.contains('details-row') && !match) {
                next.style.display = 'none';
                next.classList.remove('show');
            }
        });
    }

    // ── Service State Filter ──
    function filterServiceState(btn, state) {
        btn.parentElement.querySelectorAll('.btn').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        const table = document.getElementById('serviceTable');
        if (!table) return;
        const rows = table.querySelectorAll('tbody tr');
        rows.forEach(row => {
            if (row.classList.contains('details-row')) {
                if (state === 'all') { /* leave as-is, controlled by expand */ }
                return;
            }
            if (state === 'all') {
                row.style.display = '';
                return;
            }
            const hasState = row.innerHTML.includes('>' + state + '<');
            row.style.display = hasState ? '' : 'none';
            const next = row.nextElementSibling;
            if (next && next.classList.contains('details-row') && !hasState) {
                next.style.display = 'none';
                next.classList.remove('show');
            }
        });
    }

    // ── Scroll to Top ──
    function scrollToTop() { window.scrollTo({ top: 0, behavior: 'smooth' }); }
    window.addEventListener('scroll', function() {
        const btn = document.querySelector('.scroll-top');
        if (window.scrollY > 400) btn.classList.add('visible');
        else btn.classList.remove('visible');
    });

    // ── Smooth scroll for nav links ──
    document.querySelectorAll('.nav-bar a').forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({ behavior: 'smooth', block: 'start' });
                // Ensure section is expanded
                const card = target.closest('.card') || target;
                if (card.classList.contains('collapsed')) {
                    card.classList.remove('collapsed');
                }
            }
        });
    });
</script>
</body>
</html>
"@

# ── Write Output ───────────────────────────────────────────────────────────────
$html | Out-File -FilePath $OutputPath -Encoding UTF8 -Force
Write-Host "`nReport saved to: $OutputPath" -ForegroundColor Green
Write-Host "Opening in default browser..." -ForegroundColor Cyan
Start-Process $OutputPath