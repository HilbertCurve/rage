use crate::ecs::component::{ComponentError, DynComponent};
use crate::ecs::entity::Entity;

use std::error::Error;
use std::fmt::{Debug, Display};

pub struct Scene {
    pub e_vec: Vec<Entity>,
    pub id: usize,
    name: String,
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

static mut SCENE_ID_ACC: usize = 0;
impl Scene {
    pub fn new(name: String) -> Scene {
        unsafe {
            SCENE_ID_ACC += 1;
            Scene {
                e_vec: vec![],
                id: SCENE_ID_ACC,
                name,
            }
        }
    }

    pub const fn empty() -> Scene {
        //TODO: make default scene
        Scene {
            e_vec: vec![],
            id: 0,
            name: String::new(),
        }
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
    pub fn spawn(&mut self, name: &str) -> Result<&mut Entity, SceneError> {
        for i in 0..self.e_vec.len() {
            if &self.e_vec[i].name() == name {
                return Err(SceneError::new(
                        &format!("Entity of name: {}, id: {} already in Scene of name: {}, id: {}",
                                 &self.e_vec[i].name(), &self.e_vec[i].id(),
                                 self.name, self.id)));
            }
        }
        self.e_vec.push(Entity::new(name.to_owned()));
        Ok(self.e_vec.last_mut().expect("entity creation failed"))
    }
    pub fn despawn(&mut self, name: &str) -> Result<(), SceneError> {
        // search and remove
        for i in 0..self.e_vec.len() {
            if &self.e_vec[i].name() == name {
                self.e_vec.remove(i);
                return Ok(());
            }
        }
        Err(SceneError::new(
                &format!("Entity of name: {} not found in Scene of name: {}",
                         name,
                         self.name)))
    }
    pub fn get(&mut self, name: &str) -> Result<&mut Entity, SceneError> {
        for i in 0..self.e_vec.len() {
            if &self.e_vec[i].name() == name {
                return Ok(&mut self.e_vec[i]);
            }
        }

        Err(SceneError::new(&format!("Entity of name: {} not found in Scene of name: {}", name, self.name)))
    }

    pub fn update<T: DynComponent>(&mut self, dt: f64) -> Result<(), ComponentError> {
        for i in 0..self.e_vec.len() {
            match self.e_vec[i].update::<T>(dt) {
                Ok(_) => {}
                Err(ComponentError::NotPresent(_)) => {}
                Err(err) => return Err(err)
            }
        }
        Ok(())
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

