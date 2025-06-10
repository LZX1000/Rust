const SIMULATION_HEIGHT: usize = 16;
const SIMULATION_WIDTH: usize = 16;
const GROUND_HEIGHT : usize = 10;
const STARTING_ANT_COUNT: u8 = 5;

use crossterm::{
    cursor::{MoveTo},
    event::{self, Event, KeyCode},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode},
    execute,
};
use std::{io::{stdout, Write, Result}, time::Duration};


#[derive(Debug, Clone)]
pub struct Tile(u8);

impl Tile {
    // Bit masks
    const HAS_ANT: u8 = 0b1000_0000;
    const HAS_FOOD: u8 = 0b0100_0000;
    const IS_OBSTACLE: u8 = 0b0010_0000;
    const PHEROMONE_LEVEL_MASK: u8 = 0b0001_1111;

    pub fn new() -> Self {
        Tile(0)
    }

    // Getters
    pub fn has_ant(&self) -> bool {
        self.0 & Self::HAS_ANT != 0
    }
    pub fn has_food(&self) -> bool {
        self.0 & Self::HAS_FOOD != 0
    }
    pub fn is_obstacle(&self) -> bool {
        self.0 & Self::IS_OBSTACLE != 0
    }
    pub fn pheromone(&self) -> u8 {
        self.0 & Self::PHEROMONE_LEVEL_MASK
    }

    // Setters
    pub fn set_ant(&mut self, value: bool) {
        if value {
            self.0 |= Self::HAS_ANT;
        } else {
            self.0 &= !Self::HAS_ANT;
        }
    }
    pub fn set_food(&mut self, value: bool) {
        if value {
            self.0 |= Self::HAS_FOOD;
        } else {
            self.0 &= !Self::HAS_FOOD;
        }
    }
    pub fn set_obstacle(&mut self, value: bool) {
        if value {
            self.0 |= Self::IS_OBSTACLE;
        } else {
            self.0 &= !Self::IS_OBSTACLE;
        }
    }
    pub fn set_pheromone(&mut self, level: u8) {
        self.0 = (self.0 & !Self::PHEROMONE_LEVEL_MASK) | (level.min(31) & Self::PHEROMONE_LEVEL_MASK);
    }
}


pub struct Ant(u16);

impl Ant {
    //Bit masks
    const MAX_HEALTH_MASK: u16 = 0b1100_0000_0000_0000;
    const CURRENT_HEALTH_MASK: u16 = 0b0011_0000_0000_0000;
    const STRENGTH_MASK: u16 = 0b0000_1100_0000_0000;
    const DIRECTION_MASK: u16 = 0b0000_0000_1100_0000;
    const SEES_OBSTACLE: u16 = 0b0000_0000_0010_0000;
    const SEES_FOOD: u16 = 0b0000_0000_0001_0000;
    const IS_LOST: u16 = 0b0000_0000_0000_0100;
    const IS_DEAD: u16 = 0b0000_0000_0000_0010;
    const IS_CARRYING_FOOD: u16 = 0b0000_0000_0000_0001;

    pub fn new() -> Self {
        Ant(0)
    }

    // Getters
    pub fn max_health(&self) -> u16 {
        (self.0 & Self::MAX_HEALTH_MASK) >> 14
    }
    pub fn current_health(&self) -> u16 {
        (self.0 & Self::CURRENT_HEALTH_MASK) >> 12
    }
    pub fn strength(&self) -> u16 {
        (self.0 & Self::STRENGTH_MASK) >> 10
    }
    pub fn direction(&self) -> u16 {
        (self.0 & Self::DIRECTION_MASK) >> 6
    }
    pub fn sees_obstacle(&self) -> bool {
        self.0 & Self::SEES_OBSTACLE != 0
    }
    pub fn sees_food(&self) -> bool {
        self.0 & Self::SEES_FOOD != 0
    }
    pub fn is_lost(&self) -> bool {
        self.0 & Self::IS_LOST != 0
    }
    pub fn is_dead(&self) -> bool {
        self.0 & Self::IS_DEAD != 0
    }
    pub fn is_carrying_food(&self) -> bool {
        self.0 & Self::IS_CARRYING_FOOD != 0
    }

    // Setters
    pub fn set_max_health(&mut self, health: u16) {
        self.0 = (self.0 & !Self::MAX_HEALTH_MASK) | ((health & 0b11) << 14);
    }
    pub fn set_current_health(&mut self, health: u16) {
        self.0 = (self.0 & !Self::CURRENT_HEALTH_MASK) | ((health & 0b11) << 12);
    }
    pub fn set_strength(&mut self, strength: u16) {
        self.0 = (self.0 & !Self::STRENGTH_MASK) | ((strength & 0b11) << 10);
    }
    pub fn set_direction(&mut self, direction: u16) {
        self.0 = (self.0 & !Self::DIRECTION_MASK) | ((direction & 0b11) << 6);
    }
    pub fn set_sees_obstacle(&mut self, value: bool) {
        if value {
            self.0 |= Self::SEES_OBSTACLE;
        } else {
            self.0 &= !Self::SEES_OBSTACLE;
        }
    }
    pub fn set_sees_food(&mut self, value: bool) {
        if value {
            self.0 |= Self::SEES_FOOD;
        } else {
            self.0 &= !Self::SEES_FOOD;
        }
    }
    pub fn set_is_lost(&mut self, value: bool) {
        if value {
            self.0 |= Self::IS_LOST;
        } else {
            self.0 &= !Self::IS_LOST;
        }
    }
    pub fn set_dead(&mut self, value: bool) {
        if value {
            self.0 |= Self::IS_DEAD;
        } else {
            self.0 &= !Self::IS_DEAD;
        }
    }
    pub fn set_carrying_food(&mut self, value: bool) {
        if value {
            self.0 |= Self::IS_CARRYING_FOOD;
        } else {
            self.0 &= !Self::IS_CARRYING_FOOD;
        }
    }
}


fn clear_screen() {
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        MoveTo(0, 0)
    ).unwrap();
}


fn display_tile(tile: &Tile) -> char {
    if tile.is_obstacle() {
        '#'
    } else if tile.has_ant() {
        'X'
    } else if tile.has_food() {
        'O'
    } else {
        '.'
    }
}


fn run_simulation() -> Result<()> {
    let mut grid = vec![vec![Tile::new(); SIMULATION_WIDTH]; SIMULATION_HEIGHT];

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            if y >= SIMULATION_HEIGHT - GROUND_HEIGHT {
                tile.set_obstacle(true);
            } else if y == SIMULATION_HEIGHT - GROUND_HEIGHT - 1 && x == SIMULATION_WIDTH / 2 - 1 {
                tile.set_ant(true);
            } else if (x + y) % 3 == 0 {
                tile.set_food(true);
            }
        }
    }

    loop {
        clear_screen();
        println!("Use WASD to move. Press 'q' to quit.\n");

        for row in &grid {
            let row_display: String = row.iter().map(display_tile).collect();
            println!("{}", row_display);
        }
        stdout().flush().unwrap();

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => println!("Move Up"),
                    KeyCode::Char('s') => println!("Move Down"),
                    KeyCode::Char('a') => println!("Move Left"),
                    KeyCode::Char('d') => println!("Move Right"),
                    _ => {}
                }
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}


fn main() {
    let mut stdout = stdout();

    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let result = run_simulation();

    execute!(stdout, LeaveAlternateScreen).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();

    if let Err(err) = result {
        eprintln!("Simulation error: {}", err);
    }
}