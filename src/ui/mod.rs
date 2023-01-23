use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{App, InputMode};

static GLOBAL_MARGIN: u16 = 1;

pub fn menu<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Menu");

    let menu_lists = vec!["Env", "Up", "Down", "Start", "Stop"];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = vec!["Up/K Down/J - Navigate", "Enter/E - Select", "Esc/Q - Exit"];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn env<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Env");

    let items: Vec<ListItem> = app
        .list_profile
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = vec![
        "Up/K Down/J - Navigate",
        "Enter/E - Select",
        "Esc/Q - Exit",
        "Backspace/D - Delete",
    ];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn env_edit<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Env");

    let mut menu_lists: Vec<String> = Vec::new();
    menu_lists.push(format!("profile: {}", app.user_profile.profile.clone(),));
    menu_lists.push(format!("username: {}", app.user_profile.username.clone(),));
    menu_lists.push(format!("hostname: {}", app.user_profile.hostname.clone(),));
    menu_lists.push(format!("path: {}", app.user_profile.path.clone(),));
    menu_lists.push(String::from("save"));

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = match app.input_mode {
        InputMode::Normal => vec![
            "Up/K Down/J - Navigate",
            "Enter/E - Select/Edit",
            "Esc/Q - Return",
        ],
        InputMode::Insert => vec!["Backspace - Delete", "Enter/Esc - Return"],
    };

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn up<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Up");

    let items: Vec<ListItem> = app
        .list_profile
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = vec!["Up/K Down/J - Navigate", "Enter/E - Select", "Esc/Q - Exit"];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn up_target<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Up");

    let mut menu_lists: Vec<String> = Vec::new();
    menu_lists.push(format!("target: {}", app.user_profile.target.clone(),));
    menu_lists.push(String::from("Up"));

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = match app.input_mode {
        InputMode::Normal => vec![
            "Leave Empty for All",
            "",
            "Up/K Down/J - Navigate",
            "Enter/E - Select/Edit",
            "Esc/Q - Return",
        ],
        InputMode::Insert => vec!["Backspace - Delete", "Enter/Esc - Return"],
    };

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn down<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Down");

    let items: Vec<ListItem> = app
        .list_profile
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = vec!["Up/K Down/J - Navigate", "Enter/E - Select", "Esc/Q - Exit"];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn down_rmi<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Down");

    let menu_lists = vec!["None", "Local", "All"];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = vec!["Up/K Down/J - Navigate", "Enter/E - Select", "Esc/Q - Exit"];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn start<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Start");

    let items: Vec<ListItem> = app
        .list_profile
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = vec!["Up/K Down/J - Navigate", "Enter/E - Select", "Esc/Q - Exit"];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn start_target<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Start");

    let mut menu_lists: Vec<String> = Vec::new();
    menu_lists.push(format!("target: {}", app.user_profile.target.clone(),));
    menu_lists.push(String::from("Start"));

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = match app.input_mode {
        InputMode::Normal => vec![
            "Leave Empty for All",
            "",
            "Up/K Down/J - Navigate",
            "Enter/E - Select/Edit",
            "Esc/Q - Return",
        ],
        InputMode::Insert => vec!["Backspace - Delete", "Enter/Esc - Return"],
    };

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn stop<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Stop");

    let items: Vec<ListItem> = app
        .list_profile
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = vec!["Up/K Down/J - Navigate", "Enter/E - Select", "Esc/Q - Exit"];

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}

pub fn stop_target<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(tui::layout::Direction::Vertical)
        .margin(GLOBAL_MARGIN)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let block = Block::default().borders(Borders::ALL).title("Stop");

    let mut menu_lists: Vec<String> = Vec::new();
    menu_lists.push(format!("target: {}", app.user_profile.target.clone(),));
    menu_lists.push(String::from("Stop"));

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(match app.input_mode {
            InputMode::Normal => Modifier::REVERSED,
            InputMode::Insert => Modifier::REVERSED | Modifier::UNDERLINED,
        }));

    f.render_stateful_widget(items, chunks[0], &mut app.selected_state.current);

    let block = Block::default().borders(Borders::ALL).title("Help");

    let menu_lists = match app.input_mode {
        InputMode::Normal => vec![
            "Leave Empty for All",
            "",
            "Up/K Down/J - Navigate",
            "Enter/E - Select/Edit",
            "Esc/Q - Return",
        ],
        InputMode::Insert => vec!["Backspace - Delete", "Enter/Esc - Return"],
    };

    let items: Vec<ListItem> = menu_lists
        .iter()
        .map(|item| {
            let span = Span::from(item.to_owned());
            ListItem::new(span).style(Style::default())
        })
        .collect();

    let items = List::new(items).block(block);

    f.render_widget(items, chunks[1]);
}
