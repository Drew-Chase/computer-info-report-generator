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
    pub pending_updates: Option<Vec<UpdateItem>>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItem {
    pub title: String,
    pub kb_article_ids: Vec<String>,
    pub severity: Option<String>,
    pub is_downloaded: bool,
    pub is_mandatory: bool,
    pub categories: Vec<String>,
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
        // Read from registry — works without admin privileges
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(key) = hklm.open_subkey(r"SYSTEM\CurrentControlSet\Control\SecureBoot\State") {
            return key
                .get_value::<u32, _>("UEFISecureBootEnabled")
                .map(|v| v == 1)
                .unwrap_or(false);
        }
        false
    }

    fn fetch_tpm() -> Option<TmpInfo> {
        #[cfg(target_os = "windows")]
        {
            // Try registry first (works without admin)
            if let Some(info) = Self::fetch_tpm_registry() {
                return Some(info);
            }

            // Try WMI (requires admin)
            if let Ok(tpm_info) = Self::fetch_tpm_wmi() {
                return Some(tpm_info);
            }

            None
        }
        #[cfg(not(target_os = "windows"))]
        {
            None
        }
    }

    fn fetch_tpm_registry() -> Option<TmpInfo> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

        // Check if TPM service exists
        let tpm_service_exists = hklm
            .open_subkey(r"SYSTEM\CurrentControlSet\Services\TPM")
            .is_ok();

        if !tpm_service_exists {
            return None;
        }

        // Read TPM version from registry
        let version = hklm
            .open_subkey(r"SOFTWARE\Microsoft\Tpm")
            .ok()
            .and_then(|key| {
                // Try SpecVersion first, then ManufacturerVersion
                key.get_value::<String, _>("SpecVersion")
                    .ok()
                    .and_then(|v| v.split(',').next().map(|s| s.trim().to_string()))
                    .or_else(|| key.get_value::<String, _>("ManufacturerVersion").ok())
            })
            .unwrap_or_else(|| "Unknown".to_string());

        // Read manufacturer info
        let manufacturer = hklm
            .open_subkey(r"SOFTWARE\Microsoft\Tpm")
            .ok()
            .and_then(|key| key.get_value::<String, _>("ManufacturerDisplayName").ok())
            .unwrap_or_else(|| "Unknown".to_string());

        // Check if TPM is ready via the IsReady registry value
        let ready = hklm
            .open_subkey(r"SOFTWARE\Microsoft\Tpm")
            .ok()
            .and_then(|key| key.get_value::<u32, _>("IsReady").ok())
            .map(|v| v == 1)
            .unwrap_or(false);

        Some(TmpInfo {
            present: true,
            ready,
            enabled: true, // If the service exists, it's enabled
            activated: ready,
            version,
            manufacturer,
        })
    }

    fn fetch_tpm_wmi() -> Result<TmpInfo> {
        let com = wmi::WMIConnection::with_namespace_path(r"root\cimv2\security\microsofttpm")?;

        let query = r#"SELECT * FROM Win32_Tpm"#;
        let results: Vec<HashMap<String, Variant>> = com.raw_query(query)?;

        if let Some(data) = results.first() {
            let present = true;
            let ready = data.get_bool("IsReady_InitialValue").unwrap_or(false);
            let enabled = data.get_bool("IsEnabled_InitialValue").unwrap_or(false);
            let activated = data.get_bool("IsActivated_InitialValue").unwrap_or(false);

            let version = data
                .get_string("SpecVersion")
                .ok()
                .and_then(|v| v.split(',').next().map(|s| s.trim().to_string()))
                .unwrap_or_else(|| "Unknown".to_string());

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
        // AntiVirusProduct lives in root\SecurityCenter2, not root\cimv2
        let com = wmi::WMIConnection::with_namespace_path(r"root\SecurityCenter2").ok()?;

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

        let query = r#"SELECT ProtectionStatus FROM Win32_EncryptableVolume"#;
        let results: Vec<HashMap<String, Variant>> = match com.raw_query(query) {
            Ok(r) => r,
            Err(_) => return false,
        };

        results.iter().any(|data| {
            data.get_u32("ProtectionStatus")
                .map(|status| status == 1)
                .unwrap_or(false)
        })
    }

    fn fetch_pending_updates() -> Option<Vec<UpdateItem>> {
        Self::query_pending_updates().ok()
    }

    fn query_pending_updates() -> windows::core::Result<Vec<UpdateItem>> {
        use windows::Win32::System::Com::{
            CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
        };
        use windows::Win32::System::UpdateAgent::{IUpdateSession, UpdateSession};
        use windows::core::BSTR;

        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

            let session: IUpdateSession =
                CoCreateInstance(&UpdateSession, None, CLSCTX_INPROC_SERVER)?;
            let searcher = session.CreateUpdateSearcher()?;
            searcher.SetOnline(false.into())?;

            let criteria = BSTR::from("IsInstalled=0 AND IsHidden=0");
            let result = searcher.Search(&criteria)?;
            let updates = result.Updates()?;
            let count = updates.Count()?;

            let mut items = Vec::new();
            for i in 0..count {
                let update = updates.get_Item(i)?;

                let title = update.Title()?.to_string();

                // Collect KB article IDs
                let mut kb_ids = Vec::new();
                let kb_collection = update.KBArticleIDs()?;
                let kb_count = kb_collection.Count()?;
                for k in 0..kb_count {
                    let kb = kb_collection.get_Item(k)?;
                    kb_ids.push(format!("KB{kb}"));
                }

                // MSRC severity — empty string becomes None
                let severity_bstr = update.MsrcSeverity()?;
                let severity_str = severity_bstr.to_string();
                let severity = if severity_str.is_empty() {
                    None
                } else {
                    Some(severity_str)
                };

                let is_downloaded = update.IsDownloaded()?.as_bool();
                let is_mandatory = update.IsMandatory()?.as_bool();

                // Collect categories
                let mut categories = Vec::new();
                let cat_collection = update.Categories()?;
                let cat_count = cat_collection.Count()?;
                for c in 0..cat_count {
                    let cat = cat_collection.get_Item(c)?;
                    let name = cat.Name()?.to_string();
                    categories.push(name);
                }

                items.push(UpdateItem {
                    title,
                    kb_article_ids: kb_ids,
                    severity,
                    is_downloaded,
                    is_mandatory,
                    categories,
                });
            }

            Ok(items)
        }
    }
}
