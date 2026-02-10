use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout as RLayout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Terminal,
};

use crate::model::{Containment, Layout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Plasma,
    KWin,
    Help,
}

impl Tab {
    fn titles() -> [&'static str; 3] {
        ["Plasma", "KWin", "Help"]
    }
    fn idx(self) -> usize {
        match self {
            Tab::Plasma => 0,
            Tab::KWin => 1,
            Tab::Help => 2,
        }
    }
    fn from_idx(i: usize) -> Self {
        match i {
            0 => Tab::Plasma,
            1 => Tab::KWin,
            _ => Tab::Help,
        }
    }
}

pub fn run(layout: Layout) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut app = App::new(layout);

    let res = (|| {
        loop {
            term.draw(|f| app.draw(f.size(), f))?;

            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(k) = event::read()? {
                    if app.on_key(k)? {
                        break;
                    }
                }
            }
        }
        Ok::<_, anyhow::Error>(())
    })();

    disable_raw_mode()?;
    execute!(term.backend_mut(), LeaveAlternateScreen)?;
    term.show_cursor()?;

    res?;
    Ok(())
}

struct App {
    layout: Layout,
    tab: Tab,

    plasma_sel: usize,
    kwin_sel: usize,
}

impl App {
    fn new(layout: Layout) -> Self {
        Self {
            layout,
            tab: Tab::Plasma,
            plasma_sel: 0,
            kwin_sel: 0,
        }
    }

    fn on_key(&mut self, k: KeyEvent) -> Result<bool> {
        match (k.code, k.modifiers) {
            (KeyCode::Char('q'), _) => return Ok(true),
            (KeyCode::Esc, _) => return Ok(true),

            (KeyCode::Tab, _) => {
                let next = (self.tab.idx() + 1) % 3;
                self.tab = Tab::from_idx(next);
            }
            (KeyCode::BackTab, _) => {
                let prev = (self.tab.idx() + 3 - 1) % 3;
                self.tab = Tab::from_idx(prev);
            }

            _ => {}
        }

        match self.tab {
            Tab::Plasma => self.on_key_plasma(k),
            Tab::KWin => self.on_key_kwin(k),
            Tab::Help => Ok(false),
        }
    }

    fn on_key_plasma(&mut self, k: KeyEvent) -> Result<bool> {
        match k.code {
            KeyCode::Up => self.plasma_sel = self.plasma_sel.saturating_sub(1),
            KeyCode::Down => {
                let max = self.layout.containments.len().saturating_sub(1);
                self.plasma_sel = (self.plasma_sel + 1).min(max);
            }
            _ => {}
        }
        Ok(false)
    }

    fn on_key_kwin(&mut self, k: KeyEvent) -> Result<bool> {
        match k.code {
            KeyCode::Up => self.kwin_sel = self.kwin_sel.saturating_sub(1),
            KeyCode::Down => {
                let max = kwin_left_items(&self.layout).len().saturating_sub(1);
                self.kwin_sel = (self.kwin_sel + 1).min(max);
            }
            _ => {}
        }
        Ok(false)
    }

    fn draw(&mut self, area: Rect, f: &mut ratatui::Frame) {
        let root = RLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        let tabs = Tabs::new(
            Tab::titles()
                .iter()
                .map(|t| Line::from(*t))
                .collect::<Vec<_>>(),
        )
        .select(self.tab.idx())
        .block(block_rounded("kdesktop-copycat"))
        .style(Style::default())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        f.render_widget(tabs, root[0]);

        match self.tab {
            Tab::Plasma => self.draw_plasma(root[1], f),
            Tab::KWin => self.draw_kwin(root[1], f),
            Tab::Help => self.draw_help(root[1], f),
        }

        let hint = match self.tab {
            Tab::Plasma => "↑/↓ navigate • Tab switch tabs • q quit",
            Tab::KWin => "↑/↓ navigate • Tab switch tabs • q quit",
            Tab::Help => "Tab switch tabs • q quit",
        };

        let footer = Paragraph::new(Line::from(vec![
            Span::styled("Keys: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(hint),
        ]))
        .block(block_rounded("Help"));
        f.render_widget(footer, root[2]);
    }

    fn draw_plasma(&mut self, area: Rect, f: &mut ratatui::Frame) {
        let cols = RLayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(area);

        let items: Vec<ListItem> = self
            .layout
            .containments
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let title = containment_title(c);
                if i == self.plasma_sel {
                    ListItem::new(Line::from(Span::styled(
                        title,
                        Style::default().add_modifier(Modifier::BOLD),
                    )))
                } else {
                    ListItem::new(title)
                }
            })
            .collect();

        f.render_widget(
            List::new(items).block(block_rounded("Containments")),
            cols[0],
        );

        let detail = if let Some(c) = self.layout.containments.get(self.plasma_sel) {
            containment_details(c)
        } else {
            "No containment selected.".to_string()
        };

        f.render_widget(
            Paragraph::new(detail)
                .wrap(Wrap { trim: false })
                .block(block_rounded("Details")),
            cols[1],
        );
    }

    fn draw_kwin(&mut self, area: Rect, f: &mut ratatui::Frame) {
        let cols = RLayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
            .split(area);

        let left_items = kwin_left_items(&self.layout);
        let items: Vec<ListItem> = left_items
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if i == self.kwin_sel {
                    ListItem::new(Line::from(Span::styled(
                        s.clone(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )))
                } else {
                    ListItem::new(s.clone())
                }
            })
            .collect();

        f.render_widget(List::new(items).block(block_rounded("KWin")), cols[0]);

        let detail = kwin_right_panel(&self.layout, self.kwin_sel);
        f.render_widget(
            Paragraph::new(detail)
                .wrap(Wrap { trim: false })
                .block(block_rounded("Summary")),
            cols[1],
        );
    }

    fn draw_help(&mut self, area: Rect, f: &mut ratatui::Frame) {
        let text = r#"kdesktop-copycat

        Goal:
        Turn your KDE Plasma 6 panels, widgets, and desktops into a portable bundle.

        This viewer shows:
        • Plasma containments (desktops + panels)
        • Panel widget order (AppletOrder) when present
        • KWin Tier-1 summary:
        - enabled effects (kwinrc [Plugins] *Enabled=true)
        - enabled scripts (same)
        - task switcher (kwinrc [TabBox], [TabBoxAlternative])
        - window rules count (kwinrulesrc numeric groups)

        Notes:
        • Desktop containments often show as plugin org.kde.plasma.folder (Folder View).
        • Panels show as plugin org.kde.panel.
        "#;

        f.render_widget(
            Paragraph::new(text)
                .wrap(Wrap { trim: false })
                .block(block_rounded("Help")),
            area,
        );
    }
}

fn block_rounded(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}

fn containment_title(c: &Containment) -> String {
    if c.is_panel {
        format!(
            "Panel #{}  ({})",
            c.id,
            c.plugin.as_deref().unwrap_or("unknown")
        )
    } else {
        format!(
            "Containment #{}  ({})",
            c.id,
            c.plugin.as_deref().unwrap_or("unknown")
        )
    }
}

fn containment_details(c: &Containment) -> String {
    let mut out = String::new();
    out.push_str(&format!("{}\n", containment_title(c)));
    out.push_str(&format!("is_panel: {}\n", c.is_panel));
    out.push('\n');

    for k in [
        "location",
        "formfactor",
        "lastScreen",
        "immutability",
        "plugin",
    ] {
        if let Some(v) = c.meta.get(k) {
            out.push_str(&format!("{k}={v}\n"));
        }
    }

    if let Some(order) = &c.applet_order {
        out.push('\n');
        out.push_str("AppletOrder: ");
        out.push_str(
            &order
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" → "),
        );
        out.push('\n');
    }

    out.push('\n');
    out.push_str("Widgets:\n");
    for a in c.applets_in_order() {
        let plug = a.plugin.as_deref().unwrap_or("unknown");
        out.push_str(&format!("  • #{}  {}\n", a.id, plug));
    }

    out
}

fn kwin_left_items(layout: &Layout) -> Vec<String> {
    if layout.kwin.is_none() {
        return vec!["(kwinrc not found)".to_string()];
    }
    vec![
        "Tier-1 Summary".to_string(),
        "Enabled Effects".to_string(),
        "Enabled Scripts".to_string(),
        "Task Switcher".to_string(),
        "Task Switcher (Alt)".to_string(),
        "Window Rules".to_string(),
    ]
}

fn kv_map_to_lines(map: &std::collections::BTreeMap<String, String>, max: usize) -> String {
    let mut out = String::new();
    for (i, (k, v)) in map.iter().enumerate() {
        if i >= max {
            out.push_str(&format!("… ({} more)\n", map.len().saturating_sub(max)));
            break;
        }
        out.push_str(&format!("{k}={v}\n"));
    }
    out
}

fn kwin_right_panel(layout: &Layout, sel: usize) -> String {
    let Some(kw) = &layout.kwin else {
        return "No kwinrc loaded.\nTip: make sure ~/.config/kwinrc exists.".to_string();
    };

    let s = &kw.summary;

    match sel {
        0 => {
            let mut out = String::new();
            out.push_str("Tier-1 Summary\n\n");
            out.push_str(&format!("kwinrc: {}\n", kw.kwinrc));
            out.push_str(&format!("kwinrulesrc: {}\n\n", kw.kwinrulesrc));
            out.push_str(&format!("Enabled effects: {}\n", s.enabled_effects.len()));
            out.push_str(&format!("Enabled scripts: {}\n", s.enabled_scripts.len()));
            out.push_str(&format!("Window rules: {}\n\n", s.window_rules_count));

            if !s.enabled_effects.is_empty() {
                out.push_str("Top effects:\n");
                for e in s.enabled_effects.iter().take(12) {
                    out.push_str(&format!("  • {e}\n"));
                }
                out.push('\n');
            }
            if !s.enabled_scripts.is_empty() {
                out.push_str("Top scripts:\n");
                for e in s.enabled_scripts.iter().take(12) {
                    out.push_str(&format!("  • {e}\n"));
                }
                out.push('\n');
            }
            if !s.task_switcher.is_empty() {
                out.push_str("Task switcher (TabBox):\n");
                for key in ["LayoutName", "Mode", "ShowDesktop", "HighlightWindows"] {
                    if let Some(v) = s.task_switcher.get(key) {
                        out.push_str(&format!("  {key}={v}\n"));
                    }
                }
                out.push('\n');
            }
            out
        }
        1 => {
            let mut out = String::new();
            out.push_str("Enabled Effects\n\n");
            if s.enabled_effects.is_empty() {
                out.push_str("(none detected)\n");
            } else {
                for e in &s.enabled_effects {
                    out.push_str(&format!("• {e}\n"));
                }
            }
            out
        }
        2 => {
            let mut out = String::new();
            out.push_str("Enabled Scripts\n\n");
            if s.enabled_scripts.is_empty() {
                out.push_str("(none detected)\n");
            } else {
                for e in &s.enabled_scripts {
                    out.push_str(&format!("• {e}\n"));
                }
            }
            out
        }
        3 => {
            let mut out = String::new();
            out.push_str("Task Switcher (TabBox)\n\n");
            if s.task_switcher.is_empty() {
                out.push_str("(no TabBox section found)\n");
            } else {
                out.push_str(&kv_map_to_lines(&s.task_switcher, 60));
            }
            out
        }
        4 => {
            let mut out = String::new();
            out.push_str("Task Switcher (TabBoxAlternative)\n\n");
            if s.task_switcher_alternative.is_empty() {
                out.push_str("(no TabBoxAlternative section found)\n");
            } else {
                out.push_str(&kv_map_to_lines(&s.task_switcher_alternative, 60));
            }
            out
        }
        5 => format!(
            "Window Rules\n\nRules detected in kwinrulesrc: {}\n\n(Note: Tier-1 count of numeric groups like [1], [2], ...)",
                     s.window_rules_count
        ),
        _ => "Select an item on the left.".to_string(),
    }
}
