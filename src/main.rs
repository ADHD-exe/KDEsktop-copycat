mod export;
mod kwin;
mod model;
mod parser;
mod scripts;
mod tui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::kwin::{default_kwinrc, default_kwinrulesrc, load_kwin_info};
use crate::parser::parse_appletsrc;

#[derive(Parser)]
#[command(
    name = "kdesktop-copycat",
    version,
    about = "Turn your KDE Plasma 6 panels, widgets, and desktops into a portable bundle."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Parse plasma-org.kde.plasma.desktop-appletsrc and print JSON to stdout
    Scan {
        /// Optional appletsrc path (defaults to ~/.config/plasma-org.kde.plasma.desktop-appletsrc)
        #[arg(long)]
        file: Option<String>,

        /// Optional kwinrc path (defaults to ~/.config/kwinrc)
        #[arg(long)]
        kwinrc: Option<String>,

        /// Optional kwinrulesrc path (defaults to ~/.config/kwinrulesrc)
        #[arg(long)]
        kwinrules: Option<String>,
    },

    /// Open a TUI viewer for the parsed layout
    Tui {
        #[arg(long)]
        file: Option<String>,

        #[arg(long)]
        kwinrc: Option<String>,

        #[arg(long)]
        kwinrules: Option<String>,
    },

    /// Export a bundle (layout.json + scripts + optional snapshot + optional plasmoids)
    Export {
        #[arg(long)]
        out: PathBuf,

        #[arg(long, default_value_t = true)]
        snapshot: bool,

        #[arg(long, default_value_t = true)]
        bundle_plasmoids: bool,

        #[arg(long)]
        file: Option<String>,

        #[arg(long)]
        kwinrc: Option<String>,

        #[arg(long)]
        kwinrules: Option<String>,
    },
}

fn default_appletsrc() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    format!("{home}/.config/plasma-org.kde.plasma.desktop-appletsrc")
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Scan {
            file,
            kwinrc,
            kwinrules,
        } => {
            let applets = file.unwrap_or_else(default_appletsrc);
            let mut layout = parse_appletsrc(&applets)?;

            let kwinrc_path = kwinrc.unwrap_or_else(default_kwinrc);
            let kwinrules_path = kwinrules.unwrap_or_else(default_kwinrulesrc);

            if std::path::Path::new(&kwinrc_path).exists() {
                layout.kwin = Some(load_kwin_info(&kwinrc_path, &kwinrules_path)?);
            }

            println!("{}", serde_json::to_string_pretty(&layout)?);
        }

        Command::Tui {
            file,
            kwinrc,
            kwinrules,
        } => {
            let applets = file.unwrap_or_else(default_appletsrc);
            let mut layout = parse_appletsrc(&applets)?;

            let kwinrc_path = kwinrc.unwrap_or_else(default_kwinrc);
            let kwinrules_path = kwinrules.unwrap_or_else(default_kwinrulesrc);

            if std::path::Path::new(&kwinrc_path).exists() {
                layout.kwin = Some(load_kwin_info(&kwinrc_path, &kwinrules_path)?);
            }

            tui::run(layout)?;
        }

        Command::Export {
            out,
            snapshot,
            bundle_plasmoids,
            file,
            kwinrc,
            kwinrules,
        } => {
            let applets = file.unwrap_or_else(default_appletsrc);
            let mut layout = parse_appletsrc(&applets)?;

            let kwinrc_path = kwinrc.unwrap_or_else(default_kwinrc);
            let kwinrules_path = kwinrules.unwrap_or_else(default_kwinrulesrc);

            if std::path::Path::new(&kwinrc_path).exists() {
                layout.kwin = Some(load_kwin_info(&kwinrc_path, &kwinrules_path)?);
            }

            // IMPORTANT: your ExportOpts has `snapshot` (per compiler error), not `include_snapshot`
            let opts = export::ExportOpts {
                out,
                snapshot,
                bundle_plasmoids,
            };

            export::export_bundle(&layout, opts)?;
        }
    }

    Ok(())
}
