use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

use crate::SavePoints;

pub fn run_tui(save_points: &mut SavePoints) -> Result<(), Box<dyn Error>> {
    // Set up the terminal in raw mode to capture keystrokes directly.
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    // Enter the alternate screen and enable mouse capture.
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Application state: a vector of path strings and a selected index.
    let mut paths: Vec<_> = save_points
        .memories
        .iter()
        .flat_map(|path| path.to_str())
        .map(|p| p.to_owned())
        .collect();
    let mut selected: usize = 0;

    // Run the main application loop.
    let app_result = run_app(&mut terminal, &mut paths, &mut selected);

    // Restore the terminal to its previous state.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match app_result {

       Err(err)  => eprintln!("Application error: {:?}", err),
       Ok(comm) if comm == Command::Close => {
            // This should call the SavePoints save memory function but that requires
                    // the path to save to at the moment which ideally should be bound to the
                    // instance of SavePoints is not atm as such is not passed to the run_tui
                    // function. Need a way of getting that path here, adding it as a field to
                    // SavePoints may require more bespoke Serde serialisation to ensure the save
                    // and load methods still work and dont serialsied uneccessary data?

            save_points.save_to_self_path()?;
           

       }, // receive Close command, save SavePoint
        _ => (),
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Close,
}

/// Runs the application event loop.
///
/// * `terminal` - a mutable reference to the Terminal.
/// * `paths` - a mutable reference to the vector of paths.
/// * `selected` - a mutable reference to the index of the currently selected path.
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    paths: &mut Vec<String>,
    selected: &mut usize,
) -> io::Result<Command> { // return the Command to propogate closing functions, save etc
    loop {
        // Render the UI on each iteration.
        terminal.draw(|f| {
            let size = f.size();

            // Create a list of ListItem widgets from the vector of paths.
            let items: Vec<ListItem> = paths.iter().map(|p| ListItem::new(p.to_string())).collect();

            // Set up the list widget with a border, title, and a highlight symbol.
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Paths"))
                .highlight_symbol(">> ")
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );

            // Maintain the state (selected index) of the list.
            let mut state = ListState::default();
            state.select(Some(*selected));

            // Render the stateful list widget.
            f.render_stateful_widget(list, size, &mut state);
        })?;

        // Poll for events (with a timeout to allow UI updates).
        if event::poll(Duration::from_millis(100))? {
            // Read the event.
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // Quit on 'q'
                    KeyCode::Char('q') => return Ok(Command::Close),

                    // Navigate down on 'j'
                    KeyCode::Char('j') => {
                        if *selected < paths.len().saturating_sub(1) {
                            *selected += 1;
                        }
                    }

                    // Navigate up on 'k'
                    KeyCode::Char('k') => {
                        if *selected > 0 {
                            *selected -= 1;
                        }
                    }

                    // Append a new path on 'a'
                    KeyCode::Char('a') => {
                        let new_path = format!("/path/{}", paths.len() + 1);
                        paths.push(new_path);
                    }

                    _ => {}
                }
            }
        }
    }
}
