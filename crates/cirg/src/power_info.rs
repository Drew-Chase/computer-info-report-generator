use crate::VariantExt;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use wmi::Variant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PowerInfo {
    pub plan: String,
    pub battery: Option<BatteryInfo>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatteryInfo {
    pub name: String,
    pub status: String,
    pub charge_pct: String,
    pub run_time_mins: String,
    pub design_capacity: String,
    pub full_charge_capacity: String,
    pub chemistry: String,
}

impl PowerInfo {
    pub fn fetch() -> Result<Self> {
        let output = Command::new("powercfg").arg("/getactivescheme").output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Parse output: "Power Scheme GUID: <guid>  (<name>)"
        let plan = stdout
            .lines()
            .find(|line| line.contains("Power Scheme GUID:"))
            .and_then(|line| {
                let start = line.rfind('(')?;
                let end = line.rfind(')')?;
                if start < end {
                    Some(line[start + 1..end].to_string())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "Unknown".to_string());
        let battery = BatteryInfo::fetch().ok();

        Ok(PowerInfo { plan, battery })
    }
}

impl BatteryInfo {
    pub fn fetch() -> Result<Self> {
        let com = wmi::WMIConnection::new()?;
        let results: Vec<HashMap<String, Variant>> =
            com.raw_query("SELECT * FROM Win32_Battery")?;

        let data = results.first().ok_or_else(|| anyhow!("No battery found"))?;

        let chemistry_code = data.get_u16("Chemistry").unwrap_or(0);
        let chemistry = match chemistry_code {
            1 => "Other",
            2 => "Unknown",
            3 => "Lead Acid",
            4 => "Nickel Cadmium",
            5 => "Nickel Metal Hydride",
            6 => "Lithium-ion",
            _ => "Other",
        };

        Ok(BatteryInfo {
            name: data.get_string("Name")?,
            status: data.get_string("Status")?,
            charge_pct: data.get_u16("EstimatedChargeRemaining")?.to_string(),
            run_time_mins: data.get_u32("EstimatedRunTime")?.to_string(),
            design_capacity: data
                .get_u32("DesignCapacity")
                .map(|v| v.to_string())
                .unwrap_or_else(|_| "Unknown".to_string()),
            full_charge_capacity: data
                .get_u32("FullChargeCapacity")
                .map(|v| v.to_string())
                .unwrap_or_else(|_| "Unknown".to_string()),
            chemistry: chemistry.to_string(),
        })
    }
}
