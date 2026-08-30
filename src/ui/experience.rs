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

    match &app.experience {
        LoadState::Loading => lines.push(Line::from(Span::styled("loading…", theme::dim()))),
        LoadState::Error(e) => lines.push(Line::from(Span::styled(
            format!("! {e}"),
            ratatui::style::Color::Red,
        ))),
        LoadState::Ready(items) if items.is_empty() => {
            lines.push(Line::from(Span::styled("No experience added yet.", theme::dim())));
        }
        LoadState::Ready(items) => {
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    lines.push(Line::from(""));
                }
                let end = item
                    .end_date
                    .clone()
                    .unwrap_or_else(|| "Present".to_string());
                lines.push(Line::from(vec![
                    Span::styled(item.company.clone(), theme::accent()),
                    Span::styled("  ·  ", theme::muted()),
                    Span::styled(format!("{start} — {end}", start = item.start_date), theme::dim()),
                ]));
                lines.push(Line::from(Span::styled(item.title.clone(), theme::text())));
            }
        }
    }

    let content = Paragraph::new(lines)
        .block(ui::panel("EXPERIENCE"))
        .wrap(Wrap { trim: true });
    ui::render_scrollable(f, content, area, app.exp_scroll);
}