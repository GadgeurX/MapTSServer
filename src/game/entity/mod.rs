
use std::time::{Duration, SystemTime};
use rand::Rng;

pub mod unit;

pub struct EntityProp {
    id: u32,
    name: String,
    build_time: u32,
    life: u32,
    max_life: u32,
    player_id: u32,
    //Vector3D m_Position;
    range: u32,
    //double m_Angle;
    base_damage: u32,
    base_armor: u32,
    //Resource m_Cost, m_SellValue;
    //EntityAction m_Action;
    time_watch: SystemTime,
}

impl EntityProp {
    fn new(name: String, build_time: u32, life: u32, max_life: u32, player_id: u32, range: u32, base_damage: u32, base_armor: u32) -> Self {
        return EntityProp {
            id: rand::thread_rng().gen(),
            name,
            build_time,
            life,
            max_life,
            player_id,
            range,
            base_damage,
            base_armor,
            time_watch: SystemTime::now()
        };
    }
}

pub trait Entity {
    fn update(&mut self) -> Duration;
}

impl Entity for EntityProp {
    fn update(&mut self) -> Duration {
        let duration = self.time_watch.elapsed().unwrap();
        // Implement here
        self.time_watch = SystemTime::now();
        return duration;
    }
}