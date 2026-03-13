mod app;
mod converter;
mod filesystem;
mod ui;

use app::{App, Screen};
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> Result<()> {
    color_eyre::install()?;

    // Create working directories
    let (icns_dir, ico_dir) = filesystem::create_directories()?;

    // Open the input folder immediately for convenience and track the handle
    let initial_explorer = filesystem::open_folder(&icns_dir);

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(icns_dir, ico_dir);
    if let Some(child) = initial_explorer {
        app.explorer_children.push(child);
    }
    let result = run_app(&mut terminal, &mut app);

    // Kill all explorer windows opened by this app
    for child in &mut app.explorer_children {
        let _ = child.kill();
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events (not release on Windows)
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match app.screen {
                    Screen::WaitingForInput => match key.code {
                        KeyCode::Enter => {
                            app.screen = Screen::Converting;
                            // Draw the "converting" screen once before blocking
                            terminal.draw(|f| ui::draw(f, app))?;

                            // Run the conversion (blocks briefly; files convert in parallel)
                            let results = converter::convert_all(&app.icns_dir, &app.ico_dir);
                            for r in results {
                                app.add_result(r.success, r.message);
                            }

                            app.screen = Screen::Done;
                            // Open output folder if anything succeeded, track handle
                            if app.converted > 0 {
                                if let Some(child) = filesystem::open_folder(&app.ico_dir) {
                                    app.explorer_children.push(child);
                                }
                            }
                        }
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.should_quit = true;
                        }
                        _ => {}
                    },
                    Screen::Done => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
                            app.should_quit = true;
                        }
                        _ => {}
                    },
                    Screen::Converting => {}
                }
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
