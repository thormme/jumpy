use piston_window::UpdateArgs;
use event::EventData;

#[derive(Debug)]
pub struct UpdateEvent {
    pub args: UpdateArgs
}

impl EventData for UpdateEvent {
    fn get_priority(&self) -> u32 {
        50u32
    }
}
