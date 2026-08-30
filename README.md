# portfolio-tui

A terminal-based (TUI) portfolio app built in Rust with [ratatui](https://github.com/ratatui/ratatui) and [crossterm](https://github.com/crossterm-rs/crossterm). It renders my portfolio — projects, tech stack, work experience, blog posts, and contact links — right in your terminal, fetching live data from the web portfolio's API.

It is the terminal companion to the web portfolio at [nevo.is-a.dev](https://nevo.is-a.dev).

## Screens

| # | Screen | Shows |
|---|--------|-------|
| 1 | Home | Logo, blurb, tagline |
| 2 | Projects | Project list + detail (live/source links) |
| 3 | Stack | Tech stack grouped by category |
| 4 | Experience | Work experience |
| 5 | Blog | Posts + reader pane |
| 6 | Links | GitHub, LinkedIn, web, email, resume |

## Requirements

- [Rust](https://rustup.rs/) toolchain (`cargo`)
- A terminal with ANSI support
- **Network access** — the app fetches its content from the live API at startup

## Install

```sh
cargo install --git https://github.com/iknevo/portfolio-tui
```

Then run it:

```sh
portfolio-tui
```

## Run from source

```sh
git clone https://github.com/iknevo/portfolio-tui
cd portfolio-tui
cargo run --release
```

## Usage / keybindings

### Global

| Key | Action |
|-----|--------|
| `1`–`6` | Jump to a screen (1 home, 2 projects, 3 stack, 4 experience, 5 blog, 6 links) |
| `Tab` / `Shift+Tab` | Next / previous screen |
| `h` / `←` | Back to Home |
| `q` | Quit |

### Projects

| Key | Action |
|-----|--------|
| `j` / `k` | Move up / down the list |
| `f` / `/` | Toggle focus between list and detail panes |
| `Enter` / `l` / `→` | Open the project's live URL |
| `s` | Open the project's source repo |
| `Esc` | Return focus to the list |

### Stack & Experience

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll down / up |
| `PgDn` / `PgUp` | Scroll by a page |
| `g` / `Home` | Jump to top |
| `G` / `End` | Jump to bottom |

### Blog

| Key | Action |
|-----|--------|
| `j` / `k` | Move through posts |
| `f` / `/` | Toggle focus between list and reader |
| `Enter` / `l` / `→` | Open the selected post (animated loader shows while the body loads) |
| `Esc` | Back to the post list |

### Links

| Key | Action |
|-----|--------|
| `g` | Open GitHub |
| `l` | Open LinkedIn |
| `w` | Open the web version |
| `e` | Compose email |
| `r` | Open resume |

## License

[MIT](./LICENSE)