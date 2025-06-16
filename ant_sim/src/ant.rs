pub enum AntRole {
    Worker,
    Soldier,
    Scout,
    Queen,
}

impl AntRole {
    pub const COUNT: u8 = 4;
}

impl From<AntRole> for String {
    fn from(role: AntRole) -> Self {
        match role {
            AntRole::Worker => "Worker".to_string(),
            AntRole::Soldier => "Soldier".to_string(),
            AntRole::Scout => "Scout".to_string(),
            AntRole::Queen => "Queen".to_string(),
        }
    }
}

impl From<u8> for AntRole {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => AntRole::Worker,
            1 => AntRole::Soldier,
            2 => AntRole::Scout,
            3 => AntRole::Queen,
            _ => unreachable!()
        }
    }
}

impl From<AntRole> for u8 {
    fn from(role: AntRole) -> Self {
        match role {
            AntRole::Worker => 0,
            AntRole::Soldier => 1,
            AntRole::Scout => 2,
            AntRole::Queen => 3,
        }
    }
}

// *****************************************************

pub enum Carrying {
    None,
    Food,
    a,
    b,
}

impl Carrying {
    pub const COUNT: u8 = 4;
}

impl From<Carrying> for String {
    fn from(carrying: Carrying) -> Self {
        match carrying {
            Carrying::None => "None".to_string(),
            Carrying::Food => "Food".to_string(),
            Carrying::a => "a".to_string(),
            Carrying::b => "b".to_string(),
        }
    }
}

impl From<u8> for Carrying {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => Carrying::None,
            1 => Carrying::Food,
            2 => Carrying::a,
            3 => Carrying::b,
            _ => unreachable!()
        }
    }
}

impl From<Carrying> for u8 {
    fn from(carrying: Carrying) -> Self {
        match carrying {
            Carrying::None => 0,
            Carrying::Food => 1,
            Carrying::a => 2,
            Carrying::b => 3,
        }
    }
}

// *****************************************************

pub enum UniqueFlag {
    None,
    Flag1,
    Flag2,
    Flag3,
    Flag4,
    Flag5,
    Flag6,
    Flag7,
}

impl UniqueFlag {
    pub const COUNT: u8 = 8;
}

impl From<UniqueFlag> for String {
    fn from(flag: UniqueFlag) -> Self {
        match flag {
            UniqueFlag::None => "None".to_string(),
            UniqueFlag::Flag1 => "Flag1".to_string(),
            UniqueFlag::Flag2 => "Flag2".to_string(),
            UniqueFlag::Flag3 => "Flag3".to_string(),
            UniqueFlag::Flag4 => "Flag4".to_string(),
            UniqueFlag::Flag5 => "Flag5".to_string(),
            UniqueFlag::Flag6 => "Flag6".to_string(),
            UniqueFlag::Flag7 => "Flag7".to_string(),
        }
    }
}

impl From<u8> for UniqueFlag {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => UniqueFlag::None,
            1 => UniqueFlag::Flag1,
            2 => UniqueFlag::Flag2,
            3 => UniqueFlag::Flag3,
            4 => UniqueFlag::Flag4,
            5 => UniqueFlag::Flag5,
            6 => UniqueFlag::Flag6,
            7 => UniqueFlag::Flag7,
            _ => unreachable!()
        }
    }
}

impl From<UniqueFlag> for u8 {
    fn from(flag: UniqueFlag) -> Self {
        match flag {
            UniqueFlag::None => 0,
            UniqueFlag::Flag1 => 1,
            UniqueFlag::Flag2 => 2,
            UniqueFlag::Flag3 => 3,
            UniqueFlag::Flag4 => 4,
            UniqueFlag::Flag5 => 5,
            UniqueFlag::Flag6 => 6,
            UniqueFlag::Flag7 => 7,
        }
    }
}

// *****************************************************

pub enum Direction {
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
    pub const COUNT: u8 = 8;
}

impl Direction {
    const DIRECTION_COUNT: u8 = 8;

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
        Direction::from((self as u8 + 7) % Self::DIRECTION_COUNT)
    }

    pub fn turn_right(self) -> Self {
        Direction::from((self as u8 + 1) % Self::DIRECTION_COUNT)
    }
}

impl From<Direction> for String {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => "Up".to_string(),
            Direction::UpRight => "UpRight".to_string(),
            Direction::Right => "Right".to_string(),
            Direction::DownRight => "DownRight".to_string(),
            Direction::Down => "Down".to_string(),
            Direction::DownLeft => "DownLeft".to_string(),
            Direction::Left => "Left".to_string(),
            Direction::UpLeft => "UpLeft".to_string(),
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => Direction::Up,
            1 => Direction::UpRight,
            2 => Direction::Right,
            3 => Direction::DownRight,
            4 => Direction::Down,
            5 => Direction::DownLeft,
            6 => Direction::Left,
            7 => Direction::UpLeft,
            _ => unreachable!()
        }
    }
}

impl From<Direction> for u8 {
    fn from(direction: Direction) -> Self {
        direction as u8
    }
}

// *****************************************************

#[derive(Debug, Clone)]
pub struct Ant(u16);

impl Ant {
    //Bit masks
    const MAX_HEALTH_MASK: u16     = 0b1100_0000_0000_0000;
    const CURRENT_HEALTH_MASK: u16 = 0b0011_0000_0000_0000;
    const STRENGTH_MASK: u16       = 0b0000_1100_0000_0000;
    const DIRECTION_MASK: u16      = 0b0000_0011_1000_0000; // 8 directions (full (enum))
    const ANT_ROLE_MASK: u16       = 0b0000_0000_0110_0000; // 4 ant roles (full (enum))
    const CARRYING_MASK: u16       = 0b0000_0000_0001_1000; // 4 carrying options (2, 3 available)
    const UNIQUE_FLAGS_MASK: u16   = 0b0000_0000_0000_0111; // 7 unique flags (1-7 available)

    pub fn new() -> Self {
        Ant(0)
    }

    // Getters
    pub fn max_health(&self) -> u8 {
        ((self.0 & Self::MAX_HEALTH_MASK) >> 14) as u8
    }
    pub fn current_health(&self) -> u8 {
        ((self.0 & Self::CURRENT_HEALTH_MASK) >> 12) as u8
    }
    pub fn strength(&self) -> u8 {
        ((self.0 & Self::STRENGTH_MASK) >> 10) as u8
    }
    pub fn direction(&self) -> Direction {
        Direction::from(((self.0 & Self::DIRECTION_MASK) >> 7) as u8)
    }
    pub fn ant_role(&self) -> AntRole {
        AntRole::from(((self.0 & Self::ANT_ROLE_MASK) >> 5) as u8)
    }
    pub fn carrying(&self) -> Carrying {
        Carrying::from(((self.0 & Self::CARRYING_MASK) >> 3) as u8)
    }
    pub fn unique_flag(&self) -> UniqueFlag {
        UniqueFlag::from((self.0 & Self::UNIQUE_FLAGS_MASK) as u8)
    }

    // Setters
    pub fn set_max_health(&mut self, health: u8) {
        self.0 = (self.0 & !Self::MAX_HEALTH_MASK) | (((health & 0b11) as u16) << 14);
    }
    pub fn set_current_health(&mut self, health: u8) {
        self.0 = (self.0 & !Self::CURRENT_HEALTH_MASK) | (((health & 0b11) as u16) << 12);
    }
    pub fn set_strength(&mut self, strength: u8) {
        self.0 = (self.0 & !Self::STRENGTH_MASK) | (((strength & 0b11) as u16) << 10);
    }
    pub fn set_direction(&mut self, direction: Direction) {
        self.0 = (self.0 & !Self::DIRECTION_MASK) | ((u8::from(direction) & 0b111) as u16) << 7;
    }
    pub fn set_ant_role(&mut self, role: AntRole) {
        self.0 = (self.0 & !Self::ANT_ROLE_MASK) | ((u8::from(role) & 0b11) as u16) << 5;
    }
    pub fn set_carrying(&mut self, value: Carrying) {
        self.0 = (self.0 & !Self::CARRYING_MASK) | ((u8::from(value) & 0b11) as u16) << 3;
    }
    pub fn set_unique_flag(&mut self, flags: UniqueFlag) {
        self.0 = (self.0 & !Self::UNIQUE_FLAGS_MASK) | (u8::from(flags) as u16);
    }
}