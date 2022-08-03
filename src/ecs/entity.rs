use super::component::{Component, ComponentError};

pub struct Entity {
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn attach<T: Component + Copy>(&'static mut self, com: T) -> Result<(), ComponentError> {
        // ensure component of type T doesn't exist yet
        for g_com in &mut self.components {
            if let Some(_) = g_com.as_any_mut().downcast_mut::<T>() {
                return Err(ComponentError::AlreadyPresent(String::from(T::type_str())))
            }
        }

        self.components.push(Box::new(com));

        Ok(())
    }

    pub fn get<T: Component + 'static>(&'static self) -> Result<&T, ComponentError> {
        for g_com in &self.components {
            if let Some(com) = g_com.as_any().downcast_ref::<T>() {
                return Ok(com);
            }
        }
        Err(ComponentError::NotPresent(String::from(T::type_str())))
    }

    pub fn get_mut<T: Component + 'static>(&'static mut self) -> Result<&mut T, ComponentError> {
        for g_com in &mut self.components {
            if let Some(com) = g_com.as_any_mut().downcast_mut::<T>() {
                return Ok(com);
            }
        }
        Err(ComponentError::NotPresent(String::from(T::type_str())))
    }

    pub fn remove<T: Component + 'static>(&'static mut self) -> Result<(), ComponentError> {
        let mut i: usize = 0;
        for g_com in &mut self.components {
            if let Some(_) = g_com.as_any_mut().downcast_mut::<T>() {
                self.components.remove(i);
                return Ok(());
            } else {
                i += 1;
            }
        }

        Err(ComponentError::NotPresent(String::from(T::type_str())))
    }
}

