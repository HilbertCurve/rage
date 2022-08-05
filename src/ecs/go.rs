use super::component::{Component, ComponentEnum, ComponentError};

// TODO: rename to entity
pub struct Go {
    components: Vec<Box<dyn Component>>,
}

// TODO: add, remove, detach, integrate DynComponent, use as_any and as_any_mut, implement
// component macro
impl Go {
    pub fn attach<T: Component + Copy>(&'static mut self, com: T) -> Result<(), ComponentError> {
        // ensure component of type T doesn't exist yet
        for g_com in &mut self.components {
            if g_com.as_ref().type_enum() == com.type_enum() {
                return Err(ComponentError::AlreadyPresent);
            }
        }

        self.components.push(Box::new(com));

        Ok(())
    }

    pub fn get_component(&'static mut self, com_type: ComponentEnum) -> Result<&mut dyn Component, ComponentError> {
        for g_com in &mut self.components {
            if g_com.as_ref().type_enum() == com_type {
                return Ok(g_com.as_mut());
            }
        }
        Err(ComponentError::NotPresent)
    }
    pub fn update(&'static self) {
        for com in &self.components {
            com.as_ref().update(self);
        }
    }
}

