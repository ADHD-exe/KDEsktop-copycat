use anyhow::{Context, Result};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use walkdir::WalkDir;

use crate::model::Layout;
use crate::scripts;

#[derive(Debug, Clone)]
pub struct ExportOpts {
    pub out: PathBuf,
    pub snapshot: bool,
    pub bundle_plasmoids: bool,
}

fn mkdirp(p: &Path) -> Result<()> {
    fs::create_dir_all(p).with_context(|| format!("mkdir -p {}", p.display()))
}

fn write_file(p: &Path, content: &str, mode_executable: bool) -> Result<()> {
    if let Some(parent) = p.parent() {
        mkdirp(parent)?;
    }
    fs::write(p, content).with_context(|| format!("write {}", p.display()))?;
    if mode_executable {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perm = fs::metadata(p)?.permissions();
            perm.set_mode(0o755);
            fs::set_permissions(p, perm)?;
        }
    }
    Ok(())
}

fn copy_file(src: &Path, dst: &Path) -> Result<()> {
    if let Some(parent) = dst.parent() {
        mkdirp(parent)?;
    }
    fs::copy(src, dst).with_context(|| format!("copy {} -> {}", src.display(), dst.display()))?;
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let rel = entry.path().strip_prefix(src)?;
        let out_path = dst.join(rel);
        if entry.file_type().is_dir() {
            mkdirp(&out_path)?;
        } else if entry.file_type().is_file() {
            if let Some(parent) = out_path.parent() {
                mkdirp(parent)?;
            }
            fs::copy(entry.path(), &out_path).with_context(|| {
                format!("copy {} -> {}", entry.path().display(), out_path.display())
            })?;
        }
    }
    Ok(())
}

fn home_dir() -> Option<PathBuf> {
    dirs::home_dir()
}

fn user_plasmoid_dir(plugin_id: &str) -> Option<PathBuf> {
    let home = home_dir()?;
    Some(home.join(".local/share/plasma/plasmoids").join(plugin_id))
}

fn system_plasmoid_dir(plugin_id: &str) -> Option<PathBuf> {
    let p1 = PathBuf::from("/usr/share/plasma/plasmoids").join(plugin_id);
    if p1.is_dir() {
        return Some(p1);
    }
    let p2 = PathBuf::from("/usr/local/share/plasma/plasmoids").join(plugin_id);
    if p2.is_dir() {
        return Some(p2);
    }
    None
}

fn collect_plasmoid_ids(layout: &Layout) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for c in &layout.containments {
        for a in &c.applets {
            if let Some(pid) = &a.plugin {
                // Skip some pseudo/known non-plasmoid plugins if needed; generally safe to include all.
                out.insert(pid.clone());
            }
        }
    }
    out
}

pub fn export_bundle(layout: &Layout, opts: ExportOpts) -> Result<()> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let bundle_dir = opts.out.join(format!("plasma-layout-bundle-{}", ts));
    let scripts_dir = bundle_dir.join("scripts");
    let snapshot_dir = bundle_dir.join("snapshot");
    let plasmoids_dir = bundle_dir.join("plasmoids");

    mkdirp(&bundle_dir)?;
    mkdirp(&scripts_dir)?;

    // layout.json
    let layout_json = serde_json::to_string_pretty(layout)?;
    write_file(&bundle_dir.join("layout.json"), &layout_json, false)?;

    // scripts
    write_file(
        &scripts_dir.join("restore-layout.js"),
        &scripts::restore_layout_js(layout),
        false,
    )?;
    write_file(
        &scripts_dir.join("restore-portable.sh"),
        &scripts::restore_portable_sh(),
        true,
    )?;
    write_file(
        &scripts_dir.join("restore-snapshot.sh"),
        &scripts::restore_snapshot_sh(),
        true,
    )?;

    // snapshot
    if opts.snapshot {
        mkdirp(&snapshot_dir)?;
        let src = PathBuf::from(&layout.source_file);
        let dst = snapshot_dir.join("plasma-org.kde.plasma.desktop-appletsrc");
        copy_file(&src, &dst)?;
    }

    // plasmoids bundle (user-installed only, by default)
    if opts.bundle_plasmoids {
        mkdirp(&plasmoids_dir)?;
        for pid in collect_plasmoid_ids(layout) {
            // Prefer user-installed (this is what makes a layout portable)
            if let Some(p) = user_plasmoid_dir(&pid) {
                if p.is_dir() {
                    let dst = plasmoids_dir.join(&pid);
                    copy_dir_recursive(&p, &dst)?;
                    continue;
                }
            }

            // If not in user dir, do not bundle system plasmoids by default.
            // They should be installed by the OS. We leave them out intentionally.
            // (If you want, we can add a flag later to bundle system plasmoids too.)
            let _ = system_plasmoid_dir(&pid);
        }
    }

    eprintln!("Exported bundle: {}", bundle_dir.display());
    eprintln!(" - layout.json");
    eprintln!(" - scripts/restore-portable.sh");
    eprintln!(" - scripts/restore-snapshot.sh");
    if opts.snapshot {
        eprintln!(" - snapshot/plasma-org.kde.plasma.desktop-appletsrc");
    }
    if opts.bundle_plasmoids {
        eprintln!(" - plasmoids/ (user-installed plasmoids referenced by layout)");
    }

    Ok(())
}
