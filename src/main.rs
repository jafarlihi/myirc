use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use tui_input::backend::crossterm as input_backend;
use tui_input::Input;

struct State {
    input: Input,
    log: String
}

impl State {
    fn init() -> State {
        State {
            input: Input::default(),
            log: String::new()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = State::init();
    let res = run_app(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    state.log.push_str(state.input.value());
                    state.log.push_str("\n");
                    state.input.reset();
                }
                _ => {
                    input_backend::to_input_request(Event::Key(key)).and_then(|req| state.input.handle(req));
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let chunks_h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Percentage(90),
            Constraint::Percentage(10),
            ].as_ref()).split(f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Percentage(90),
            Constraint::Percentage(10),
            ].as_ref()).split(chunks_h[0]);

    let width = chunks[0].width.max(3) - 3;
    let scroll = (state.input.cursor() as u16).max(width) - width;

    let height = chunks[0].height.max(3) - 3;
    let scroll_log = (state.log.matches("\n").count() as u16).max(height) - height;

    let channels = Block::default().title("Channels").borders(Borders::ALL);
    f.render_widget(channels, chunks_h[1]);

    let logs = Paragraph::new(state.log.clone()).scroll((scroll_log, 0)).block(Block::default().borders(Borders::ALL));
    f.render_widget(logs.clone(), chunks[0]);

    let input = Paragraph::new(state.input.value()).scroll((0, scroll)).block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);

    f.set_cursor(
        chunks[1].x + (state.input.cursor() as u16).min(width) + 1,
        chunks[1].y + 1,
    );
}

