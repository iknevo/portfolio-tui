use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{app::App, identity, theme};

// const LOGO: &str = "
// ███╗   ██╗███████╗██╗   ██╗ ██████╗
// ████╗  ██║██╔════╝██║   ██║██╔═══██╗
// ██╔██╗ ██║█████╗  ██║   ██║██║   ██║
// ██║╚██╗██║██╔══╝  ╚██╗ ██╔╝██║   ██║
// ██║ ╚████║███████╗ ╚████╔╝ ╚██████╔╝
// ╚═╝  ╚═══╝╚══════╝  ╚═══╝   ╚═════╝
// ";

const LOGO: &str = "
 █████  ██   ██ ███    ███ ███████ ██████  
██   ██ ██   ██ ████  ████ ██      ██   ██ 
███████ ███████ ██ ████ ██ █████   ██   ██ 
██   ██ ██   ██ ██  ██  ██ ██      ██   ██ 
██   ██ ██   ██ ██      ██ ███████ ██████  
                                           
                                           
";

pub fn draw(f: &mut Frame, area: Rect, _app: &mut App) {
    let s = scale(area.width);
    let logo_h = 6 * s;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(logo_h),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .flex(Flex::Center)
        .split(area);

    f.render_widget(
        Paragraph::new(framed_logo(s)).alignment(Alignment::Center),
        chunks[0],
    );

    let blurb = Text::from(vec![
        Line::from(vec![
            Span::styled("Ahmed Abdelhafiez  ·  ", theme::heading()),
            Span::styled("frontend developer", theme::heading()),
        ])
        .alignment(Alignment::Center),
        Line::from(identity::TAGLINE).alignment(Alignment::Center),
    ]);
    f.render_widget(
        Paragraph::new(blurb)
            .style(theme::dim())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        chunks[2],
    );

    let age = identity::age();
    f.render_widget(
        Paragraph::new(
            Line::from(vec![
                Span::styled(format!("{age} years old"), theme::accent()),
                Span::styled(" · Cairo, Egypt", theme::heading()),
                Span::styled(" · i can't stop configuring my code editor", theme::dim()),
            ])
            .alignment(Alignment::Center),
        ),
        chunks[4],
    );
}

fn scale(width: u16) -> u16 {
    if width < 90 {
        1
    } else if width <= 159 {
        2
    } else {
        3
    }
}

fn framed_logo(s: u16) -> Vec<Line<'static>> {
    let lines: Vec<&str> = LOGO.lines().collect();

    let mut out: Vec<Line<'static>> = Vec::new();
    for l in lines.iter() {
        let scaled = l
            .chars()
            .flat_map(|c| std::iter::repeat_n(c, s as usize))
            .collect::<String>();
        for _ in 0..s {
            out.push(Line::from(Span::styled(scaled.clone(), theme::accent())));
        }
    }

    out
}
