use amethyst::{
    prelude::*,
    input::{VirtualKeyCode, is_key_down},
    core::timing::Time,
};

#[derive(Default)]
pub struct PauseState;

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut time = data.world.write_resource::<Time>();
        println!("Game Paused");
        time.set_time_scale(0.0);
    }

    fn handle_event(&mut self, mut _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::P){
                return Trans::Pop;
            }
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut time = data.world.write_resource::<Time>();
        time.set_time_scale(1.0);
    }
}
