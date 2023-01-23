use std::io;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{cli, env, App, CurrentBlock, InputMode, UserProfile};

pub fn main(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(5);
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => return Some(Ok(())),
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        KeyCode::Enter | KeyCode::Char('e') => {
            app.user_profile = UserProfile::default();
            app.list_profile = env::load_name();
            if app.selected_state.current.selected() == Some(0) {
                app.list_profile.insert(0, "<new>".into());
            }
            app.current_block =
                CurrentBlock::from_usize(app.selected_state.current.selected().unwrap() + 1);
            app.selected_state.set_current(0);
        }
        _ => (),
    }
    return None;
}

pub fn env(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
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
                    let profile = env::load(app.list_profile[u].clone());
                    app.user_profile.set(
                        app.list_profile[u].clone(),
                        profile[0].clone(),
                        profile[1].clone(),
                        profile[2].clone(),
                        String::new(),
                        String::new(),
                    );
                    app.current_block = CurrentBlock::EnvEdit;
                    app.selected_state.set_current(0);
                }
            }
        }
        KeyCode::Backspace | KeyCode::Char('d') => {
            match app.selected_state.current.selected().unwrap() {
                0 => (),
                u => {
                    env::remove(String::from(app.list_profile[u].clone()));
                    app.list_profile.remove(u);
                }
            }
        }
        _ => (),
    }
    return None;
}

pub fn env_edit(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
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
                match app.selected_state.current.selected().unwrap() {
                    0..=3 => app.input_mode = InputMode::Insert,
                    _ => {
                        if app.user_profile.profile.is_empty()
                            | app.user_profile.username.is_empty()
                            | app.user_profile.hostname.is_empty()
                            | app.user_profile.path.is_empty()
                        {
                            return None;
                        }
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
            }
            _ => (),
        },
        InputMode::Insert => {
            let selected = app.selected_state.current.selected().unwrap();
            match selected {
                0..=3 => match key.code {
                    KeyCode::Enter | KeyCode::Esc => app.input_mode = InputMode::Normal,
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
    return None;
}

pub fn up(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(app.list_profile.len());
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_block = CurrentBlock::Main;
            app.selected_state.set_current(1);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        KeyCode::Enter | KeyCode::Char('e') => {
            match app.selected_state.current.selected().unwrap() {
                u => {
                    let profile = env::load(app.list_profile[u].clone());
                    app.user_profile.set(
                        app.list_profile[u].clone(),
                        profile[0].clone(),
                        profile[1].clone(),
                        profile[2].clone(),
                        String::new(),
                        String::new(),
                    );
                    app.current_block = CurrentBlock::UpTarget;
                    app.selected_state.set_current(0);
                }
            }
        }
        _ => (),
    }
    return None;
}

pub fn up_target(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(2);
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.user_profile = UserProfile::default();
                app.list_profile = env::load_name();
                app.current_block = CurrentBlock::Up;
                app.selected_state.set_current(0);
            }
            KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
            KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
            KeyCode::Enter | KeyCode::Char('e') => {
                match app.selected_state.current.selected().unwrap() {
                    0 => app.input_mode = InputMode::Insert,
                    1 => {
                        app.output = Some(cli::up(
                            app.user_profile.username.clone(),
                            app.user_profile.hostname.clone(),
                            app.user_profile.path.clone(),
                            Some(app.user_profile.target.clone()),
                        ));
                        return Some(Ok(()));
                    }
                    _ => unreachable!(),
                }
            }
            _ => (),
        },
        InputMode::Insert => {
            let selected = app.selected_state.current.selected().unwrap();
            match selected {
                0 => match key.code {
                    KeyCode::Enter | KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Char(c) => match selected {
                        0 => {
                            app.user_profile.target.push(c);
                        }
                        _ => unreachable!(),
                    },
                    KeyCode::Backspace => match selected {
                        0 => {
                            app.user_profile.target.pop();
                        }
                        _ => unreachable!(),
                    },
                    _ => (),
                },
                _ => unreachable!(),
            }
        }
    }
    return None;
}

pub fn down(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(app.list_profile.len());
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_block = CurrentBlock::Main;
            app.selected_state.set_current(2);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        KeyCode::Enter | KeyCode::Char('e') => {
            match app.selected_state.current.selected().unwrap() {
                u => {
                    let profile = env::load(app.list_profile[u].clone());
                    app.user_profile.set(
                        app.list_profile[u].clone(),
                        profile[0].clone(),
                        profile[1].clone(),
                        profile[2].clone(),
                        String::new(),
                        String::new(),
                    );
                    app.current_block = CurrentBlock::DownRmi;
                    app.selected_state.set_current(0);
                }
            }
        }
        _ => (),
    }
    return None;
}

pub fn down_rmi(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(3);
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.user_profile = UserProfile::default();
            app.list_profile = env::load_name();
            app.current_block = CurrentBlock::Down;
            app.selected_state.set_current(0);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        KeyCode::Enter | KeyCode::Char('e') => {
            match app.selected_state.current.selected().unwrap() {
                0 => {
                    app.output = Some(cli::down(
                        app.user_profile.username.clone(),
                        app.user_profile.hostname.clone(),
                        app.user_profile.path.clone(),
                        None,
                    ));
                    return Some(Ok(()));
                }
                1 => {
                    app.user_profile.set_rmi(String::from("local"));
                    app.output = Some(cli::down(
                        app.user_profile.username.clone(),
                        app.user_profile.hostname.clone(),
                        app.user_profile.path.clone(),
                        Some(app.user_profile.rmi.clone()),
                    ));
                    return Some(Ok(()));
                }
                2 => {
                    app.user_profile.set_rmi(String::from("all"));
                    app.output = Some(cli::down(
                        app.user_profile.username.clone(),
                        app.user_profile.hostname.clone(),
                        app.user_profile.path.clone(),
                        Some(app.user_profile.rmi.clone()),
                    ));
                    return Some(Ok(()));
                }
                _ => unreachable!(),
            }
        }
        _ => (),
    }
    return None;
}

pub fn start(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(app.list_profile.len());
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_block = CurrentBlock::Main;
            app.selected_state.set_current(3);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        KeyCode::Enter | KeyCode::Char('e') => {
            match app.selected_state.current.selected().unwrap() {
                u => {
                    let profile = env::load(app.list_profile[u].clone());
                    app.user_profile.set(
                        app.list_profile[u].clone(),
                        profile[0].clone(),
                        profile[1].clone(),
                        profile[2].clone(),
                        String::new(),
                        String::new(),
                    );
                    app.current_block = CurrentBlock::StartTarget;
                    app.selected_state.set_current(0);
                }
            }
        }
        _ => (),
    }
    return None;
}

pub fn start_target(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(2);
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.user_profile = UserProfile::default();
                app.list_profile = env::load_name();
                app.current_block = CurrentBlock::Start;
                app.selected_state.set_current(0);
            }
            KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
            KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
            KeyCode::Enter | KeyCode::Char('e') => {
                match app.selected_state.current.selected().unwrap() {
                    0 => app.input_mode = InputMode::Insert,
                    1 => {
                        app.output = Some(cli::start(
                            app.user_profile.username.clone(),
                            app.user_profile.hostname.clone(),
                            app.user_profile.path.clone(),
                            Some(app.user_profile.target.clone()),
                        ));
                        return Some(Ok(()));
                    }
                    _ => unreachable!(),
                }
            }
            _ => (),
        },
        InputMode::Insert => {
            let selected = app.selected_state.current.selected().unwrap();
            match selected {
                0 => match key.code {
                    KeyCode::Enter | KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Char(c) => match selected {
                        0 => {
                            app.user_profile.target.push(c);
                        }
                        _ => unreachable!(),
                    },
                    KeyCode::Backspace => match selected {
                        0 => {
                            app.user_profile.target.pop();
                        }
                        _ => unreachable!(),
                    },
                    _ => (),
                },
                _ => unreachable!(),
            }
        }
    }
    return None;
}

pub fn stop(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(app.list_profile.len());
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_block = CurrentBlock::Main;
            app.selected_state.set_current(4);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        KeyCode::Enter | KeyCode::Char('e') => {
            match app.selected_state.current.selected().unwrap() {
                u => {
                    let profile = env::load(app.list_profile[u].clone());
                    app.user_profile.set(
                        app.list_profile[u].clone(),
                        profile[0].clone(),
                        profile[1].clone(),
                        profile[2].clone(),
                        String::new(),
                        String::new(),
                    );
                    app.current_block = CurrentBlock::StopTarget;
                    app.selected_state.set_current(0);
                }
            }
        }
        _ => (),
    }
    return None;
}

pub fn stop_target(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(2);
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.user_profile = UserProfile::default();
                app.list_profile = env::load_name();
                app.current_block = CurrentBlock::Stop;
                app.selected_state.set_current(0);
            }
            KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
            KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
            KeyCode::Enter | KeyCode::Char('e') => {
                match app.selected_state.current.selected().unwrap() {
                    0 => app.input_mode = InputMode::Insert,
                    1 => {
                        app.output = Some(cli::stop(
                            app.user_profile.username.clone(),
                            app.user_profile.hostname.clone(),
                            app.user_profile.path.clone(),
                            Some(app.user_profile.target.clone()),
                        ));
                        return Some(Ok(()));
                    }
                    _ => unreachable!(),
                }
            }
            _ => (),
        },
        InputMode::Insert => {
            let selected = app.selected_state.current.selected().unwrap();
            match selected {
                0 => match key.code {
                    KeyCode::Enter | KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Char(c) => match selected {
                        0 => {
                            app.user_profile.target.push(c);
                        }
                        _ => unreachable!(),
                    },
                    KeyCode::Backspace => match selected {
                        0 => {
                            app.user_profile.target.pop();
                        }
                        _ => unreachable!(),
                    },
                    _ => (),
                },
                _ => unreachable!(),
            }
        }
    }
    return None;
}
