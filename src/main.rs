use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
    Frame, Terminal,
};
use std::io::{self, stdout};

// Import modules
#[path = "lib/ewutsearch_lib.rs"] mod ewutsearch_lib;
#[path = "lib/ewutcom_lib.rs"] mod ewutcom_lib;
#[path = "lib/ewutrm_lib.rs"] mod ewutrm_lib;

// Main App Struct
struct App {
    input: String,
    messages: Vec<String>,
    should_quit: bool,
    list_state: ListState,
    config: ewutrm_lib::EwutConfig, // We store config in memory for performance
}

impl App {
    fn new() -> Self {
        let config = ewutrm_lib::_event_load_configs();
        let startup_msg = ewutrm_lib::_LIBFUNC_getstartup();
        
        let mut messages = Vec::new();
        for line in startup_msg.lines() {
            messages.push(line.to_string());
        }

        let mut state = ListState::default();
        if !messages.is_empty() {
             state.select(Some(messages.len() - 1));
        }

        Self {
            input: String::new(),
            messages,
            should_quit: false,
            list_state: state,
            config,
        }
    }

    fn submit_message(&mut self) {
        let input_text = self.input.trim().to_string();
        if input_text.is_empty() { return; }

        // User Input Line (Styled with Arrow)
        self.messages.push(format!("{} {}", self.config.settings.prompt_symbol, input_text));

        match ewutsearch_lib::_SEARCH_commandsearch(&input_text) {
            Ok(response) => {
                if response == "__EXIT_SIGNAL__" {
                    self.should_quit = true;
                } else if response == "__CLEAR_SIGNAL__" {
                    self.messages.clear();
                } else {
                    for line in response.lines() {
                        self.messages.push(line.to_string());
                    }
                }
            },
            Err(err_msg) => self.messages.push(format!("❌ {}", err_msg)),
        }

        self.input.clear();
        
        // Auto-scroll logic
        if !self.messages.is_empty() {
            self.list_state.select(Some(self.messages.len() - 1));
        }
    }
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(c) => app.input.push(c),
                        KeyCode::Backspace => { app.input.pop(); },
                        KeyCode::Enter => app.submit_message(),
                        KeyCode::Esc => app.should_quit = true,
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    // 1. Convert Colors from Config
    let bg_color = ewutrm_lib::to_rgb(app.config.colors.background_color);
    let primary_col = ewutrm_lib::to_rgb(app.config.colors.primary_color);
    let second_col = ewutrm_lib::to_rgb(app.config.colors.secondary_color);
    let text_col = ewutrm_lib::to_rgb(app.config.colors.text_color);
    let input_col = ewutrm_lib::to_rgb(app.config.colors.input_color);

    // 2. Main Background Painting
    let main_block = Block::default().style(Style::default().bg(bg_color));
    f.render_widget(main_block, f.area());

    // 3. Layout Splitting (Header, Body, Input)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Messages (Body)
            Constraint::Length(3), // Input
        ])
        .split(f.area());

    // --- HEADER SECTION ---
    let header_text = format!(" {} | SYSTEM: ONLINE | MODE: RAW", app.config.settings.term_ascii);
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(bg_color).bg(primary_col)) // Inverted Colors
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(primary_col)));
    f.render_widget(header, chunks[0]);

    // --- MESSAGES SECTION ---
    let messages: Vec<ListItem> = app.messages
        .iter()
        .map(|m| {
            // Contextual Coloring based on content
            let style = if m.starts_with("❌") {
                Style::default().fg(ewutrm_lib::to_rgb(app.config.colors.error_color))
            } else if m.starts_with(&app.config.settings.prompt_symbol) {
                Style::default().fg(second_col).bold()
            } else if m.contains("📁") || m.contains("📄") {
                Style::default().fg(text_col)
            } else {
                Style::default().fg(text_col)
            };
            ListItem::new(m.as_str()).style(style)
        })
        .collect();

    let history_block = List::new(messages)
        .block(Block::default()
            .borders(Borders::LEFT | Borders::RIGHT) // Modern look: only side borders
            .border_style(Style::default().fg(primary_col))
            .title(" Activity Log ").title_style(Style::default().fg(second_col)));
    
    f.render_stateful_widget(history_block, chunks[1], &mut app.list_state);

    // --- INPUT SECTION ---
    let input_block = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(input_col))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(primary_col))
            .title(" Input Command "));

    f.render_widget(input_block, chunks[2]);

    // Cursor positioning
    f.set_cursor_position((
        chunks[2].x + app.input.len() as u16 + 1,
        chunks[2].y + 1,
    ));
}