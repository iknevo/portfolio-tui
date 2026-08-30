use ratatui::{
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{App, LoadState},
    theme, ui,
};

pub fn draw(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    let mut lines: Vec<Line> = Vec::new();

    match &app.stack {
        LoadState::Loading => lines.push(Line::from(Span::styled("loading…", theme::dim()))),
        LoadState::Error(e) => lines.push(Line::from(Span::styled(
            format!("! {e}"),
            ratatui::style::Color::Red,
        ))),
        LoadState::Ready(groups) if groups.is_empty() => {
            lines.push(Line::from(Span::styled(
                "No stack added yet.",
                theme::dim(),
            )));
        }
        LoadState::Ready(groups) => {
            for (gi, group) in groups.iter().enumerate() {
                if gi > 0 {
                    lines.push(Line::from(""));
                }
                lines.push(ui::heading_line(&group.type_name.to_uppercase()));
                lines.push(Line::from(Span::styled(
                    group
                        .items
                        .iter()
                        .map(|i| i.name.clone())
                        .collect::<Vec<_>>()
                        .join("   "),
                    theme::text(),
                )));
            }
        }
    }

    let content = Paragraph::new(lines)
        .block(ui::panel("STACK"))
        .wrap(Wrap { trim: true });
    ui::render_scrollable(f, content, area, app.stack_scroll);
}