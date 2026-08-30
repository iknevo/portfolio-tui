use ratatui::{
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{app::App, identity, theme, ui};

pub fn draw(f: &mut Frame, area: ratatui::layout::Rect, _app: &mut App) {
    let mut lines: Vec<Line> = vec![];

    for (i, (name, url)) in identity::SOCIALS.iter().enumerate() {
        lines.push(line(format!("[{}]", key_for(i)), name, url));
    }
    lines.push(line("[e]".to_string(), "email", identity::EMAIL));
    lines.push(line("[o]".to_string(), "work", identity::EMAIL_WORK));
    lines.push(line("[w]".to_string(), "web", identity::WEB_URL));
    lines.push(line("[r]".to_string(), "resume", identity::RESUME_URL));

    f.render_widget(
        Paragraph::new(lines)
            .block(ui::panel("LINKS"))
            .wrap(Wrap { trim: true }),
        area,
    );
}

fn line<'a>(key: String, name: &'a str, url: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("{key:<6}"), theme::accent()),
        Span::styled(format!("{name:<9}"), theme::heading()),
        Span::styled(url, theme::dim()),
    ])
}

fn key_for(i: usize) -> char {
    match i {
        0 => 'g',
        _ => 'l',
    }
}
