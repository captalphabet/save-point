use std::{
    error::Error,
    fs::File,
    io::{self, Write},
    path::PathBuf,
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

use crate::{get_current_path, SavePoints};

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
        Err(err) => eprintln!("Application error: {:?}", err),
        Ok(comm) if comm == Command::Close => {
            // This should call the SavePoints save memory function but that requires
            // the path to save to at the moment which ideally should be bound to the
            // instance of SavePoints is not atm as such is not passed to the run_tui
            // function. Need a way of getting that path here, adding it as a field to
            // SavePoints may require more bespoke Serde serialisation to ensure the save
            // and load methods still work and dont serialsied uneccessary data?
            save_points.memories = paths.iter().map(PathBuf::from).collect();

            save_points.save_to_self_path()?;
        } // receive Close command, save SavePoint
        Ok(comm) if comm != Command::Close => {
            if let Command::ExitPath(exit_path_string) = comm {
                // dbg!("Creating path file");
                let mut temp_path = save_points.memory_path.parent().map(|path| path.to_path_buf()).unwrap(); // should only fail if
                                                                           // memory path was not
                                                                           // defined is somehow
                                                                           // root
                temp_path.extend(["_path.temp"].iter());
                let mut file =
                    File::create(&temp_path).expect("failed to create temp path file"); // I have decided the best way would be to run a script in the current shell and hopefully the tui works and save the target path to a file
                // THis is then read in the script as a cd target, before removing the temp file
                // Will need to ensure script points to this standardised path, perhaps a method to
                // print the config file into the script?

                let formed_args = exit_path_string.as_str();
                match write!(&mut file, "{}", formed_args) {
                    Ok(_) => (),
                    Err(e) => {eprintln!("Error: {e}")}

                }
            }
        }
        _ => (),
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Command<T> {
    Close,
    ExitPath(T),
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
) -> io::Result<Command<String>> {
    // return the Command to propogate closing functions, save etc
    loop {
        // Render the UI on each iteration.
        terminal.draw(|f| {
            let size = f.size();

            // layout setup
            // Create a vertical layout with two chunks:
            // - The upper chunk for the list (using all available space).
            // - The bottom chunk for the legend (fixed height).
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Min(0),    // List takes up available space.
                        Constraint::Length(3), // Legend occupies a fixed height.
                    ]
                    .as_ref(),
                )
                .split(size);

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
            f.render_stateful_widget(list, chunks[0], &mut state);

            // Legend
            let legend_text = "j: down, k: up, Enter: Select, d: delete, q: quit";
            let legend = Paragraph::new(legend_text)
                .style(Style::default().fg(Color::White))
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Instructions"));
            f.render_widget(legend, chunks[1])
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
                        let new_path = get_current_path()
                            .map(|path_buf| path_buf.to_string_lossy().to_string())
                            .unwrap_or_else(|_e| "./".to_string());
                        if !paths.contains(&new_path) {
                            paths.push(new_path); // Consider using a hashmap or vector backed hashmap
                                                  // if number of paths gets large, checking if an element exists in a vector
                                                  // will be slower O(n) checks
                        }
                    }

                    // Delete a path on a 'd'
                    KeyCode::Char('d') => {
                        // let current_path = get_current_path()
                        //     .map(|path_buf| path_buf.to_string_lossy().to_string())
                        //     .unwrap_or_else(|_e| "./".to_string());
                        // if let Some(pos) = paths.iter().position(|x| *x == current_path) { //
                        // This only deleted when in the selected path?
                        //     paths.remove(pos);
                        // }
                        paths.remove(*selected);
                    }

                    // Go to a path and exit and save on an <Enter> press
                    KeyCode::Enter => {
                        let target_path_string = paths
                            .get(*selected)
                            .expect("Failed to read current path from paths");

                        return Ok(Command::ExitPath(target_path_string.to_string()));
                    }

                    _ => {}
                }
            }
        }
    }
}
