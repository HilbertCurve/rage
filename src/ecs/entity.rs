use super::component::{Component, ComponentError, DynComponent};

pub struct Entity {
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new() -> Entity {
        Entity { components: vec![] }
    }

    pub fn add<T: Component>(&mut self, com: T) -> Result<(), ComponentError> {
        // ensure component of type T doesn't exist yet
        for g_com in &mut self.components {
            if let Some(_) = g_com.as_any_mut().downcast_mut::<T>() {
                return Err(ComponentError::AlreadyPresent(T::type_str().to_owned()));
            }
        }

        self.components.push(Box::new(com));

        Ok(())
    }

    pub fn attach<T: DynComponent>(&mut self, mut com: T) -> Result<(), ComponentError> {
        for g_com in &mut self.components {
            if let Some(_) = g_com.as_any_mut().downcast_mut::<T>() {
                return Err(ComponentError::AlreadyPresent(T::type_str().to_owned()));
            }
        }

        com.set_parent(self);
        self.components.push(Box::new(com));

        Ok(())
    }

    pub fn remove<T: Component>(&mut self) -> Result<(), ComponentError> {
        let mut acc: usize = 0;
        for g_com in &mut self.components {
            if let Some(_) = g_com.as_any_mut().downcast_mut::<T>() {
                self.components.remove(acc);
                return Ok(());
            } else {
                acc += 1;
            }
        }

        Err(ComponentError::NotPresent(T::type_str().to_owned()))
    }

    pub fn detach<T: DynComponent>(&mut self) -> Result<(), ComponentError> {
        let mut acc: usize = 0;
        for g_com in &mut self.components {
            if let Some(com) = g_com.as_any_mut().downcast_mut::<T>() {
                com.detach();
                self.components.remove(acc);
                return Ok(());
            } else {
                acc += 1;
            }
        }

        Err(ComponentError::NotPresent(T::type_str().to_owned()))
    }

    pub fn get<T: Component>(&self) -> Result<&T, ComponentError> {
        for g_com in &self.components {
            if let Some(com) = g_com.as_any().downcast_ref::<T>() {
                return Ok(com);
            }
        }

        Err(ComponentError::NotPresent(T::type_str().to_owned()))
    }

    pub fn get_mut<T: Component>(&mut self) -> Result<&mut T, ComponentError> {
        for g_com in &mut self.components {
            if let Some(com) = g_com.as_any_mut().downcast_mut::<T>() {
                return Ok(com);
            }
        }

        Err(ComponentError::NotPresent(T::type_str().to_owned()))
    }

    pub fn update<T: DynComponent>(&mut self) -> Result<(), ComponentError> {
        self.get_mut::<T>()?.update()
    }
}

