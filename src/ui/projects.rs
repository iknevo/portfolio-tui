use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{App, Focus, LoadState},
    theme, ui,
};

pub fn draw(f: &mut Frame, area: Rect, app: &mut App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(44), Constraint::Percentage(56)])
        .split(area);

    draw_list(f, cols[0], app);
    draw_detail(f, cols[1], app);
}

fn draw_list(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = match &app.projects {
        LoadState::Loading => vec![ListItem::new(Line::from(Span::styled(
            "loading…",
            theme::dim(),
        )))],
        LoadState::Error(e) => vec![ListItem::new(Line::from(Span::styled(
            format!("! {e}"),
            Color::Red,
        )))],
        LoadState::Ready(projects) => projects
            .iter()
            .map(|p| {
                let mut lines = vec![Line::from(Span::styled(p.name.clone(), theme::heading()))];
                let stack = p.tech_stack.iter().take(3).cloned().collect::<Vec<_>>().join(" · ");
                if !stack.is_empty() {
                    lines.push(Line::from(Span::styled("  ".to_string() + &stack, theme::dim())));
                }
                ListItem::new(lines)
            })
            .collect(),
    };

    let mut state = ListState::default();
    let len = app.projects.ready().map(|v| v.len()).unwrap_or(0);
    state.select(Some(app.projects_idx.min(len.saturating_sub(1))));

    let focused = app.projects_focus == Focus::List;
    let list = List::new(items).block(ui::panel_for("PROJECTS", focused)).highlight_style(theme::selected()).highlight_symbol("> ");
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_detail(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.projects_focus == Focus::Detail;
    match app.selected_project() {
        None => {
            f.render_widget(
                Paragraph::new("No project selected.")
                    .block(ui::panel_for("DETAIL", focused)),
                area,
            );
        }
        Some(p) => {
            let mut lines: Vec<Line> = Vec::new();
            lines.push(Line::from(Span::styled(p.name.clone(), theme::accent())));
            lines.push(Line::from(Span::styled(
                format!(
                    "{} / {}",
                    p.year,
                    p.tech_stack.join(" · ")
                ),
                theme::muted(),
            )));
            lines.push(Line::from(""));
            lines.push(ui::rule());
            lines.push(Line::from(""));

            if !p.description.is_empty() {
                lines.push(ui::heading_line("DESCRIPTION"));
                lines.push(Line::from(Span::styled(p.description.clone(), theme::text())));
                lines.push(Line::from(""));
            }
            if !p.features.is_empty() {
                lines.push(ui::heading_line("FEATURES"));
                for feat in p.features.iter() {
                    lines.push(Line::from(vec![
                        Span::styled("  •  ", theme::accent()),
                        Span::styled(feat.clone(), theme::text()),
                    ]));
                }
                lines.push(Line::from(""));
            }
            if !p.tech_stack.is_empty() {
                lines.push(ui::heading_line("TECH STACK"));
                lines.push(Line::from(Span::styled(
                    p.tech_stack.join("   "),
                    theme::text(),
                )));
                lines.push(Line::from(""));
            }

            lines.push(ui::heading_line("LINKS"));
            lines.push(Line::from(vec![
                Span::styled("  [l] live   ", theme::dim()),
                Span::raw(p.live_url.as_deref().unwrap_or("—")),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  [s] source ", theme::dim()),
                Span::raw(p.source_code.as_deref().unwrap_or("—")),
            ]));

            let content = Paragraph::new(lines)
                .block(ui::panel_for("DETAIL", focused))
                .wrap(Wrap { trim: true });
            ui::render_scrollable(f, content, area, app.projects_scroll);
        }
    }
}