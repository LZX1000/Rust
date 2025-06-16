mod ant;
mod world;
mod display;
mod constants;

use crossterm::{
    cursor::{MoveTo},
    event::{self, Event, KeyCode},
    execute,
};

use std::{io::{Write, Result}, time::Duration};


fn run_simulation(main_display: &mut display::Display) -> Result<()> {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut world: world::World = world::World::new(&mut rng);
    {
        let stdout = main_display.stdout_mut();

        // // Hide cursor
        execute!(stdout, crossterm::cursor::Hide)?;
    }
    loop {
        {
            let stdout = main_display.stdout_mut();
            // Move cursor to top left without clearing
            execute!(stdout, MoveTo(0, 0))?;

            writeln!(stdout, "Press 'q' to quit.\n")?;
        }

        main_display.display_world(&world)?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    // Show cursor
    {
        let stdout = main_display.stdout_mut();
        execute!(stdout, crossterm::cursor::Show)?;
    }

    Ok(())
}


fn main() {
    if let Err(e) = display::Display::new().run_program(run_simulation) {
        eprintln!("Error: {}", e);
    }
}