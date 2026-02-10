use anyhow::{bail, Context, Result};
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;

use crate::model::{Applet, Containment, Layout};

fn tokenize_section(section: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_bracket = false;
    for ch in section.chars() {
        match ch {
            '[' => {
                in_bracket = true;
                cur.clear();
            }
            ']' => {
                if in_bracket {
                    out.push(cur.clone());
                    in_bracket = false;
                }
            }
            _ => {
                if in_bracket {
                    cur.push(ch);
                }
            }
        }
    }
    out
}

#[derive(Debug, Default)]
struct IniTree {
    sections: BTreeMap<Vec<String>, BTreeMap<String, String>>,
}

impl IniTree {
    fn insert_kv(&mut self, path: Vec<String>, k: String, v: String) {
        self.sections.entry(path).or_default().insert(k, v);
    }
    fn section(&self, path: &[String]) -> Option<&BTreeMap<String, String>> {
        self.sections.get(path)
    }
    fn iter_sections(&self) -> impl Iterator<Item = (&Vec<String>, &BTreeMap<String, String>)> {
        self.sections.iter()
    }
}

fn parse_applet_order(s: &str) -> Option<Vec<u32>> {
    // AppletOrder=546;552;551;...
    let mut out = Vec::new();
    for part in s.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Ok(n) = part.parse::<u32>() {
            out.push(n);
        }
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

pub fn parse_appletsrc(path: &str) -> Result<Layout> {
    let raw = fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
    let header_re = Regex::new(r"^\s*(\[.*\])\s*$")?;

    let mut tree = IniTree::default();
    let mut current_path: Option<Vec<String>> = None;

    for (lineno, line) in raw.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }
        if let Some(cap) = header_re.captures(line) {
            let sec = cap.get(1).unwrap().as_str();
            let toks = tokenize_section(sec);
            if toks.is_empty() {
                bail!("bad section header at line {}: {}", lineno + 1, line);
            }
            current_path = Some(toks);
            continue;
        }

        let Some(path_tokens) = current_path.clone() else {
            continue;
        };
        if let Some((k, v)) = line.split_once('=') {
            tree.insert_kv(path_tokens, k.trim().to_string(), v.trim().to_string());
        }
    }

    let mut containment_ids: Vec<u32> = Vec::new();
    for (path, _) in tree.iter_sections() {
        if path.len() >= 2 && path[0] == "Containments" {
            if let Ok(id) = path[1].parse::<u32>() {
                if !containment_ids.contains(&id) {
                    containment_ids.push(id);
                }
            }
        }
    }
    containment_ids.sort_unstable();

    let mut containments = Vec::new();

    for cid in containment_ids {
        let cpath = vec!["Containments".to_string(), cid.to_string()];
        let meta = tree.section(&cpath).cloned().unwrap_or_default();

        let plugin = meta.get("plugin").cloned();
        let is_panel = plugin.as_deref() == Some("org.kde.panel");

        let applet_order = meta.get("AppletOrder").and_then(|s| parse_applet_order(s));

        let mut applet_ids: Vec<u32> = Vec::new();
        for (path, _) in tree.iter_sections() {
            if path.len() >= 4
                && path[0] == "Containments"
                && path[1] == cid.to_string()
                && path[2] == "Applets"
            {
                if let Ok(aid) = path[3].parse::<u32>() {
                    if !applet_ids.contains(&aid) {
                        applet_ids.push(aid);
                    }
                }
            }
        }
        applet_ids.sort_unstable();

        let mut applets = Vec::new();
        for aid in applet_ids {
            let apath = vec![
                "Containments".to_string(),
                cid.to_string(),
                "Applets".to_string(),
                aid.to_string(),
            ];
            let ameta = tree.section(&apath).cloned().unwrap_or_default();
            let aplug = ameta.get("plugin").cloned();

            let mut config: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
            for (spath, skv) in tree.iter_sections() {
                if spath.len() >= 5
                    && spath[0] == "Containments"
                    && spath[1] == cid.to_string()
                    && spath[2] == "Applets"
                    && spath[3] == aid.to_string()
                    && spath[4] == "Configuration"
                {
                    let group = if spath.len() == 5 {
                        "Configuration".to_string()
                    } else {
                        spath[5..].join("/")
                    };
                    config.insert(group, skv.clone());
                }
            }

            applets.push(Applet {
                id: aid,
                plugin: aplug,
                meta: ameta,
                config,
            });
        }

        containments.push(Containment {
            id: cid,
            plugin,
            is_panel,
            meta,
            applet_order,
            applets,
        });
    }

    Ok(Layout {
        source_file: path.to_string(),
        containments,
        kwin: None,
    })
}
