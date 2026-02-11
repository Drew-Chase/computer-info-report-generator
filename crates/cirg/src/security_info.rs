use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ComputerInfoExt, VariantExt};
use winreg::RegKey;
use winreg::enums::*;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityInfo {
    pub secure_boot: bool,
    pub tpm: Option<TmpInfo>,
    pub antivirus: Option<String>,
    pub firewall: Option<FirewallInfo>,
    pub uac: bool,
    pub rdp_enabled: bool,
    pub bit_locker: bool,
    pub pending_updates: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TmpInfo {
    pub present: bool,
    pub ready: bool,
    pub enabled: bool,
    pub activated: bool,
    pub version: String,
    pub manufacturer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FirewallInfo {
    pub domain_enabled: Option<bool>,
    pub domain_inbound: Option<String>,
    pub domain_outbound: Option<String>,
    pub private_enabled: Option<bool>,
    pub private_inbound: Option<String>,
    pub private_outbound: Option<String>,
    pub public_enabled: Option<bool>,
    pub public_inbound: Option<String>,
    pub public_outbound: Option<String>,
}

impl ComputerInfoExt for SecurityInfo {
    fn fetch() -> Result<Self> {
        Ok(SecurityInfo {
            secure_boot: Self::fetch_secure_boot(),
            tpm: Self::fetch_tpm(),
            antivirus: Self::fetch_antivirus(),
            firewall: Self::fetch_firewall(),
            uac: Self::fetch_uac(),
            rdp_enabled: Self::fetch_rdp_status(),
            bit_locker: Self::fetch_bitlocker(),
            pending_updates: Self::fetch_pending_updates(),
        })
    }
}
impl SecurityInfo {
    fn fetch_secure_boot() -> bool {
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::System::WindowsProgramming::GetFirmwareEnvironmentVariableW;

            // Try to query SecureBoot UEFI variable
            // This requires admin privileges
            let mut buffer = [0u8; 1];
            let result = unsafe {
                GetFirmwareEnvironmentVariableW(
                    windows::core::w!("SecureBoot"),
                    windows::core::w!("{8be4df61-93ca-11d2-aa0d-00e098032b8c}"),
                    Some(buffer.as_mut_ptr() as *mut _),
                    buffer.len() as u32,
                )
            };

            result > 0 && buffer[0] == 1
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    fn fetch_tpm() -> Option<TmpInfo> {
        #[cfg(target_os = "windows")]
        {
            // Try WMI first
            if let Ok(tpm_info) = Self::fetch_tpm_wmi() {
                return Some(tpm_info);
            }

            // Fallback: check registry to see if TPM service exists
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            if hklm
                .open_subkey(r"SYSTEM\CurrentControlSet\Services\TPM")
                .is_ok()
            {
                return Some(TmpInfo {
                    present: true,
                    ready: false,
                    enabled: false,
                    activated: false,
                    version: "Unknown (run as admin)".to_string(),
                    manufacturer: "Unknown (run as admin)".to_string(),
                });
            }

            // No TPM found
            None
        }
        #[cfg(not(target_os = "windows"))]
        {
            None
        }
    }

    fn fetch_tpm_wmi() -> Result<TmpInfo> {
        let com = wmi::WMIConnection::with_namespace_path(r"root\cimv2\security\microsofttpm")?;

        // Try querying Win32_Tpm from the TPM namespace
        let query = r#"SELECT * FROM Win32_Tpm"#;
        let results: Vec<HashMap<String, Variant>> = com.raw_query(query)?;

        if let Some(data) = results.first() {
            // The presence of WMI data indicates TPM is present
            let present = true;

            // Get TPM status fields
            let ready = data.get_bool("IsReady_InitialValue").unwrap_or(false);
            let enabled = data.get_bool("IsEnabled_InitialValue").unwrap_or(false);
            let activated = data.get_bool("IsActivated_InitialValue").unwrap_or(false);

            // Extract TPM version
            let version = data
                .get_string("SpecVersion")
                .ok()
                .and_then(|v| v.split(',').next().map(|s| s.trim().to_string()))
                .unwrap_or_else(|| "Unknown".to_string());

            // Get manufacturer
            let manufacturer = data
                .get_string("ManufacturerIdTxt")
                .unwrap_or_else(|_| "Unknown".to_string());

            return Ok(TmpInfo {
                present,
                ready,
                enabled,
                activated,
                version,
                manufacturer,
            });
        }

        Err(anyhow!("No TPM found"))
    }

    fn fetch_antivirus() -> Option<String> {
        let com = wmi::WMIConnection::new().ok()?;

        // Query SecurityCenter2 namespace for antivirus products
        let query = r#"SELECT displayName, productState FROM AntiVirusProduct"#;
        let results: Vec<HashMap<String, Variant>> = com.raw_query(query).ok()?;

        if results.is_empty() {
            return None;
        }

        let av_list: Vec<String> = results
            .iter()
            .filter_map(|data| {
                let name = data.get_string("displayName").ok()?;
                let state = data.get_u32("productState").ok()?;

                // Product state bits: 0x1000 = enabled, 0x0800 = up to date
                let enabled = (state & 0x1000) != 0;
                let up_to_date = (state & 0x0800) != 0;

                Some(format!(
                    "{} (Enabled: {}, Up-to-date: {})",
                    name, enabled, up_to_date
                ))
            })
            .collect();

        if av_list.is_empty() {
            None
        } else {
            Some(av_list.join("; "))
        }
    }

    fn fetch_firewall() -> Option<FirewallInfo> {
        // Query Windows Firewall settings from registry
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        let mut fw_info = FirewallInfo::default();
        let mut has_data = false;

        // Domain Profile
        if let Ok(key) = hklm.open_subkey(
            r"SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\DomainProfile"
        ) {
            fw_info.domain_enabled = Some(key.get_value::<u32, _>("EnableFirewall").unwrap_or(0) == 1);
            let inbound = key.get_value::<u32, _>("DefaultInboundAction").unwrap_or(1);
            let outbound = key.get_value::<u32, _>("DefaultOutboundAction").unwrap_or(0);
            fw_info.domain_inbound = Some(if inbound == 1 { "Block" } else { "Allow" }.to_string());
            fw_info.domain_outbound = Some(if outbound == 1 { "Block" } else { "Allow" }.to_string());
            has_data = true;
        }

        // Private Profile (StandardProfile)
        if let Ok(key) = hklm.open_subkey(
            r"SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\StandardProfile"
        ) {
            fw_info.private_enabled = Some(key.get_value::<u32, _>("EnableFirewall").unwrap_or(0) == 1);
            let inbound = key.get_value::<u32, _>("DefaultInboundAction").unwrap_or(1);
            let outbound = key.get_value::<u32, _>("DefaultOutboundAction").unwrap_or(0);
            fw_info.private_inbound = Some(if inbound == 1 { "Block" } else { "Allow" }.to_string());
            fw_info.private_outbound = Some(if outbound == 1 { "Block" } else { "Allow" }.to_string());
            has_data = true;
        }

        // Public Profile
        if let Ok(key) = hklm.open_subkey(
            r"SYSTEM\CurrentControlSet\Services\SharedAccess\Parameters\FirewallPolicy\PublicProfile"
        ) {
            fw_info.public_enabled = Some(key.get_value::<u32, _>("EnableFirewall").unwrap_or(0) == 1);
            let inbound = key.get_value::<u32, _>("DefaultInboundAction").unwrap_or(1);
            let outbound = key.get_value::<u32, _>("DefaultOutboundAction").unwrap_or(0);
            fw_info.public_inbound = Some(if inbound == 1 { "Block" } else { "Allow" }.to_string());
            fw_info.public_outbound = Some(if outbound == 1 { "Block" } else { "Allow" }.to_string());
            has_data = true;
        }

        if has_data { Some(fw_info) } else { None }
    }

    fn fetch_uac() -> bool {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(key) =
            hklm.open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System")
        {
            key.get_value::<u32, _>("EnableLUA")
                .map(|v| v == 1)
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn fetch_rdp_status() -> bool {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(key) = hklm.open_subkey(r"SYSTEM\CurrentControlSet\Control\Terminal Server") {
            let deny_connections = key.get_value::<u32, _>("fDenyTSConnections").unwrap_or(1);
            deny_connections == 0
        } else {
            false
        }
    }

    fn fetch_bitlocker() -> bool {
        let com = match wmi::WMIConnection::new() {
            Ok(c) => c,
            Err(_) => return false,
        };

        // Query BitLocker/EncryptableVolume information
        let query = r#"SELECT ProtectionStatus FROM Win32_EncryptableVolume"#;
        let results: Vec<HashMap<String, Variant>> = match com.raw_query(query) {
            Ok(r) => r,
            Err(_) => return false,
        };

        // Check if any volume has BitLocker protection enabled (ProtectionStatus == 1)
        results.iter().any(|data| {
            data.get_u32("ProtectionStatus")
                .map(|status| status == 1)
                .unwrap_or(false)
        })
    }

    fn fetch_pending_updates() -> String {
        // Windows Update API requires COM automation (IUpdateSession, IUpdateSearcher, etc.)
        // This is complex and would require additional COM bindings
        // For now, indicate this feature is not implemented
        "Not implemented (requires COM automation)".to_string()
    }
}
