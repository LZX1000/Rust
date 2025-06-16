use crate::ant;
use crate::constants;


pub enum Objects {
    None,
    Ant,
    Food,
    Obstacle,
    Object4,
    Object5,
    Object6,
    Object7,
}

impl From<Objects> for String {
    fn from(object: Objects) -> Self {
        match object {
            Objects::None => "None".to_string(),
            Objects::Ant => "Ant".to_string(),
            Objects::Food => "Food".to_string(),
            Objects::Obstacle => "Obstacle".to_string(),
            Objects::Object4 => "Object4".to_string(),
            Objects::Object5 => "Object5".to_string(),
            Objects::Object6 => "Object6".to_string(),
            Objects::Object7 => "Object7".to_string(),
        }
    }
}

impl From<u8> for Objects {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => Objects::None,
            1 => Objects::Ant,
            2 => Objects::Food,
            3 => Objects::Obstacle,
            4 => Objects::Object4,
            5 => Objects::Object5,
            6 => Objects::Object6,
            7 => Objects::Object7,
            _ => unreachable!(),
        }
    }
}

impl From<Objects> for u8 {
    fn from(object: Objects) -> Self {
        match object {
            Objects::None => 0,
            Objects::Ant => 1,
            Objects::Food => 2,
            Objects::Obstacle => 3,
            Objects::Object4 => 4,
            Objects::Object5 => 5,
            Objects::Object6 => 6,
            Objects::Object7 => 7,
        }
    }
}

// *****************************************************

#[derive(Debug, Clone)]
pub struct Tile(u8);

impl Tile {
    // Bit masks
    const HAS_OBJECT_MASK: u8           = 0b1110_0000;
    const FOOD_PHEROMONE_LEVEL_MASK: u8 = 0b0001_1000;
    const HOME_PHEROMONE_LEVEL_MASK: u8 = 0b0000_0111;

    pub fn new(rng: &mut impl rand::Rng, food_chance_percentage: u8) -> Self {
        let mut tile = Tile(0);

        if rng.gen_range(0..100) < food_chance_percentage {
            tile.set_object(Objects::Food);
        }
        tile
    }

    // Getters
    pub fn object(&self) -> Objects {
        Objects::from((self.0 & Self::HAS_OBJECT_MASK) >> 5)
    }
    pub fn pheromone(&self) -> u8 {
        (self.0 & Self::FOOD_PHEROMONE_LEVEL_MASK) >> 3
    }
    pub fn home_pheromone(&self) -> u8 {
        self.0 & Self::HOME_PHEROMONE_LEVEL_MASK
    }

    // Setters
    pub fn set_object(&mut self, object: Objects) {
        self.0 = (self.0 & !Self::HAS_OBJECT_MASK) | ((object as u8) << 5);
    }
    pub fn set_pheromone(&mut self, level: u8) {
        self.0 = (self.0 & !Self::FOOD_PHEROMONE_LEVEL_MASK) | ((level.min(3) & 0b11) << 3);
    }
    pub fn set_home_pheromone(&mut self, level: u8) {
        self.0 = (self.0 & !Self::HOME_PHEROMONE_LEVEL_MASK) | (level.min(7) & 0b111);
    }
}

// *****************************************************

pub struct AntUnit {
    pub ant: ant::Ant,
    pub x: usize,
    pub y: usize,
}

// *****************************************************

pub struct World {
    pub grid: [Tile; constants::SIMULATION_HEIGHT * constants::SIMULATION_WIDTH],
    pub ants: Vec<AntUnit>
}

impl World {
    pub fn new(rng: &mut impl rand::Rng) -> Self {
        let grid = std::array::from_fn(|_| Tile::new(rng, constants::FOOD_SPAWNING_CHANCE_PERCENTAGE));
        let mut world = World {
            grid,
            ants: Vec::with_capacity(constants::STARTING_ANT_COUNT as usize),
        };

        // Add ants
        for _ in 0..constants::STARTING_ANT_COUNT {
            let x = rng.gen_range(0..constants::SIMULATION_WIDTH);
            let y = rng.gen_range(0..constants::SIMULATION_HEIGHT - constants::GROUND_HEIGHT);
            world.add_object(x, y, Objects::Ant);
        }
        
        // Add ground
        for y in constants::SIMULATION_HEIGHT - constants::GROUND_HEIGHT..constants::SIMULATION_HEIGHT {
            for x in 0..constants::SIMULATION_WIDTH {
                if let Some(tile) = world.get_tile_mut(x, y) {
                    tile.set_object(Objects::Obstacle);
                }
            }
        }

        world
    }

    pub fn idx(x: usize, y: usize) -> usize {
        y * constants::SIMULATION_WIDTH + x
    }

    // pub fn has_food(&self, x: usize, y: usize) -> bool {
    //     let index: usize = Self::idx(x, y);
    //     self.grid[index].
    // }

    pub fn add_object(&mut self, x: usize, y: usize, object: Objects) {
        let ant = ant::Ant::new();
        self.ants.push(AntUnit { ant, x, y });
        let index: usize = Self::idx(x, y);
        self.grid[index].set_object(object);
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x < constants::SIMULATION_WIDTH && y < constants::SIMULATION_HEIGHT {
            let index: usize = Self::idx(x, y);
            Some(&mut self.grid[index])
        } else {
            None
        }
    }
}