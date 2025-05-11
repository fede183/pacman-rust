use std::time::Duration;
use bevy::time::{Time, Timer, TimerMode};
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct EventBlocker {
    timer_option: Option<Timer>,
    midprocess: bool,
}


impl EventBlocker {
    pub fn new() -> Self {
        Self { timer_option: None, midprocess: false }
    }

    pub fn check(&mut self, time: Time) -> bool {
        if self.midprocess {
            return false;
        }

        let timer_option_mut = &mut self.timer_option;

        if let Some(timer) = timer_option_mut {
            timer.tick(time.delta());

            if !(timer.finished() && timer.just_finished()) {
                return false;
            }
        } 
        
        self.timer_option = None;

        return true;
    }

    pub fn lock_process(&mut self) {
        self.midprocess = true;
        self.timer_option = Some(Timer::new(Duration::from_secs_f32(0.3), TimerMode::Once));
    }

    pub fn finish_process(&mut self) {
        self.midprocess = false;
    }
}

// We need to implement Default so that our timer can be initialized as
// a resource when we call `init_resource`
impl Default for EventBlocker {
    fn default() -> Self {
        Self::new()
    }
}
