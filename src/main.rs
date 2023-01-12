mod env;
mod ui;

use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::ListState,
    Frame, Terminal,
};

pub enum CurrentBlock {
    Main,
    Env,
    Up,
    Down,
    Start,
    Stop,
    EnvEdit,
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
            _ => unreachable!(),
        }
    }
}

pub enum InputMode {
    Normal,
    Insert,
}

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

pub struct UserProfile {
    profile: String,
    username: String,
    hostname: String,
    path: String,
    rmi: bool,
    target: String,
}

impl Default for UserProfile {
    fn default() -> UserProfile {
        UserProfile {
            profile: String::new(),
            username: String::new(),
            hostname: String::new(),
            path: String::new(),
            rmi: false,
            target: String::new(),
        }
    }
}

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
    let res = run_app(&mut terminal, app);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();

    if let Err(err) = res {
        println!("{:?}", err);
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    app.selected_state.current.select(Some(0));
    loop {
        terminal.draw(|f| ui(f, &mut app)).unwrap();

        if let Ok(Event::Key(key)) = crossterm::event::read() {
            match app.current_block {
                CurrentBlock::Main => {
                    app.selected_state.set_max(5);
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
                        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
                        KeyCode::Enter | KeyCode::Char('e') => {
                            app.user_profile = UserProfile::default();
                            app.list_profile = env::load_name();
                            app.list_profile.insert(0, "<new>".into());
                            app.current_block = CurrentBlock::from_usize(
                                app.selected_state.current.selected().unwrap() + 1,
                            );
                            app.selected_state.set_current(0);
                        }
                        _ => (),
                    }
                }
                CurrentBlock::Env => {
                    app.selected_state.set_max(app.list_profile.len());
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            app.current_block = CurrentBlock::Main;
                            app.selected_state.set_current(0);
                        }
                        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
                        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
                        KeyCode::Enter | KeyCode::Char('e') => {
                            match app.selected_state.current.selected().unwrap() {
                                0 => {
                                    app.current_block = CurrentBlock::EnvEdit;
                                    app.selected_state.set_current(0);
                                }
                                u => {
                                    let s = app.list_profile[u].clone();
                                    let split = s.as_str().split('.');
                                    let collect: Vec<&str> = split.collect();
                                    let profile = env::load(String::from(collect[0]));
                                    app.user_profile.profile = String::from(collect[0]);
                                    app.user_profile.username = profile[0].clone();
                                    app.user_profile.hostname = profile[1].clone();
                                    app.user_profile.path = profile[2].clone();
                                    app.current_block = CurrentBlock::EnvEdit;
                                    app.selected_state.set_current(0);
                                }
                            }
                        }
                        KeyCode::Backspace | KeyCode::Char('d') => {
                            match app.selected_state.current.selected().unwrap() {
                                0 => (),
                                u => {
                                    let s = app.list_profile[u].clone();
                                    let split = s.as_str().split('.');
                                    let collect: Vec<&str> = split.collect();
                                    env::remove(String::from(collect[0]));
                                    app.list_profile.remove(u);
                                }
                            }
                        }
                        _ => (),
                    }
                }
                CurrentBlock::EnvEdit => {
                    app.selected_state.set_max(5);
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                app.user_profile = UserProfile::default();
                                app.list_profile = env::load_name();
                                app.list_profile.insert(0, "<new>".into());
                                app.current_block = CurrentBlock::Env;
                                app.selected_state.set_current(0);
                            }
                            KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
                            KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
                            KeyCode::Enter | KeyCode::Char('e') => {
                                if app.selected_state.current.selected().unwrap() < 5 - 1 {
                                    app.input_mode = InputMode::Insert;
                                } else {
                                    env::create(
                                        app.user_profile.profile.clone(),
                                        app.user_profile.username.clone(),
                                        app.user_profile.hostname.clone(),
                                        app.user_profile.path.clone(),
                                    );
                                    app.user_profile = UserProfile::default();
                                    app.list_profile = env::load_name();
                                    app.list_profile.insert(0, "<new>".into());
                                    app.current_block = CurrentBlock::Env;
                                    app.selected_state.set_current(0);
                                }
                            }
                            _ => (),
                        },
                        InputMode::Insert => {
                            let selected = app.selected_state.current.selected().unwrap();
                            match selected {
                                0..=3 => match key.code {
                                    KeyCode::Enter | KeyCode::Esc => {
                                        app.input_mode = InputMode::Normal
                                    }
                                    KeyCode::Char(c) => match selected {
                                        0 => {
                                            app.user_profile.profile.push(c);
                                        }
                                        1 => {
                                            app.user_profile.username.push(c);
                                        }
                                        2 => {
                                            app.user_profile.hostname.push(c);
                                        }
                                        3 => {
                                            app.user_profile.path.push(c);
                                        }
                                        _ => unreachable!(),
                                    },
                                    KeyCode::Backspace => match selected {
                                        0 => {
                                            app.user_profile.profile.pop();
                                        }
                                        1 => {
                                            app.user_profile.username.pop();
                                        }
                                        2 => {
                                            app.user_profile.hostname.pop();
                                        }
                                        3 => {
                                            app.user_profile.path.pop();
                                        }
                                        _ => unreachable!(),
                                    },
                                    _ => (),
                                },
                                _ => unreachable!(),
                            }
                        }
                    }
                }
                CurrentBlock::Up => {
                    app.selected_state.set_max(0);
                    match key.code {
                        KeyCode::Esc => {
                            app.current_block = CurrentBlock::Main;
                            app.selected_state.set_current(1);
                        }
                        _ => (),
                    }
                }
                CurrentBlock::Down => {
                    app.selected_state.set_max(0);
                    match key.code {
                        KeyCode::Esc => {
                            app.current_block = CurrentBlock::Main;
                            app.selected_state.set_current(2);
                        }
                        _ => (),
                    }
                }
                CurrentBlock::Start => {
                    app.selected_state.set_max(0);
                    match key.code {
                        KeyCode::Esc => {
                            app.current_block = CurrentBlock::Main;
                            app.selected_state.set_current(3);
                        }
                        _ => (),
                    }
                }
                CurrentBlock::Stop => {
                    app.selected_state.set_max(0);
                    match key.code {
                        KeyCode::Esc => {
                            app.current_block = CurrentBlock::Main;
                            app.selected_state.set_current(4);
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.current_block {
        CurrentBlock::Main => ui::menu(f, app),
        CurrentBlock::Env => ui::env(f, app),
        CurrentBlock::Up => ui::up(f, app),
        CurrentBlock::Down => ui::down(f, app),
        CurrentBlock::Start => ui::start(f, app),
        CurrentBlock::Stop => ui::stop(f, app),
        CurrentBlock::EnvEdit => ui::env_edit(f, app),
    };
}
