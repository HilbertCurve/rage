use super::component::{Component, ComponentError};

use std::rc::{Weak, Rc};

pub struct Go {
    components: Vec<Box<dyn Component>>,
}

impl Go {
    pub fn attach<T: Component + 'static>(&mut self, mut com: T) -> Result<(), ComponentError> {
        // ensure component of type T doesn't exist yet
        for g_com in &mut self.components {
            if g_com.as_ref().type_enum() == com.type_enum() {
                return Err(ComponentError::AlreadyPresent);
            }
        }

        com.set_parent(&self);
        self.components.push(Box::new(com));

        Ok(())
    }
}

