use crate::ecs::component::{ComponentError, DynComponent};
use crate::ecs::entity::Entity;

use std::error::Error;
use std::fmt::{Debug, Display};

pub struct Scene {
    pub e_vec: Vec<Entity>,
    pub id: usize,
    // TODO: start, update, and stop function ptrs
}

#[derive(Debug)]
pub struct SceneError {
    what: String,
}

impl SceneError {
    pub fn new(what: &str) -> SceneError {
        SceneError { what: String::from(what) }
    }
}
impl Error for SceneError {}
impl Display for SceneError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.what)?;
        Ok(())
    }
}

impl Scene {
    pub fn new() -> Scene {
        static mut ID: usize = 0;
        unsafe {
            ID += 1;
            Scene { e_vec: vec![], id: ID }
        }
    }
    pub const fn empty() -> Scene {
        Scene { e_vec: vec![], id: 0 }
    }
    /*
    pub fn add(&mut self, entity: &'static mut Entity) -> Result<(), SceneError> {
        // ensure entity not added
        for i in 0..self.e_vec.len() {
            if self.e_vec[i] == entity {
                return Err(SceneError::new(
                        &format!("Entity of ID: {} already added to scene of ID: {}",
                                 entity.id, self.id)));
            }
        }
        Ok(self.e_vec.push(entity))
    }
    pub fn remove(&mut self, entity: &'static mut Entity) -> Result<(), SceneError> {
        // search and remove
        for i in 0..self.e_vec.len() {
            if self.e_vec[i] == entity {
                self.e_vec.remove(i);
                return Ok(());
            }
        }
        Err(SceneError::new(
                &format!("Entity of ID: {} not found in scene of ID: {}",
                         entity.id, self.id)))
    }
    */
    pub fn spawn(&mut self) -> &mut Entity {
        self.e_vec.push(Entity::new());
        self.e_vec.last_mut().expect("entity creation failed")
    }
    pub fn despawn(&mut self, entity: &mut Entity) -> Result<(), SceneError> {
        // search and remove
        for i in 0..self.e_vec.len() {
            if &self.e_vec[i] == entity {
                self.e_vec.remove(i);
                return Ok(());
            }
        }
        Err(SceneError::new(
                &format!("Entity of id: {} not found in Scene of id: {}",
                         entity.id, self.id)))
    }
    pub fn update<T: DynComponent>(&mut self) -> Result<(), ComponentError> {
        for i in 0..self.e_vec.len() {
            self.e_vec[i].update::<T>()?;
        }
        Ok(())
    }
}

