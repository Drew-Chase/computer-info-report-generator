use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use winreg::RegKey;
use winreg::enums::*;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComputerInfo {
    pub name: String,
    pub domain: String,
    pub manufacturer: String,
    pub system_type: String,
    pub operating_system: OSInfo,
    pub bios: BIOSInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OSInfo {
    pub name: String,
    pub version: String,
    pub build_lab: String,
    pub architecture: String,
    pub install_date: chrono::NaiveDateTime,
    pub last_boot_date: chrono::NaiveDateTime,
    pub uptime: u64,
    pub timezone: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BIOSInfo {
    pub manufacturer: String,
    pub version: String,
    pub release_date: chrono::NaiveDate,
}

impl ComputerInfo {
    pub async fn fetch() -> Result<ComputerInfo> {
        let com = wmi::WMIConnection::new()?;
        let results: Vec<HashMap<String, Variant>> =
            com.raw_query(r#"select * from Win32_ComputerSystem"#)?;
        let data = results
            .first()
            .ok_or_else(|| anyhow!("No OS info query data found"))?;

        Ok(ComputerInfo {
            name: data.get_string("Name")?,
            domain: if data.get_bool("PartOfDomain")? {
                data.get_string("Domain")?
            } else {
                format!("{} (Workgroup)", data.get_string("Workgroup")?)
            },
            manufacturer: data.get_string("Manufacturer")?,
            system_type: data.get_string("SystemType")?,
            operating_system: OSInfo::fetch().await?,
            bios: BIOSInfo::fetch().await?,
        })
    }
}

impl OSInfo {
    pub async fn fetch() -> Result<OSInfo> {
        let mut os_info = OSInfo::default();
        let com = wmi::WMIConnection::new()?;
        let results: Vec<HashMap<String, Variant>> =
            com.raw_query(r#"select * from Win32_OperatingSystem"#)?;
        let data = results
            .first()
            .ok_or_else(|| anyhow!("No OS info query data found"))?;

        os_info.name = data.get_string("Caption")?;

        // Get Windows version and build info from registry
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let cur_ver = hklm.open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion")?;

        // Get display version (e.g., "22H2", "23H2")
        os_info.version = cur_ver
            .get_value("DisplayVersion")
            .unwrap_or_else(|_| data.get_string("Version").unwrap_or_default());

        // Get build lab (e.g., "19041.1.amd64fre.vb_release.191206-1406")
        os_info.build_lab = cur_ver
            .get_value("BuildLabEx")
            .unwrap_or_else(|_| "N/A".to_string());

        os_info.architecture = data.get_string("OSArchitecture")?;
        os_info.install_date = chrono::NaiveDateTime::parse_from_str(
            &data.get_string("InstallDate")?[..14],
            "%Y%m%d%H%M%S",
        )?;
        os_info.last_boot_date = chrono::NaiveDateTime::parse_from_str(
            &data.get_string("LastBootUpTime")?[..14],
            "%Y%m%d%H%M%S",
        )?;

        // Calculate uptime in seconds
        let now = chrono::Utc::now().naive_utc();
        let uptime_duration = now.signed_duration_since(os_info.last_boot_date);
        os_info.uptime = uptime_duration.num_seconds() as u64;

        let tz_data: Vec<HashMap<String, Variant>> =
            com.raw_query(r#"select * from Win32_TimeZone"#)?;
        let tz_info = tz_data
            .first()
            .ok_or_else(|| anyhow!("No timezone info query data found"))?;
        os_info.timezone = tz_info.get_string("Caption")?;
        Ok(os_info)
    }
}

impl BIOSInfo {
    pub async fn fetch() -> Result<Self> {
        let mut bios_info: Self = BIOSInfo::default();
        let com = wmi::WMIConnection::new()?;
        let results: Vec<HashMap<String, Variant>> =
            com.raw_query(r#"select * from Win32_BIOS"#)?;
        let data = results
            .first()
            .ok_or_else(|| anyhow!("BIOS information not found in WMI query results"))?;

        bios_info.manufacturer = data.get_string("Manufacturer")?;
        bios_info.version = data.get_string("SMBIOSBIOSVersion")?;

        if let Ok(release_date) = data.get_string("ReleaseDate") {
            bios_info.release_date =
                chrono::NaiveDate::parse_from_str(&release_date[..8], "%Y%m%d")?;
        }

        Ok(bios_info)
    }
}

trait VariantExt {
    fn get_string(&self, key: &str) -> Result<String>;
    fn get_bool(&self, key: &str) -> Result<bool>;
}

impl VariantExt for HashMap<String, Variant> {
    fn get_string(&self, key: &str) -> Result<String> {
        match self.get(key) {
            Some(Variant::String(s)) => Ok(s.clone()),
            None => Err(anyhow!("Key not found in HashMap")),
            Some(_) => Err(anyhow!("Value for key is not a string")),
        }
    }

    fn get_bool(&self, key: &str) -> Result<bool> {
        match self.get(key) {
            Some(Variant::Bool(b)) => Ok(*b),
            None => Err(anyhow!("Key not found in HashMap")),
            Some(_) => Err(anyhow!("Value for key is not a boolean")),
        }
    }
}
