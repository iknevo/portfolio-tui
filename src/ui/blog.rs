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
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    draw_list(f, cols[0], app);
    draw_reader(f, cols[1], app);
}

fn draw_list(f: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = match &app.blog {
        LoadState::Loading => vec![ListItem::new(Line::from(Span::styled(
            "loading…",
            theme::dim(),
        )))],
        LoadState::Error(e) => vec![ListItem::new(Line::from(Span::styled(
            format!("! {e}"),
            Color::Red,
        )))],
        LoadState::Ready(posts) => posts
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let mut lines = vec![Line::from(vec![
                    Span::styled(format!("{}. ", i + 1), theme::accent()),
                    Span::styled(p.title.clone(), theme::heading()),
                ])];
                let meta = p
                    .reading_time
                    .clone()
                    .map(|t| format!("{t} · {} views", p.views.unwrap_or(0)))
                    .unwrap_or_default();
                if !meta.is_empty() {
                    lines.push(Line::from(Span::styled("  ".to_string() + &meta, theme::dim())));
                }
                ListItem::new(lines)
            })
            .collect(),
    };

    let mut state = ListState::default();
    let len = app.blog.ready().map(|v| v.len()).unwrap_or(0);
    state.select(Some(app.blog_idx.min(len.saturating_sub(1))));

    let focused = app.blog_focus == Focus::List;
    let list = List::new(items).block(ui::panel_for("POSTS", focused)).highlight_style(theme::selected()).highlight_symbol("> ");
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_reader(f: &mut Frame, area: Rect, app: &mut App) {
    let focused = app.blog_focus == Focus::Detail;
    let block = ui::panel_for("READER", focused);
    let base: Vec<Line> = match &app.blog_post {
        LoadState::Loading => vec![Line::from(Span::styled(
            "select a post (enter)…",
            theme::muted(),
        ))],
        LoadState::Error(e) => vec![Line::from(Span::styled(format!("! {e}"), Color::Red))],
        LoadState::Ready(Some(post)) => render_post(post),
        LoadState::Ready(None) => vec![Line::from(Span::styled("no post", theme::muted()))],
    };

    if !matches!(&app.blog_post, LoadState::Ready(Some(_))) {
        f.render_widget(Paragraph::new(base).block(block), area);
        return;
    }

    let content = Paragraph::new(base).block(block).wrap(Wrap { trim: true });
    ui::render_scrollable(f, content, area, app.blog_scroll);
}

fn render_post(post: &crate::api::BlogPost) -> Vec<Line<'static>> {
    let mut out = vec![
        Line::from(Span::styled(post.title.clone(), theme::accent())),
        Line::from(Span::styled(
            post.reading_time.clone().unwrap_or_default(),
            theme::muted(),
        )),
        Line::from(""),
    ];

    for raw in post.doc.lines() {
        let t = raw.trim();
        if t.is_empty() {
            out.push(Line::from(""));
            continue;
        }
        if let Some(rest) = t.strip_prefix('#') {
            let level = t.bytes().take_while(|&b| b == b'#').count();
            let content = rest.trim_start();
            let style = if level == 1 { theme::accent() } else { theme::heading() };
            out.push(Line::from(Span::styled(
                strip_inline(content).to_string(),
                style,
            )));
        } else if let Some(rest) = t.strip_prefix("- ").or_else(|| t.strip_prefix("* ")) {
            out.push(Line::from(vec![
                Span::styled("  •  ", theme::accent()),
                Span::styled(strip_inline(rest).to_string(), theme::text()),
            ]));
        } else {
            out.push(Line::from(Span::styled(
                strip_inline(t).to_string(),
                theme::text(),
            )));
        }
    }

    out
}

fn strip_inline(s: &str) -> String {
    let mut text = s.to_string();
    // [label](url) -> label
    if let Some(rest) = text.strip_prefix('[') {
        if let Some(end) = rest.find("](") {
            let label = &rest[..end];
            if let Some(r) = rest.find(')') {
                let url = &rest[end + 2..r];
                if url.len() <= 1024 {
                    return label.to_string();
                }
            }
        }
    }

    for pair in [("**", ""), ("__", ""), ("`", ""), ("*", ""), ("~~", "")] {
        text = text.replace(pair.0, pair.1);
    }
    text
}