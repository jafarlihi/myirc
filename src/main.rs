use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
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
    input: Input
}

impl State {
    fn init() -> State {
        State {
            input: Input::default()
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
        terminal.draw(|f| ui(f, &state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    state.input.reset();
                }
                _ => {
                    input_backend::to_input_request(Event::Key(key)).and_then(|req| state.input.handle(req));
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &State) {
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

    let channels_block = Block::default().title("Channels").borders(Borders::ALL);
    f.render_widget(channels_block, chunks_h[1]);

    let logs_block = Block::default().title("Logs").borders(Borders::ALL);
    f.render_widget(logs_block, chunks[0]);

    let input = Paragraph::new(state.input.value()).scroll((0, scroll)).block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
}

