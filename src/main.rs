use std::io::{stdout, Stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use oxmail::homepage;
use oxmail::story::Story;
use ratatui::{prelude::*, widgets::*};

#[derive(PartialEq)]
enum Window {
    Home,
    Article,
}

struct App {
    stories: Vec<oxmail::story::Story>,
    list_state: ListState,
    window: Window,
}

impl App {
    fn get_items(&self) -> Vec<String> {
        self.stories
            .iter()
            .map(|story| story.headline.clone())
            .collect()
    }

    fn current_story(&mut self) -> &mut Story {
        self.stories
            .get_mut(self.list_state.selected().unwrap())
            .unwrap()
    }
}

fn draw_home_page(app: &mut App, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    let list = List::new(app.get_items())
        .block(
            Block::default()
                .title("Stories")
                .borders(Borders::ALL)
                .padding(Padding::top(1)),
        )
        .style(Style::default().fg(Color::Black))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);
    let _ = terminal.draw(|frame| {
        let area = frame.size();
        frame.render_stateful_widget(list, area, &mut app.list_state)
    });
}

fn draw_article(app: &mut App, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    let story = app.current_story();
    let content = story.get_content().clone().to_string();
    let title = story.headline.clone();
    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title(format!("Article - {}", title))
                .borders(Borders::ALL)
                .padding(Padding::new(2, 2, 1, 2)),
        )
        .style(Style::default().fg(Color::Black))
        .wrap(Wrap { trim: false });
    let _ = terminal.draw(|frame| {
        let area = frame.size();
        frame.render_widget(paragraph, area);
    });
}

fn increment_modulus(lhs: usize, modulus: usize) -> usize {
    (lhs + 1) % modulus
}

fn decrement_modulus(lhs: usize, modulus: usize) -> usize {
    if lhs == 0 {
        return modulus - 1;
    }
    lhs - 1
}

fn main() -> std::io::Result<()> {
    let document = homepage::home_page_html();
    let stories = homepage::article_links(&document);

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App {
        stories,
        list_state: ListState::default().with_selected(Some(0)),
        window: Window::Home,
    };

    loop {
        match app.window {
            Window::Home => draw_home_page(&mut app, &mut terminal),
            Window::Article => draw_article(&mut app, &mut terminal),
        }

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            break;
                        }
                        KeyCode::Down => {
                            if app.window == Window::Home {
                                if let Some(val) = app.list_state.selected_mut() {
                                    *val = increment_modulus(*val, app.stories.len());
                                };
                            };
                        }
                        KeyCode::Up => {
                            if app.window == Window::Home {
                                if let Some(val) = app.list_state.selected_mut() {
                                    *val = decrement_modulus(*val, app.stories.len());
                                };
                            }
                        }
                        KeyCode::Enter => {
                            app.window = match app.window {
                                Window::Article => Window::Home,
                                Window::Home => Window::Article,
                            };
                        }
                        KeyCode::Char('n') => {
                            if app.window == Window::Article {
                                if let Some(val) = app.list_state.selected_mut() {
                                    *val = increment_modulus(*val, app.stories.len())
                                };
                            }
                        }
                        KeyCode::Char('p') => {
                            if app.window == Window::Article {
                                if let Some(val) = app.list_state.selected_mut() {
                                    *val = decrement_modulus(*val, app.stories.len())
                                };
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
