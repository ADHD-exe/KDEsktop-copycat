use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub source_file: String,
    pub containments: Vec<Containment>,

    /// Optional Tier-1 KWin scan (kwinrc / kwinrulesrc summaries)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwin: Option<KWinScan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Containment {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,
    pub is_panel: bool,

    /// Raw key/value pairs from the containment section
    pub meta: BTreeMap<String, String>,

    pub applets: Vec<Applet>,

    /// Panel widget order (AppletOrder) if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applet_order: Option<Vec<u32>>,
}

impl Containment {
    /// Returns applets sorted using AppletOrder if present, otherwise by id.
    pub fn applets_in_order(&self) -> Vec<&Applet> {
        if let Some(order) = &self.applet_order {
            let mut out = Vec::new();
            for id in order {
                if let Some(a) = self.applets.iter().find(|x| &x.id == id) {
                    out.push(a);
                }
            }
            // Append any applets not in the order list
            for a in &self.applets {
                if !order.contains(&a.id) {
                    out.push(a);
                }
            }
            out
        } else {
            let mut out: Vec<&Applet> = self.applets.iter().collect();
            out.sort_by_key(|a| a.id);
            out
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Applet {
    pub id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<String>,

    /// Raw key/value pairs from the applet section
    pub meta: BTreeMap<String, String>,

    /// Configuration groups under Applets/<id>/Configuration/...
    pub config: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KWinScan {
    /// Path to the kwinrc that was scanned
    pub kwinrc: String,

    /// Path to the kwinrulesrc that was scanned
    pub kwinrulesrc: String,

    pub summary: KWinSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KWinSummary {
    pub enabled_effects: Vec<String>,
    pub enabled_scripts: Vec<String>,

    /// From [TabBox]
    pub task_switcher: BTreeMap<String, String>,

    /// From [TabBoxAlternative]
    pub task_switcher_alternative: BTreeMap<String, String>,

    pub window_rules_count: usize,
}
