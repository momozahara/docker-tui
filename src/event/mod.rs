use crossterm::event::{KeyCode, KeyEvent};

use crate::{env, App, CurrentBlock, InputMode, UserProfile};

pub fn main(app: &mut App, key: KeyEvent) {
    app.selected_state.set_max(5);
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.window_should_close = true,
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
}

pub fn env(app: &mut App, key: KeyEvent) {
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
                        false,
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
}

pub fn env_edit(app: &mut App, key: KeyEvent) {
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
                    if app.user_profile.profile.is_empty()
                        | app.user_profile.username.is_empty()
                        | app.user_profile.hostname.is_empty()
                        | app.user_profile.path.is_empty()
                    {
                        return;
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
}

pub fn up(app: &mut App, key: KeyEvent) {
    app.selected_state.set_max(app.list_profile.len());
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_block = CurrentBlock::Main;
            app.selected_state.set_current(0);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        _ => (),
    }
}

pub fn down(app: &mut App, key: KeyEvent) {
    app.selected_state.set_max(app.list_profile.len());
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_block = CurrentBlock::Main;
            app.selected_state.set_current(0);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        _ => (),
    }
}

pub fn start(app: &mut App, key: KeyEvent) {
    app.selected_state.set_max(app.list_profile.len());
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_block = CurrentBlock::Main;
            app.selected_state.set_current(0);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        _ => (),
    }
}

pub fn stop(app: &mut App, key: KeyEvent) {
    app.selected_state.set_max(app.list_profile.len());
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.current_block = CurrentBlock::Main;
            app.selected_state.set_current(0);
        }
        KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
        KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
        _ => (),
    }
}
