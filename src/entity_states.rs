
use std::collections::hash_map::Keys;
use std::collections::HashSet;
use nalgebra::Point2;
use std::collections::HashMap;
use entity::Entity;
use snowflake::ProcessUniqueId;

const ZONE_WIDTH: usize = 50;
const ZONE_HEIGHT: usize = 50;

pub struct EntityStates {
    entities: HashMap<ProcessUniqueId, Box<Entity>>,
    entity_zones: Vec<Vec<HashSet<ProcessUniqueId>>>,
}

impl EntityStates {
    pub fn new() -> EntityStates {
        EntityStates {
            entities: HashMap::new(),
            entity_zones: vec![],
        }
    }

    fn get_zone(pos: Point2<f32>) -> Point2<usize> {
        return Point2::new(pos.x as usize / ZONE_WIDTH, pos.y as usize / ZONE_HEIGHT);
    }

    pub fn remove(&mut self, id: &ProcessUniqueId) -> Option<Box<Entity>> {
        if let Some(entity) = self.entities.remove(id) {
            if let Some(body) = entity.get_body() {
                let zone = EntityStates::get_zone(body.pos);
                if zone.y < self.entity_zones.len() && zone.x < self.entity_zones[zone.y].len() {
                    let mut id_set = &mut self.entity_zones[zone.y][zone.x];
                    id_set.remove(id);
                }
            }
            return Some(entity);
        } else {
            return None;
        }
    }

    pub fn insert(&mut self, id: ProcessUniqueId, entity: Box<Entity>) {
        if let Some(body) = entity.get_body() {
            let zone = EntityStates::get_zone(body.pos);
            while self.entity_zones.len() <= zone.y {
                self.entity_zones.push(vec![]);
            }
            while self.entity_zones[zone.y].len() <= zone.x {
                self.entity_zones[zone.y].push(HashSet::new());
            }
            self.entity_zones[zone.y][zone.x].insert(id);
        }
        self.entities.insert(id, entity);
    }

    pub fn keys(&self) -> Keys<ProcessUniqueId, Box<Entity>> {
        self.entities.keys()
    }

    pub fn get(&self, key: &ProcessUniqueId) -> Option<&Box<Entity>> {
        self.entities.get(key)
    }

    pub fn for_each<F>(&self, mut f: F) where F : FnMut(&Box<Entity>) {
        for (_, entity) in &self.entities {
            f(&entity);
        }
    }

    pub fn for_zone<F>(&mut self, pos: Point2<f32>, range: usize, mut f: F) where F : FnMut(&mut Box<Entity>) {
        let zone = EntityStates::get_zone(pos);
        for y in zone.y.saturating_sub(range) ..= zone.y.saturating_add(range) {
            if y >= self.entity_zones.len() {
                continue;
            }
            for x in zone.x.saturating_sub(range) ..= zone.x.saturating_add(range) {
                if x >= self.entity_zones[y].len() {
                    continue;
                }
                let id_set = &mut self.entity_zones[y][x];
                for id in id_set.iter() {
                    if let Some(entity) = self.entities.get_mut(id) {
                        f(entity);
                    }
                }
            }
        }
    }
}
