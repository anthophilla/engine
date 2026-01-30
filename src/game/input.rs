use glfw::{Key, WindowEvent};

use crate::{Crash, game::settings::InputSettings, renderer::window::Window};

// enum KeyAlias {
//     W,
//     A,
//     S,
//     D,

//     Escape,
// }
// impl Into<glfw::Key> for KeyAlias {

// }

// pub enum InputError {

// }

//value, positive key, negative key
type Axis = (f32, Key, Key);
pub struct Input {
    pub forward: Axis,
    pub right: Axis,

    pub exit: (f32, Key),
}

impl Input {
    //TODO refactor pleaseeeee :(
    pub fn process(&mut self, events: &glfw::GlfwReceiver<(f64, WindowEvent)>) -> Result<(), Crash>{
        //window.glfw.poll_events();

        for (_, event) in glfw::flush_messages(events) {
            match event {
                //wtf is this syntax xd loool?
                WindowEvent::Key(key,_ , glfw::Action::Press, _) => {match key {
                    k if k == self.forward.1 => self.forward.0 = 1.0,
                    k if k == self.forward.2 => self.forward.0 = -1.0,

                    k if k == self.right.1 => self.right.0 = 1.0,                    k if k == self.forward.1 => self.forward.0 = 1.0,
                    k if k == self.right.2 => self.right.0 = -1.0,
                    
                    k if k == self.exit.1 => self.exit.0 = 1.0,

                    _ => {},
                }},
                _ => {},              
            };
        }

        Ok(())
    }
    pub fn from_settings(settings: &InputSettings) -> Self {
        Self {
            forward: (0.0, settings.forward.0, settings.forward.1),
            right: (0.0, settings.right.0, settings.right.1),

            exit: (0.0, settings.exit)
        }
    }
}