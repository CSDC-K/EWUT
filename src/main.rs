use std::{io::{self, stdout}, time::Duration};
use color_eyre::eyre::Ok;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use crossterm::{
    ExecutableCommand, event::{self, Event, KeyCode, KeyEventKind}, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};

use eyre::Result;


struct App {
    input: String,
    messages: Vec<String>,
}

impl App {
    fn new() -> Self {
        Self {
            input: String::new(),
            messages: vec!["TEST".to_string()],
        }
    }

    // Sadece metni yukarı aktarır, komut çalıştırmaz
    fn push_message(&mut self) {
        if !self.input.is_empty() {
            self.messages.push(self.input.clone());
            self.input.clear();
        }
    }
}

fn main() -> Result<()>{
    enable_raw_mode();
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);


    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => { app.input.pop(); },
                    KeyCode::Enter => app.push_message(),
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(f.area());

    
    let list_items: Vec<ListItem> = app.messages
        .iter()
                .rev()
        .map(|m| ListItem::new(m.as_str()))
        .collect();

    let history = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(" Msgs "));
    f.render_widget(history, chunks[0]);

    // Input
    let input_area = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(" Input "));
    f.render_widget(input_area, chunks[1]);


    f.set_cursor_position((
        chunks[1].x + app.input.len() as u16 + 1,
        chunks[1].y + 1,
    ));
}