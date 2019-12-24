use rand::prelude::*;
#[derive(Copy, Debug, Clone)]
pub enum Cell {
    Dead,
    Alive,
}

impl Cell {
    pub fn as_num(self) -> u8 {
        match self {
            Cell::Dead => 0,
            Cell::Alive => 1,
        }
    }
    pub fn is_alive(self) -> bool {
        match self {
            Cell::Dead => false,
            Cell::Alive => true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Model {
    world: Vec<Cell>,
    pub width: u32,
    pub height: u32,
    pub population: u32,
    rule: Rule,
    stable: bool,
}

pub fn random_world(size: u32) -> Vec<Cell> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| {
            if rng.gen::<bool>() {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect()
}

impl Model {
    pub fn from_world(w: Vec<Cell>, width: u32, height: u32, rule: Rule) -> Self {
        assert_eq!(w.len() as u32, width * height);
        let p = w
            .iter()
            .fold(0, |acc, &c| if c.is_alive() { acc + 1 } else { acc });
        Model {
            world: w,
            width,
            height,
            population: p,
            rule,
            stable: false,
        }
    }
    pub fn tick_and_report(&mut self) -> Vec<(u32, Cell)> {
        // update itself, and reports the change in population
        if self.stable {
            return vec![];
        };
        let current = self.clone();
        let mut updates = Vec::new();
        for (i, &cell) in current.world.iter().enumerate() {
            let neighbours = current.neighbours_of(i as u32);
            if cell.is_alive() {
                if neighbours < self.rule.alive_min || self.rule.alive_max < neighbours {
                    self.world[i] = Cell::Dead;
                    self.population -= 1;
                    updates.push((i as u32, Cell::Dead));
                }
            } else {
                // for dead cells
                if self.rule.birth_min <= neighbours && neighbours <= self.rule.birth_max {
                    self.world[i] = Cell::Alive;
                    self.population += 1;
                    updates.push((i as u32, Cell::Alive));
                }
            }
        }
        // if there's no change, there won't be any change
        self.stable = updates.is_empty();
        updates
    }
}

impl Model {
    fn at(&self, x: u32, y: u32) -> Cell {
        self.world[(y * self.width + x) as usize]
    }
    fn neighbours_of(&self, loc: u32) -> u8 {
        let mut ns = 0;
        let (x, y) = (loc % self.width, loc / self.width);
        for &dx in &[self.width - 1, 0, 1] {
            for &dy in &[self.height - 1, 0, 1] {
                ns += self
                    .at((x + dx) % self.width, (y + dy) % self.height)
                    .as_num();
            }
        }
        ns -= self.world[loc as usize].as_num();
        ns
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Rule {
    /// for a Cell::Dead, if birth_min <= neighbour <= birth_max then new cell is born
    /// for a Cell::Alive, if alive_min <= neighbour <= alive_max then it stays alive
    pub birth_min: u8,
    pub birth_max: u8,
    pub alive_min: u8,
    pub alive_max: u8,
}

pub const CONWAY: Rule = Rule {
    birth_min: 3,
    birth_max: 3,
    alive_min: 2,
    alive_max: 3,
};
