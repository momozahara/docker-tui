mod env;
mod event;
mod ui;

use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::ListState,
    Frame, Terminal,
};

#[derive(Debug)]
pub enum CurrentBlock {
    Main,
    Env,
    Up,
    Down,
    Start,
    Stop,
    EnvEdit,
    UpTarget,
    DownRmi,
    StartTarget,
    StopTarget,
}

impl CurrentBlock {
    fn from_usize(u: usize) -> CurrentBlock {
        match u {
            0 => CurrentBlock::Main,
            1 => CurrentBlock::Env,
            2 => CurrentBlock::Up,
            3 => CurrentBlock::Down,
            4 => CurrentBlock::Start,
            5 => CurrentBlock::Stop,
            6 => CurrentBlock::EnvEdit,
            7 => CurrentBlock::UpTarget,
            8 => CurrentBlock::DownRmi,
            9 => CurrentBlock::StartTarget,
            10 => CurrentBlock::StopTarget,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Insert,
}

#[derive(Debug)]
pub struct App {
    selected_state: SelectedState,
    current_block: CurrentBlock,
    user_profile: UserProfile,
    list_profile: Vec<String>,
    input_mode: InputMode,
}

impl Default for App {
    fn default() -> App {
        App {
            selected_state: SelectedState::default(),
            current_block: CurrentBlock::Main,
            user_profile: UserProfile::default(),
            list_profile: Vec::new(),
            input_mode: InputMode::Normal,
        }
    }
}

#[derive(Debug)]
pub struct UserProfile {
    profile: String,
    username: String,
    hostname: String,
    path: String,
    rmi: String,
    target: String,
}

impl UserProfile {
    fn set(
        &mut self,
        profile: String,
        username: String,
        hostname: String,
        path: String,
        rmi: String,
        target: String,
    ) {
        self.profile = profile;
        self.username = username;
        self.hostname = hostname;
        self.path = path;
        self.rmi = rmi;
        self.target = target;
    }
    fn set_rmi(&mut self, rmi: String) {
        self.rmi = rmi;
    }
}

impl Default for UserProfile {
    fn default() -> UserProfile {
        UserProfile {
            profile: String::new(),
            username: String::new(),
            hostname: String::new(),
            path: String::new(),
            rmi: String::new(),
            target: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct SelectedState {
    max: usize,
    current: ListState,
}

impl SelectedState {
    fn set_max(&mut self, max: usize) {
        self.max = max;
    }

    fn set_current(&mut self, current: usize) {
        self.current.select(Some(current));
    }

    fn next(&mut self) {
        let i = match self.current.selected() {
            Some(i) => {
                if i >= self.max - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.current.select(Some(i));
    }

    fn prev(&mut self) {
        let i = match self.current.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.current.select(Some(i));
    }
}

impl Default for SelectedState {
    fn default() -> SelectedState {
        SelectedState {
            max: 0,
            current: ListState::default(),
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let app = App::default();
    run_app(&mut terminal, app).unwrap();

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    app.selected_state.current.select(Some(0));
    loop {
        terminal.draw(|f| ui(f, &mut app)).unwrap();

        if let Ok(Event::Key(key)) = crossterm::event::read() {
            let result = match app.current_block {
                CurrentBlock::Main => event::main(&mut app, key),
                CurrentBlock::Env => event::env(&mut app, key),
                CurrentBlock::EnvEdit => event::env_edit(&mut app, key),
                CurrentBlock::Up => event::up(&mut app, key),
                CurrentBlock::UpTarget => event::up_target(&mut app, key),
                CurrentBlock::Down => event::down(&mut app, key),
                CurrentBlock::DownRmi => event::down_rmi(&mut app, key),
                CurrentBlock::Start => event::start(&mut app, key),
                CurrentBlock::StartTarget => event::start_target(&mut app, key),
                CurrentBlock::Stop => event::stop(&mut app, key),
                CurrentBlock::StopTarget => event::stop_target(&mut app, key),
            };
            match result {
                Some(r) => return r,
                None => (),
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.current_block {
        CurrentBlock::Main => ui::menu(f, app),
        CurrentBlock::Env => ui::env(f, app),
        CurrentBlock::EnvEdit => ui::env_edit(f, app),
        CurrentBlock::Up => ui::up(f, app),
        CurrentBlock::UpTarget => ui::up_target(f, app),
        CurrentBlock::Down => ui::down(f, app),
        CurrentBlock::DownRmi => ui::down_rmi(f, app),
        CurrentBlock::Start => ui::start(f, app),
        CurrentBlock::StartTarget => ui::start_target(f, app),
        CurrentBlock::Stop => ui::stop(f, app),
        CurrentBlock::StopTarget => ui::stop_target(f, app),
    };
}
