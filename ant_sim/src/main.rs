const SIMULATION_HEIGHT: usize = 32;
const SIMULATION_WIDTH: usize = 128;
const GROUND_HEIGHT : usize = 10;

const FOOD_SPAWNING_CHANCE_PERCENTAGE: u8 = 1;
const STARTING_ANT_COUNT: u8 = 5;


use crossterm::{
    cursor::{MoveTo},
    event::{self, Event, KeyCode},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use rand::{Rng};
use std::{io::{stdout, Write, Result}, time::Duration};
// use std::thread;


#[derive(Debug, Clone)]
pub struct Tile(u8);

impl Tile {
    // Bit masks
    const HAS_OBJECT_MASK: u8           = 0b1110_0000;
    const FOOD_PHEROMONE_LEVEL_MASK: u8 = 0b0001_1000;
    const HOME_PHEROMONE_LEVEL_MASK: u8 = 0b0000_0111;

    const HAS_ANT: u8      = 1;
    const HAS_FOOD: u8     = 2;
    const HAS_OBSTACLE: u8 = 3;
    // Has up to 7 for flags

    pub fn new(rng: &mut impl rand::Rng, food_chance_percentage: u8) -> Self {
        let mut tile = Tile(0);

        if rng.gen_range(0..100) < food_chance_percentage {
            tile.set_food(true);
        }
        tile
    }

    // Getters
    pub fn has_ant(&self) -> bool {
        (self.0 & Self::HAS_OBJECT_MASK) >> 5 == Self::HAS_ANT
    }
    pub fn has_food(&self) -> bool {
        (self.0 & Self::HAS_OBJECT_MASK) >> 5 == Self::HAS_FOOD
    }
    pub fn is_obstacle(&self) -> bool {
        (self.0 & Self::HAS_OBJECT_MASK) >> 5 == Self::HAS_OBSTACLE
    }
    pub fn pheromone(&self) -> u8 {
        (self.0 & Self::FOOD_PHEROMONE_LEVEL_MASK) >> 3
    }
    pub fn home_pheromone(&self) -> u8 {
        self.0 & Self::HOME_PHEROMONE_LEVEL_MASK
    }

    // Setters
    pub fn set_ant(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::HAS_OBJECT_MASK) | Self::HAS_ANT << 5;
        } else if Self::has_ant(self) {
            self.0 &= !Self::HAS_OBJECT_MASK;
        }
    }
    pub fn set_food(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::HAS_OBJECT_MASK) | Self::HAS_FOOD << 5;
        } else if Self::has_food(self) {
            self.0 &= !Self::HAS_OBJECT_MASK;
        }
    }
    pub fn set_obstacle(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::HAS_OBJECT_MASK) | Self::HAS_OBSTACLE << 5;
        } else if Self::is_obstacle(self) {
            self.0 &= !Self::HAS_OBJECT_MASK;
        }
    }
    pub fn set_pheromone(&mut self, level: u8) {
        self.0 = (self.0 & !Self::FOOD_PHEROMONE_LEVEL_MASK) | ((level.min(3) & 0b11) << 3);
    }
    pub fn set_home_pheromone(&mut self, level: u8) {
        self.0 = (self.0 & !Self::HOME_PHEROMONE_LEVEL_MASK) | (level.min(7) & 0b111);
    }
}


enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn to_u16(self) -> u16 {
        self as u16
    }

    pub fn from_u16(value: u16) -> Direction {
        match value & 0b111{
            0 => Direction::Up,
            1 => Direction::UpRight,
            2 => Direction::Right,
            3 => Direction::DownRight,
            4 => Direction::Down,
            5 => Direction::DownLeft,
            6 => Direction::Left,
            7 => Direction::UpLeft,
            _ => unreachable!(), 
        }
    }

    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::UpRight => (1, -1),
            Direction::Right => (1, 0),
            Direction::DownRight => (1, 1,),
            Direction::Down => (0, 1),
            Direction::DownLeft => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::UpLeft => (-1, -1),
        }
    }

    pub fn turn_left(self) -> Self {
        Direction::from_u16((self.to_u16() + 7) % 8)
    }

    pub fn turn_right(self) -> Self {
        Direction::from_u16((self.to_u16() + 1) % 8)
    }
}


#[derive(Debug, Clone)]
pub struct Ant(u16);

impl Ant {
    //Bit masks
    const MAX_HEALTH_MASK: u16     = 0b1100_0000_0000_0000;
    const CURRENT_HEALTH_MASK: u16 = 0b0011_0000_0000_0000;
    const STRENGTH_MASK: u16       = 0b0000_1100_0000_0000;
    const DIRECTION_MASK: u16      = 0b0000_0011_1000_0000;
    const ANT_ROLE_MASK: u16       = 0b0000_0000_0110_0000;
    const CARRYING_MASK: u16       = 0b0000_0000_0001_1000;
    const UNIQUE_FLAGS_MASK: u16   = 0b0000_0000_0000_0111;

    // Ant roles, 4 available (bits 6 and 7)
    const WORKER: u16  = 0;
    const SOLDIER: u16 = 1;
    const SCOUT: u16   = 2;
    const QUEEN: u16   = 3;
    // Carrying flags, 3 available (bits 3 and 4)
    const IS_CARRYING_FOOD: u16 = 1;
    // Unique flags, 8 available (bits 0 to 2)

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
        (self.0 & Self::DIRECTION_MASK) >> 7
    }
    pub fn this_type(&self) -> u16 {
        (self.0 & Self::ANT_ROLE_MASK) >> 5
    }
    pub fn carrying_food(&self) -> bool {
        (self.0 & Self::CARRYING_MASK) >> 3 == Self::IS_CARRYING_FOOD
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
        self.0 = (self.0 & !Self::DIRECTION_MASK) | ((direction & 0b111) << 7);
    }
    pub fn set_carrying_food(&mut self, value: bool) {
        if value {
            self.0 = (self.0 & !Self::CARRYING_MASK) | Self::IS_CARRYING_FOOD << 3;
        } else if Self::carrying_food(self) {
            self.0 &= !Self::IS_CARRYING_FOOD;
        }
    }
}


pub struct AntUnit {
    pub ant: Ant,
    pub x: usize,
    pub y: usize,
}


pub struct World {
    pub grid: [Tile; SIMULATION_HEIGHT * SIMULATION_WIDTH],
    pub ants: Vec<AntUnit>,
}

impl World {
    pub fn new(rng: &mut impl rand::Rng) -> Self {
        let grid = std::array::from_fn(|_| Tile::new(rng, FOOD_SPAWNING_CHANCE_PERCENTAGE));
        let ants = Vec::with_capacity(STARTING_ANT_COUNT as usize);

        World { grid, ants }
    }

    pub fn idx(x: usize, y: usize) -> usize {
        y * SIMULATION_WIDTH + x
    }

    pub fn has_food(&self, x: usize, y: usize) -> bool {
        let index: usize = Self::idx(x, y);
        self.grid[index].has_food()
    }

    pub fn add_ant(&mut self, x: usize, y: usize) {
        let ant = Ant::new();
        self.ants.push(AntUnit { ant, x, y });
        let index: usize = Self::idx(x, y);
        self.grid[index].set_ant(true);
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < SIMULATION_WIDTH && y < SIMULATION_HEIGHT {
            let index: usize = Self::idx(x, y);
            Some(&mut self.grid[index])
        } else {
            None
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


// fn ant_role_name(role: u8) -> &'static str {
//     match role {
//         0 => "Worker",
//         1 => "Soldier",
//         2 => "Scout",
//         3 => "Queen",
//         _ => "Unknown",
//     }
// }


fn run_simulation() -> Result<()> {
    // Create rng thread
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    // Initialize grid with random food
    let mut world = World::new(&mut rng);

    let mut stdout = stdout();
    
    // Add ants
    for _ in 0..STARTING_ANT_COUNT {
        let x = rng.gen_range(0..SIMULATION_WIDTH);
        let y = rng.gen_range(0..SIMULATION_HEIGHT - GROUND_HEIGHT);
        world.add_ant(x, y);
    }
    // Add ground
    for y in SIMULATION_HEIGHT - GROUND_HEIGHT..SIMULATION_HEIGHT {
        for x in 0..SIMULATION_WIDTH {
            if let Some(tile) = world.get_tile_mut(x, y) {
                tile.set_obstacle(true);
            }
        }
    }

    // Hide cursor
    execute!(stdout, crossterm::cursor::Hide)?;

    loop {
        // clear_screen();

        // Move cursor to top left without clearing
        execute!(stdout, MoveTo(0, 0))?;

        writeln!(stdout, "Press 'q' to quit.\n")?;

        for row in world.grid.chunks(SIMULATION_WIDTH) {
            for tile in row {
                let ch = display_tile(tile);
                write!(stdout, "{}", ch)?;
            }
            writeln!(stdout)?;
        }

        stdout.flush()?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break,
                    // KeyCode::Char('w') => println!("Move Up"),
                    // KeyCode::Char('s') => println!("Move Down"),
                    // KeyCode::Char('a') => println!("Move Left"),
                    // KeyCode::Char('d') => println!("Move Right"),
                    _ => {}
                }
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    //Show cursor
    execute!(stdout, crossterm::cursor::Show)?;

    Ok(())
}


fn main() {
    let mut stdout = stdout();

    crossterm::terminal::enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen).unwrap();

    let result = run_simulation();

    execute!(stdout, LeaveAlternateScreen).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();

    clear_screen();
    println!("Goodbye.");

    if let Err(err) = result {
        eprintln!("Simulation error: {}", err);
    }
}