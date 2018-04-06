use component::DestroyType;
use entity_states::EntityStates;
use app::ButtonStates;
use piston_window::UpdateArgs;
use entity::Entity;
use component::Component;
use snowflake::ProcessUniqueId;
use tiled::Map;

#[derive(Debug)]
pub struct Damageable {
    health: u32,
    invulnerable_default: f64,
    invulnerable_timer: f64,
    events: Vec<DamageEvent>,
}

#[derive(Debug)]
struct DamageEvent {
    entity_id: ProcessUniqueId,
    amount: i32,
}

impl Component for Damageable {
    fn update(&mut self, entity: &mut Entity, args: &UpdateArgs, keys: &ButtonStates, entities: &mut EntityStates, map: &Map) -> DestroyType {
        if self.invulnerable_timer > args.dt {
            self.invulnerable_timer -= args.dt;
        } else {
            self.invulnerable_timer = 0f64;
        }
        DestroyType::None
    }
}

impl Damageable {
    pub fn new(health: u32, invulnerable_timeout: f64) -> Self {
        Damageable {
            health: health,
            invulnerable_default: invulnerable_timeout,
            invulnerable_timer: 0f64,
            events: Vec::new(),
        }
    }

    pub fn get_health(&self) -> u32 {
        self.health
    }

    pub fn set_health(&mut self, new_health: u32, entity_id: ProcessUniqueId) -> u32 {
        if self.invulnerable_timer == 0f64 {
            self.events.push(DamageEvent {
                entity_id: entity_id,
                amount: new_health as i32 - self.health as i32,
            });
            if new_health < self.health {
                self.invulnerable_timer = self.invulnerable_default;
            }
            self.health = new_health;
        }
        self.health
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}
