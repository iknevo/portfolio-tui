pub mod blog;
pub mod experience;
pub mod home;
pub mod links;
pub mod projects;
pub mod stack;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use crate::app::{App, Screen};
use crate::theme;

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(f, chunks[0]);
    render_tabs(f, chunks[1], app.screen);

    match app.screen {
        Screen::Home => home::draw(f, chunks[2], app),
        Screen::Projects => projects::draw(f, chunks[2], app),
        Screen::Stack => stack::draw(f, chunks[2], app),
        Screen::Experience => experience::draw(f, chunks[2], app),
        Screen::Blog => blog::draw(f, chunks[2], app),
        Screen::Links => links::draw(f, chunks[2], app),
    }

    render_footer(f, chunks[3], app.screen);
}

fn render_header(f: &mut Frame, area: Rect) {
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("NEVO", theme::accent()),
            Span::styled("  ·  terminal portfolio", theme::heading()),
        ]))
        .alignment(ratatui::layout::Alignment::Left),
        area,
    );
}

fn render_tabs(f: &mut Frame, area: Rect, screen: Screen) {
    let mut spans: Vec<Span<'static>> = Vec::new();
    for (i, tab) in Screen::ALL.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("   "));
        }
        spans.push(Span::styled(format!("{} ", i + 1), theme::muted()));
        if *tab == screen {
            spans.push(Span::styled(tab.label(), theme::accent()));
        } else {
            spans.push(Span::styled(tab.label(), Style::default().fg(theme::DIM)));
        }
    }

    f.render_widget(Paragraph::new(Line::from(spans)).alignment(ratatui::layout::Alignment::Left), area);
}

fn render_footer(f: &mut Frame, area: Rect, screen: Screen) {
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            footer_for(screen),
            theme::dim(),
        )))
        .alignment(ratatui::layout::Alignment::Center),
        area,
    );
}

pub fn panel(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme::muted())
        .title(Line::from(Span::styled(
            format!(" {title} "),
            theme::accent(),
        )))
}

pub fn panel_focus(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .border_style(theme::selected())
        .title(Line::from(Span::styled(
            format!(" {title} "),
            theme::selected(),
        )))
}

pub fn panel_for(title: &str, focused: bool) -> Block<'static> {
    if focused {
        panel_focus(title)
    } else {
        panel(title)
    }
}

pub fn heading_line(text: &str) -> Line<'static> {
    Line::from(Span::styled(text.to_string(), theme::accent()))
}

pub fn rule() -> Line<'static> {
    Line::from(Span::styled("─".repeat(24), theme::muted()))
}

pub fn render_scrollable<'a>(f: &mut Frame, para: Paragraph<'a>, area: Rect, offset: u16) {
    let inner_w = area.width.saturating_sub(2);
    let view_h = area.height.saturating_sub(2);
    let total = para.line_count(inner_w) as u16;

    if total <= view_h {
        f.render_widget(para, area);
        return;
    }

    let max = total.saturating_sub(view_h);
    let offset = offset.min(max);
    f.render_widget(para.scroll((offset, 0)), area);

    let mut state = ScrollbarState::new(total as usize)
        .position(offset as usize)
        .viewport_content_length(view_h as usize);
    let bar_area = Rect {
        x: area.x + area.width - 2,
        y: area.y + 1,
        width: 1,
        height: area.height.saturating_sub(2),
    };
    f.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .thumb_symbol("█")
            .thumb_style(Style::default().fg(theme::ACCENT))
            .track_symbol(Some("░"))
            .track_style(theme::muted())
            .begin_symbol(None)
            .end_symbol(None),
        bar_area,
        &mut state,
    );
}

fn footer_for(tab: Screen) -> &'static str {
    match tab {
        Screen::Home => "[p] top project   [o] email   [q] quit",
        Screen::Projects => "j/k move   f focus   enter/l live   s source   q quit",
        Screen::Stack => "j/k/pgup/pgdn scroll   g/G top/bottom   q quit",
        Screen::Experience => "j/k/pgup/pgdn scroll   g/G top/bottom   q quit",
        Screen::Blog => "j/k move   f focus   enter/l read   esc back   q quit",
        Screen::Links => "g github   l linkedin   w web   e email   r resume   q quit",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{BlogPost, BlogSummary, Experience, Project, Resume, StackGroup, StackItem};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn sample_app() -> App {
        let mut app = App::new();
        app.projects = crate::app::LoadState::Ready(vec![Project {
            name: "Smart Care".into(),
            year: 2026,
            live_url: Some("https://x.dev".into()),
            source_code: Some("https://github.com/x".into()),
            description: "A healthcare platform.".into(),
            features: vec!["Chat".into(), "Video".into()],
            tech_stack: vec!["Next.js".into(), "MongoDB".into()],
            thumbnail: String::new(),
            slug: Some("smart-care".into()),
            hide: false,
        }]);
        app.stack = crate::app::LoadState::Ready(vec![StackGroup {
            type_name: "frontend".into(),
            items: vec![StackItem {
                name: "React".into(),
                icon: String::new(),
                r#type: "frontend".into(),
            }],
        }]);
        app.experience = crate::app::LoadState::Ready(vec![Experience {
            title: "Dev".into(),
            company: "ITI".into(),
            start_date: "01/2026".into(),
            end_date: Some("07/2026".into()),
            hide: false,
        }]);
        app.blog = crate::app::LoadState::Ready(vec![BlogSummary {
            _id: Some("abc".into()),
            title: "Things I Believe".into(),
            summary: "A summary".into(),
            tags: vec![],
            image: None,
            reading_time: Some("3 min read".into()),
            slug: None,
            views: Some(60),
            created_at: None,
        }]);
        app.blog_post = crate::app::LoadState::Ready(Some(BlogPost {
            _id: Some("abc".into()),
            title: "Things I Believe".into(),
            summary: "".into(),
            tags: vec![],
            doc: "# Heading\n\n- item one\n- item two\n\nSome paragraph.".to_string(),
            reading_time: Some("3 min read".into()),
            slug: None,
            created_at: None,
        }));
        app.resume = crate::app::LoadState::Ready(Some(Resume {
            url: Some("https://resume".into()),
            updated_at: None,
        }));
        app
    }

    fn draw_screen(screen: Screen, app: &mut App) -> String {
        draw_screen_size(screen, app, 120, 40)
    }

    fn draw_screen_size(screen: Screen, app: &mut App, w: u16, h: u16) -> String {
        let backend = TestBackend::new(w, h);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|f| {
                app.screen = screen;
                draw(f, app);
            })
            .unwrap();
        let buf = terminal.backend().buffer();
        let width = buf.area.width as usize;
        let mut out = String::new();
        for (i, cell) in buf.content().iter().enumerate() {
            if i > 0 && i % width == 0 {
                if !out.ends_with('\n') {
                    out.push('\n');
                }
                continue;
            }
            let sym = cell.symbol();
            if sym.chars().all(|c| c == ' ') {
                if !out.ends_with(' ') && !out.ends_with('\n') {
                    out.push(' ');
                }
            } else {
                out.push_str(sym);
            }
        }
        out
    }

    #[test]
    fn all_screens_render() {
        let mut app = sample_app();
        for screen in Screen::ALL {
            let out = draw_screen(screen, &mut app);
            assert!(!out.is_empty(), "screen {screen:?} produced empty buffer");
            assert!(
                out.to_lowercase().contains(screen.label()),
                "screen {} should label itself, got:\n{}",
                screen.label().to_uppercase(),
                out
            );
        }
    }

    #[test]
    fn all_screens_render_narrow() {
        let mut app = sample_app();
        for screen in Screen::ALL {
            let out = draw_screen_size(screen, &mut app, 80, 24);
            assert!(!out.is_empty(), "narrow render of {screen:?} empty");
        }
    }

    #[test]
    fn home_shows_logo_and_age() {
        let mut app = sample_app();
        let out = draw_screen(Screen::Home, &mut app);
        assert!(out.contains("NEVO"), "home should show NEVO logo");
        assert!(out.to_lowercase().contains("cairo"), "home should mention Cairo");
    }

    #[test]
    fn project_screen_shows_detail() {
        let mut app = sample_app();
        let out = draw_screen(Screen::Projects, &mut app);
        assert!(out.contains("Smart Care"));
        assert!(out.contains("Next.js"));
    }

    #[test]
    fn blog_reader_renders_markdown() {
        let mut app = sample_app();
        let out = draw_screen(Screen::Blog, &mut app);
        assert!(out.contains("Things I Believe"));
        assert!(out.contains("Heading"));
        assert!(out.contains("item one"));
    }

    #[test]
    fn detail_focus_changes_borders() {
        let mut app = sample_app();
        // Projects: list focused by default.
        let focused = draw_screen_size(Screen::Projects, &mut app, 120, 40);
        app.projects_focus = crate::app::Focus::Detail;
        let unfocused = draw_screen_size(Screen::Projects, &mut app, 120, 40);
        assert_ne!(focused, unfocused, "focusing the detail pane should restyle the UI");
    }

    #[test]
    fn long_content_scrolls_and_is_clamped() {
        let mut app = sample_app();
        app.projects = crate::app::LoadState::Ready(vec![Project {
            name: "Long Project".into(),
            year: 2026,
            live_url: None,
            source_code: None,
            description: "word ".repeat(300),
            features: (0..50).map(|i| format!("feature number {i}")).collect(),
            tech_stack: vec![],
            thumbnail: String::new(),
            slug: None,
            hide: false,
        }]);
        app.projects_scroll = 10_000; // way beyond content
        let out = draw_screen_size(Screen::Projects, &mut app, 100, 20);
        // A scrollbar track must be drawn (▸ covers the ░ characters).
        assert!(out.contains('░'), "scrollbar track should render:\n{out}");
        // Content must still be visible (clamped, not blank).
        assert!(out.contains("Long Project") || out.contains("feature"));
    }

    #[test]
    fn blog_post_loading_resets_reader_scroll() {
        use crate::app::LoadingMsg;

        let mut app = sample_app();
        app.blog_scroll = 42;
        app.apply(LoadingMsg::BlogPost(Ok(None)));
        assert_eq!(app.blog_scroll, 0, "new post load must reset reader scroll");
    }

    #[test]
    fn links_use_letter_keys() {
        let out = draw_screen(Screen::Links, &mut sample_app());
        assert!(out.contains("[g]"), "links should show [g] github");
        assert!(out.contains("[l]"), "links should show [l] linkedin");
        assert!(out.contains("[e]"), "links should show [e] email");
        assert!(
            !out.contains("[1] github") && !out.contains("[2] linkedin") && !out.contains("[3] email"),
            "socials must not collide with the number navigation:\n{out}"
        );
    }

    #[test]
    fn links_show_hardcoded_resume_url() {
        let out = draw_screen(Screen::Links, &mut sample_app());
        assert!(
            out.contains("https://nevo.is-a.dev/resume"),
            "resume row should show the hardcoded URL:\n{out}"
        );
        assert!(
            !out.contains("no resume") && !out.contains("loading"),
            "resume row must not depend on API state:\n{out}"
        );
    }

    #[test]
    fn home_banner_is_centered() {
        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();
        let mut app = sample_app();
        terminal
            .draw(|f| {
                app.screen = Screen::Home;
                draw(f, &mut app);
            })
            .unwrap();

        let buf = terminal.backend().buffer();
        let width = buf.area.width;
        let mut col = None;
        for row in 0..buf.area.height {
            col = (0..width).fold(None, |acc, c| {
                if acc.is_some() {
                    acc
                } else {
                    let cell = &buf[(buf.area.x + c, row)];
                    if cell.symbol() == "█" {
                        Some(c)
                    } else {
                        None
                    }
                }
            });
            if col.is_some() {
                break;
            }
        }
        let col = col.expect("logo art (█) not found");
        assert!(
            col > 10 && col < width / 2,
            "logo should be centered, got column {col} in width {width}"
        );
    }

    #[test]
    fn home_logo_scales_with_width() {
        let out = draw_screen_size(Screen::Home, &mut sample_app(), 120, 40);
        assert!(
            out.contains("████████████"),
            "at 120 cols the logo should be doubled (S=2):\n{out}"
        );
    }

    #[test]
    fn home_logo_stays_small_on_narrow() {
        let out = draw_screen_size(Screen::Home, &mut sample_app(), 80, 24);
        assert!(
            !out.contains("████████████"),
            "at 80 cols the logo must stay single-size (S=1):\n{out}"
        );
    }
}