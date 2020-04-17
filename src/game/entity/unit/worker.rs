use crate::game::entity::unit::UnitProp;
use crate::game::entity::Entity;
use std::time::Duration;

pub struct Worker {
    unit: UnitProp
}

impl Worker {
    pub fn new(player_id: u32) -> Self {
        return Worker { unit: UnitProp::new(String::from("Worker"), 3000, 100, 100, player_id, 1, 1, 1) }
    }
}

impl Entity for Worker {
    fn update(&mut self) -> Duration {
        return self.unit.update()
    }
}