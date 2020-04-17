use crate::game::entity::{EntityProp, Entity};
use std::time::Duration;

pub mod worker;

pub struct UnitProp {
    entity: EntityProp
}

impl UnitProp {
    fn new(name: String, build_time: u32, life: u32, max_life: u32, player_id: u32, range: u32, base_damage: u32, base_armor: u32) -> Self {
        return UnitProp {
            entity: EntityProp::new(name, build_time, life, max_life, player_id, range, base_damage, base_armor)
        }
    }
}

impl Entity for UnitProp {
    fn update(&mut self) -> Duration {
       return self.entity.update()
    }
}