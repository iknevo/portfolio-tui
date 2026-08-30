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
    let inner_w = area.width.saturating_sub(2) as usize;

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
                lines.push(Line::from(vec![
                    Span::styled(format!("{}. ", gi + 1), theme::accent()),
                    Span::styled(group.type_name.to_uppercase(), theme::accent()),
                ]));
                let names = group.items.iter().map(|i| i.name.clone()).collect::<Vec<_>>();
                for row in item_rows(&names, inner_w) {
                    lines.push(Line::from(Span::styled(row, theme::text())));
                }
            }
        }
    }

    let content = Paragraph::new(lines)
        .block(ui::panel("STACK"))
        .wrap(Wrap { trim: false });
    ui::render_scrollable(f, content, area, app.stack_scroll);
}

const ITEM_INDENT: &str = "   ";
const BULLET: char = '•';

fn item_rows(names: &[String], width: usize) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();
    let mut current = String::new();
    for name in names {
        let token = format!("{BULLET} {name}");
        if current.is_empty() {
            current.push_str(ITEM_INDENT);
            current.push_str(&token);
        } else if current.len() + 1 + token.len() <= width {
            current.push(' ');
            current.push_str(&token);
        } else {
            rows.push(current);
            current = String::from(ITEM_INDENT);
            current.push_str(&token);
        }
    }
    if !current.is_empty() {
        rows.push(current);
    }
    rows
}