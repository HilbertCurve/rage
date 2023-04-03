use super::component::{Component, ComponentError, DynComponent};

pub struct Entity {
    components: Vec<Box<dyn Component + 'static>>,
    id: usize,
    name: String,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl Entity {
    pub fn new(name: String) -> Entity {
        static mut ID: usize = 0;
        unsafe {
            ID += 1;
            Entity { components: vec![], id: ID, name }
        }
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

    pub fn attach<T: DynComponent>(&mut self, com: T) -> Result<(), ComponentError> {
        for g_com in &mut self.components {
            if let Some(_) = g_com.as_any_mut().downcast_mut::<T>() {
                return Err(ComponentError::AlreadyPresent(T::type_str().to_owned()));
            }
        }

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
            if let Some(_) = g_com.as_any_mut().downcast_mut::<T>() {
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
        let c = self as *mut Entity;
        unsafe {
            self.get_mut::<T>()?.update(c)
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

