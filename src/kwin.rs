use anyhow::{Context, Result};
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;

use crate::model::{KWinScan, KWinSummary};

#[derive(Debug, Default)]
struct Ini {
    sections: BTreeMap<String, BTreeMap<String, String>>,
}

impl Ini {
    fn get_section(&self, name: &str) -> Option<&BTreeMap<String, String>> {
        self.sections.get(name)
    }
}

fn parse_ini(path: &str) -> Result<Ini> {
    let raw = fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
    let header_re = Regex::new(r"^\s*\[(.+?)\]\s*$")?;

    let mut ini = Ini::default();
    let mut cur: Option<String> = None;

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if let Some(c) = header_re.captures(line) {
            cur = Some(c.get(1).unwrap().as_str().to_string());
            continue;
        }

        let Some(sec) = cur.clone() else {
            continue;
        };
        if let Some((k, v)) = line.split_once('=') {
            ini.sections
                .entry(sec)
                .or_default()
                .insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    Ok(ini)
}

pub fn default_kwinrc() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    format!("{home}/.config/kwinrc")
}

pub fn default_kwinrulesrc() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    format!("{home}/.config/kwinrulesrc")
}

/// Tier-1 scan:
/// - enabled effects/scripts from [Plugins] `*Enabled=true`
/// - task switcher from [TabBox], [TabBoxAlternative]
/// - window rules count from kwinrulesrc numeric sections [1], [2], ...
pub fn load_kwin_info(kwinrc_path: &str, kwinrulesrc_path: &str) -> Result<KWinScan> {
    let mut summary = KWinSummary::default();

    // kwinrc
    if std::path::Path::new(kwinrc_path).exists() {
        let ini = parse_ini(kwinrc_path)?;

        if let Some(plugins) = ini.get_section("Plugins") {
            for (k, v) in plugins {
                // KDE stores as e.g. "blurEnabled=true" "kwin4_effect_translucencyEnabled=true"
                if k.ends_with("Enabled") && v.eq_ignore_ascii_case("true") {
                    let name = k.trim_end_matches("Enabled").to_string();
                    // Very rough split: effects often start with "kwin4_effect_" but scripts can too.
                    // We'll treat everything in Plugins as "enabled module" and later bucket by prefix.
                    if name.contains("effect") || name.contains("kwin4_effect") {
                        summary.enabled_effects.push(name);
                    } else {
                        summary.enabled_scripts.push(name);
                    }
                }
            }
        }

        if let Some(tabbox) = ini.get_section("TabBox") {
            summary.task_switcher = tabbox.clone();
        }
        if let Some(tabbox_alt) = ini.get_section("TabBoxAlternative") {
            summary.task_switcher_alternative = tabbox_alt.clone();
        }
    }

    // kwinrulesrc: count numeric groups like [1], [2], ...
    if std::path::Path::new(kwinrulesrc_path).exists() {
        let raw = fs::read_to_string(kwinrulesrc_path)
            .with_context(|| format!("reading {kwinrulesrc_path}"))?;
        let re = Regex::new(r"^\s*\[(\d+)\]\s*$")?;
        let mut max_id = 0usize;
        let mut seen = std::collections::BTreeSet::new();
        for line in raw.lines() {
            if let Some(c) = re.captures(line) {
                if let Ok(n) = c.get(1).unwrap().as_str().parse::<usize>() {
                    seen.insert(n);
                    if n > max_id {
                        max_id = n;
                    }
                }
            }
        }
        // Some files might skip numbers; count distinct numeric groups.
        summary.window_rules_count = seen.len();
    }

    // Sort for stable UI output
    summary.enabled_effects.sort();
    summary.enabled_scripts.sort();

    Ok(KWinScan {
        kwinrc: kwinrc_path.to_string(),
        kwinrulesrc: kwinrulesrc_path.to_string(),
        summary,
    })
}
