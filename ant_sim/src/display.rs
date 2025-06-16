use crate::world;
use crate::constants;

use std::io::Write;

use crossterm::{
    cursor::{MoveTo},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};


pub struct Display {
    stdout: std::io::Stdout,
}

impl Display {
    pub fn new() -> Self { 
        Display { stdout: std::io::stdout() }
    }

    pub fn run_program(&mut self, function: fn(&mut Display) -> Result<(), std::io::Error>) -> Result<(), std::io::Error> {
        crossterm::terminal::enable_raw_mode().unwrap();
        execute!(self.stdout, EnterAlternateScreen).unwrap();

        let result: Result<(), _> = function(self);

        execute!(self.stdout, LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();

        self.clear_screen()?;
        println!("Goodbye.");

        if let Err(err) = result {
            eprintln!("Simulation error: {}", err);
        }
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        execute!(
            self.stdout,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;
        Ok(())
    }

    pub fn display_tile(tile: &world::Tile) -> char {
        match tile.object() {
            world::Objects::Obstacle => '#',
            world::Objects::Ant => 'X',
            world::Objects::Food => 'O',
            _ => '.',
        }
    }

    pub fn display_world(&mut self, world: &world::World) -> Result<(), std::io::Error> {
        for row in world.grid.chunks(constants::SIMULATION_WIDTH) {
            for tile in row {
                let ch = Self::display_tile(tile);
                write!(self.stdout, "{}", ch)?;
            }
            writeln!(self.stdout)?;
        }

        self.stdout.flush()?;
        Ok(())
    }

    // Getter
    pub fn stdout_mut(&mut self) -> &mut std::io::Stdout {
        &mut self.stdout
    }
}