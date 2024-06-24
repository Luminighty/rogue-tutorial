use rltk::RandomNumberGenerator;
use specs::World;

pub type Spawner = fn(&mut World, i32, i32);

#[derive(Clone, Copy)]
pub struct RandomEntry {
    pub spawner: Spawner,
    pub weight: i32,
}

pub struct RandomTable {
    entries: Vec<RandomEntry>,
    total_weight: i32,
}

impl RandomTable {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            total_weight: 0,
        }
    }

    pub fn add(mut self, spawner: Spawner, weight: i32) -> Self {
        self.entries.push(RandomEntry { spawner, weight });
        self.total_weight += weight;
        self
    }

    pub fn roll(&self, rng: &mut RandomNumberGenerator) -> Option<RandomEntry> {
        if self.entries.is_empty() {
            return None;
        }

        let mut roll = rng.roll_dice(1, self.total_weight);
        let mut index = 0;
        loop {
            if roll <= self.entries[index].weight {
                return Some(self.entries[index]);
            }

            roll -= self.entries[index].weight;
            index += 1;
        }
    }
}
