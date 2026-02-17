use glfw::{Key, WindowEvent};

use crate::{Crash, game::{GameAction, settings::InputSettings}};

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
    pub delta_time: f32,

    pub cursor: (f32, f32),
    cursor_last: (f32, f32),
    pub cursor_diff: (f32, f32),
    pub mouse_sensitivity: f32,
}

impl Input {
    //TODO refactor pleaseeeee :(
    pub fn process(&mut self, events: &glfw::GlfwReceiver<(f64, WindowEvent)>, cursor_pos: (f64, f64)) -> Result<GameAction, Crash>{
        //window.glfw.poll_events();

        for (_, event) in glfw::flush_messages(events) {
            match event {
                //wtf is this syntax xd loool?
                WindowEvent::Key(key,_ , glfw::Action::Press, _) => {match key {
                    k if k == self.forward.1 => self.forward.0 = 1.0,
                    k if k == self.forward.2 => self.forward.0 = -1.0,

                    k if k == self.right.1 => self.right.0 = 1.0,
                    k if k == self.right.2 => self.right.0 = -1.0,
                    
                    k if k == self.exit.1 => self.exit.0 = 1.0,

                    _ => {},
                }},
                WindowEvent::Key(key,_ , glfw::Action::Release, _) => {match key {
                    k if k == self.forward.1 => self.forward.0 = 0.0,
                    k if k == self.forward.2 => self.forward.0 = 0.0,

                    k if k == self.right.1 => self.right.0 = 0.0,
                    k if k == self.right.2 => self.right.0 = 0.0,
                    
                    k if k == self.exit.1 => self.exit.0 = 0.0,

                    _ => {},
                }},
                WindowEvent::Size(x, y) => return Ok(GameAction::Resize(x as u32, y as u32)),
                _ => {},              
            };
        }

        self.cursor = (cursor_pos.0 as f32, cursor_pos.1 as f32);
        self.cursor_diff = (self.cursor_last.0 - self.cursor.0, self.cursor_last.1 - self.cursor.1);
        self.cursor_last = self.cursor;

        Ok(GameAction::None)
    }
    pub fn from_settings(settings: &InputSettings) -> Self {
        Self {
            forward: (0.0, settings.forward.0, settings.forward.1),
            right: (0.0, settings.right.0, settings.right.1),

            exit: (0.0, settings.exit),
            delta_time: 0.0,
            cursor: (0.0, 0.0),
            cursor_last: (0.0, 0.0),
            cursor_diff: (0.0, 0.0),
            mouse_sensitivity: settings.mouse_sense
        }
    }
}