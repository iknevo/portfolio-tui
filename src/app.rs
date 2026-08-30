use ratatui::{
    backend::Backend,
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    Terminal,
};
use tokio::sync::mpsc;

use crate::api::{Api, ApiError, BlogPost, BlogSummary, Experience, Project, Resume, StackGroup};
use crate::identity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Projects,
    Stack,
    Experience,
    Blog,
    Links,
}

impl Screen {
    pub const ALL: [Screen; 6] = [
        Screen::Home,
        Screen::Projects,
        Screen::Stack,
        Screen::Experience,
        Screen::Blog,
        Screen::Links,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Screen::Home => "home",
            Screen::Projects => "projects",
            Screen::Stack => "stack",
            Screen::Experience => "experience",
            Screen::Blog => "blog",
            Screen::Links => "links",
        }
    }

    pub fn next(self) -> Self {
        let i = Self::ALL.iter().position(|&t| t == self).unwrap();
        Self::ALL[(i + 1) % Self::ALL.len()]
    }

    pub fn prev(self) -> Self {
        let i = Self::ALL.iter().position(|&t| t == self).unwrap();
        Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Clone)]
pub enum LoadState<T> {
    Loading,
    Error(String),
    Ready(T),
}

impl<T> LoadState<T> {
    pub fn ready(&self) -> Option<&T> {
        match self {
            LoadState::Ready(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum LoadingMsg {
    Projects(Result<Vec<Project>, ApiError>),
    Stack(Result<Vec<StackGroup>, ApiError>),
    Experience(Result<Vec<Experience>, ApiError>),
    Blog(Result<Vec<BlogSummary>, ApiError>),
    BlogPost(Result<Option<BlogPost>, ApiError>),
    Resume(Result<Option<Resume>, ApiError>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    List,
    Detail,
}

const PAGE_SCROLL: u16 = 8;

pub struct App {
    pub screen: Screen,
    pub projects: LoadState<Vec<Project>>,
    pub stack: LoadState<Vec<StackGroup>>,
    pub experience: LoadState<Vec<Experience>>,
    pub blog: LoadState<Vec<BlogSummary>>,
    pub blog_post: LoadState<Option<BlogPost>>,
    pub resume: LoadState<Option<Resume>>,
    pub blog_reading: bool,
    pub blog_spinner: usize,
    pub projects_idx: usize,
    pub blog_idx: usize,
    pub stack_scroll: u16,
    pub exp_scroll: u16,
    pub projects_scroll: u16,
    pub blog_scroll: u16,
    pub projects_focus: Focus,
    pub blog_focus: Focus,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Home,
            projects: LoadState::Loading,
            stack: LoadState::Loading,
            experience: LoadState::Loading,
            blog: LoadState::Loading,
            blog_post: LoadState::Loading,
            resume: LoadState::Loading,
            blog_reading: false,
            blog_spinner: 0,
            projects_idx: 0,
            blog_idx: 0,
            stack_scroll: 0,
            exp_scroll: 0,
            projects_scroll: 0,
            blog_scroll: 0,
            projects_focus: Focus::List,
            blog_focus: Focus::List,
            quit: false,
        }
    }

    pub fn apply(&mut self, msg: LoadingMsg) {
        match msg {
            LoadingMsg::Projects(r) => self.projects = into_state(r),
            LoadingMsg::Stack(r) => self.stack = into_state(r),
            LoadingMsg::Experience(r) => self.experience = into_state(r),
            LoadingMsg::Blog(r) => self.blog = into_state(r),
            LoadingMsg::BlogPost(r) => {
                self.blog_post = into_state(r);
                self.blog_reading = false;
                self.blog_scroll = 0;
            }
            LoadingMsg::Resume(r) => self.resume = into_state(r),
        }
    }

    pub fn selected_project(&self) -> Option<&Project> {
        self.projects
            .ready()
            .and_then(|items| items.get(self.projects_idx.min(items.len().saturating_sub(1))))
    }

    pub fn selected_blog(&self) -> Option<&BlogSummary> {
        self.blog
            .ready()
            .and_then(|items| items.get(self.blog_idx.min(items.len().saturating_sub(1))))
    }
}

fn into_state<T>(r: Result<T, ApiError>) -> LoadState<T> {
    match r {
        Ok(v) => LoadState::Ready(v),
        Err(e) => LoadState::Error(e.to_string()),
    }
}

pub fn open_url(url: &str) {
    let _ = open::that(url);
}

pub fn open_mailto(email: &str) {
    open_url(&format!("mailto:{email}"));
}

pub fn spawn_loaders(api: Api) -> mpsc::UnboundedReceiver<LoadingMsg> {
    let (tx, rx) = mpsc::unbounded_channel();
    let tx1 = tx.clone();
    let api2 = api.clone();
    tokio::spawn(async move {
        let _ = tx1.send(LoadingMsg::Projects(api2.projects().await));
    });
    let tx1 = tx.clone();
    let api2 = api.clone();
    tokio::spawn(async move {
        let _ = tx1.send(LoadingMsg::Stack(api2.stack().await));
    });
    let tx1 = tx.clone();
    let api2 = api.clone();
    tokio::spawn(async move {
        let _ = tx1.send(LoadingMsg::Experience(api2.experience().await));
    });
    let tx1 = tx.clone();
    let api2 = api.clone();
    tokio::spawn(async move {
        let _ = tx1.send(LoadingMsg::Blog(api2.blog_list().await));
    });
    let tx1 = tx.clone();
    let api2 = api.clone();
    tokio::spawn(async move {
        let _ = tx1.send(LoadingMsg::Resume(api2.resume().await.map(Some)));
    });
    drop(tx);
    rx
}

fn spawn_blog_load(api: Api, id: String) -> mpsc::UnboundedReceiver<LoadingMsg> {
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        let r = api.blog_post(&id).await.map(Some);
        let _ = tx.send(LoadingMsg::BlogPost(r));
    });
    rx
}

pub fn run_tui<B: Backend<Error = std::io::Error>>(
    terminal: &mut Terminal<B>,
    api: Api,
    mut load_rx: mpsc::UnboundedReceiver<LoadingMsg>,
) -> std::io::Result<()> {
    let mut app = App::new();
    let mut blog_rx: Option<mpsc::UnboundedReceiver<LoadingMsg>> = None;

    loop {
        while let Ok(msg) = load_rx.try_recv() {
            app.apply(msg);
        }
        if let Some(rx) = blog_rx.as_mut() {
            while let Ok(msg) = rx.try_recv() {
                app.apply(msg);
            }
        }

        terminal.draw(|f| crate::ui::draw(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(80))? {
            let ev = event::read()?;
            if let event::Event::Key(key) = ev {
                if key.kind == KeyEventKind::Press && handle_key(&api, &mut app, key, &mut blog_rx)
                {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn handle_key(
    api: &Api,
    app: &mut App,
    key: KeyEvent,
    blog_rx: &mut Option<mpsc::UnboundedReceiver<LoadingMsg>>,
) -> bool {
    use KeyCode::*;

    if key.code == Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        app.quit = true;
        return true;
    }

    if let Some(screen) = number_screen(key.code) {
        app.screen = screen;
        return false;
    }

    match app.screen {
        Screen::Home => match key.code {
            Char('q') => return true,
            Char('o') | Char('O') => open_mailto("iknevo.dev@gmail.com"),
            Char('p') | Char('P') => {
                if let Some(p) = app.selected_project() {
                    if let Some(u) = &p.live_url {
                        open_url(u);
                    }
                }
            }
            _ => {}
        },
        Screen::Projects => match key.code {
            Char('q') => return true,
            Char('f') | Char('/') => app.projects_focus = toggle_focus(app.projects_focus),
            Char('h') | Left => app.screen = Screen::Home,
            Tab => app.screen = app.screen.next(),
            BackTab => app.screen = app.screen.prev(),
            Esc => app.projects_focus = Focus::List,
            _ if app.projects_focus == Focus::Detail => {
                scroll_key(key.code, &mut app.projects_scroll);
            }
            Char('j') | Down => {
                app.projects_idx = next_idx(&app.projects, app.projects_idx);
                app.projects_scroll = 0;
            }
            Char('k') | Up => {
                app.projects_idx = prev_idx(app.projects_idx);
                app.projects_scroll = 0;
            }
            Char('l') | Right | Enter => {
                if let Some(u) = &app.selected_project().and_then(|p| p.live_url.clone()) {
                    open_url(u);
                }
            }
            Char('s') | Char('S') => {
                if let Some(u) = &app.selected_project().and_then(|p| p.source_code.clone()) {
                    open_url(u);
                }
            }
            _ => {}
        },
        Screen::Stack => match key.code {
            Char('q') => return true,
            Char('j') | Down => app.stack_scroll = app.stack_scroll.saturating_add(1),
            Char('k') | Up => app.stack_scroll = app.stack_scroll.saturating_sub(1),
            Char('h') | Left => app.screen = Screen::Home,
            Tab => app.screen = app.screen.next(),
            BackTab => app.screen = app.screen.prev(),
            _ => scroll_key(key.code, &mut app.stack_scroll),
        },
        Screen::Experience => match key.code {
            Char('q') => return true,
            Char('j') | Down => app.exp_scroll = app.exp_scroll.saturating_add(1),
            Char('k') | Up => app.exp_scroll = app.exp_scroll.saturating_sub(1),
            Char('h') | Left => app.screen = Screen::Home,
            Tab => app.screen = app.screen.next(),
            BackTab => app.screen = app.screen.prev(),
            _ => scroll_key(key.code, &mut app.exp_scroll),
        },
        Screen::Blog => match key.code {
            Char('q') => return true,
            Char('f') | Char('/') => app.blog_focus = toggle_focus(app.blog_focus),
            Char('h') | Left => {
                app.screen = Screen::Home;
                app.blog_post = LoadState::Loading;
                app.blog_reading = false;
                *blog_rx = None;
            }
            Esc => {
                if app.blog_focus == Focus::Detail {
                    app.blog_focus = Focus::List;
                } else {
                    app.blog_post = LoadState::Loading;
                    app.blog_reading = false;
                    *blog_rx = None;
                }
            }
            Tab => app.screen = app.screen.next(),
            BackTab => app.screen = app.screen.prev(),
            _ if app.blog_focus == Focus::Detail => {
                scroll_key(key.code, &mut app.blog_scroll);
            }
            Char('j') | Down => {
                app.blog_idx = next_idx(&app.blog, app.blog_idx);
                app.blog_scroll = 0;
            }
            Char('k') | Up => {
                app.blog_idx = prev_idx(app.blog_idx);
                app.blog_scroll = 0;
            }
            Char('l') | Right | Enter => {
                if let Some(post) = app.selected_blog() {
                    if let Some(id) = post._id.clone() {
                        app.blog_post = LoadState::Loading;
                        app.blog_reading = true;
                        app.blog_scroll = 0;
                        *blog_rx = Some(spawn_blog_load(api.clone(), id));
                    }
                }
            }
            _ => {}
        },
        Screen::Links => match key.code {
            Char('q') => return true,
            Char('g') | Char('G') => open_url("https://github.com/iknevo"),
            Char('l') | Char('L') => open_url("https://www.linkedin.com/in/ahmed-abdelhafiez"),
            Char('e') | Char('E') => open_mailto("iknevo.dev@gmail.com"),
            Char('o') | Char('O') => open_mailto(identity::EMAIL_WORK),
            Char('r') | Char('R') => open_url(identity::RESUME_URL),
            Char('w') | Char('W') => open_url(identity::WEB_URL),
            Char('h') | Left => app.screen = Screen::Home,
            Tab => app.screen = app.screen.next(),
            BackTab => app.screen = app.screen.prev(),
            _ => {}
        },
    }

    false
}

fn next_idx<T>(state: &LoadState<Vec<T>>, idx: usize) -> usize {
    let len = state.ready().map(|v| v.len()).unwrap_or(0);
    if len == 0 {
        0
    } else {
        (idx + 1).min(len - 1)
    }
}

fn prev_idx(idx: usize) -> usize {
    idx.saturating_sub(1)
}

fn number_screen(code: KeyCode) -> Option<Screen> {
    match code {
        KeyCode::Char(c @ '1'..='6') => {
            let idx = (c as u8 - b'1') as usize;
            Some(Screen::ALL[idx.min(Screen::ALL.len() - 1)])
        }
        _ => None,
    }
}

fn toggle_focus(f: Focus) -> Focus {
    match f {
        Focus::List => Focus::Detail,
        Focus::Detail => Focus::List,
    }
}

fn scroll_key(code: KeyCode, offset: &mut u16) {
    use KeyCode::*;
    match code {
        Char('j') | Down => *offset = offset.saturating_add(1),
        Char('k') | Up => *offset = offset.saturating_sub(1),
        PageDown => *offset = offset.saturating_add(PAGE_SCROLL),
        PageUp => *offset = offset.saturating_sub(PAGE_SCROLL),
        Char('g') | Home => *offset = 0,
        Char('G') | End => *offset = u16::MAX,
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_nav_maps_1_to_6_tabs_in_order() {
        for (n, screen) in Screen::ALL.iter().enumerate() {
            let code = KeyCode::Char(char::from(b'1' + n as u8));
            assert_eq!(
                number_screen(code),
                Some(*screen),
                "digit {} should select {screen:?}",
                n + 1
            );
        }
    }

    #[test]
    fn number_nav_ignores_other_keys() {
        assert_eq!(number_screen(KeyCode::Char('7')), None);
        assert_eq!(number_screen(KeyCode::Char('0')), None);
        assert_eq!(number_screen(KeyCode::Char('j')), None);
        assert_eq!(number_screen(KeyCode::Left), None);
    }
}
