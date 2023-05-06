use std::error::Error;
use std::fmt::Display;

use crate::ecs::entity::Entity; 
use crate::ecs::component::{Component, DynComponent, ComponentError};

#[derive(Debug)]
pub struct StateError {
    what: String
}

impl Error for StateError {  }

impl Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StateError: {}", self.what)
    }
}

impl Into<ComponentError> for StateError {
    fn into(self) -> ComponentError {
        ComponentError::BadUpdate(format!("Error updating state: {}", self.what))
    }
}

impl From<String> for StateError {
    fn from(what: String) -> Self {
        StateError { what }
    }
}

pub type StateResult = Result<(), StateError>;

#[derive(Component)]
pub struct StateMachine {
    states: Vec<State>,
    state_index: usize,
}

impl From<Vec<State>> for StateMachine {
    fn from(states: Vec<State>) -> Self {
        StateMachine {
            states,
            state_index: 0,
        }
    }
}

impl From<&[State]> for StateMachine {
    fn from(states: &[State]) -> Self {
        StateMachine {
            
        }
    }
}

impl StateMachine {
    pub fn change_state(&mut self, name: String) -> StateResult {
        for i in 0..self.states.len() {
            if self.states[i].name == name {
                self.state_index = i;
                return Ok(())
            }
        }

        Err(StateError::from(format!("No state named {} found in StateMachine.", name)))
    }
}

impl DynComponent for StateMachine {
    unsafe fn start(&mut self, parent: *mut Entity) -> Result<(), crate::ecs::component::ComponentError> {
        if let Err(e) = (self.states[self.state_index].start)(&mut *parent) {
            Err(e.into())
        } else {
            Ok(())
        }
    }

    unsafe fn update(&mut self, dt: f64, parent: *mut Entity) -> Result<(), crate::ecs::component::ComponentError> {
        if let Err(e) = (self.states[self.state_index].update)(&mut *parent, dt) {
            Err(e.into())
        } else {
            Ok(())
        }
    }

    unsafe fn stop(&mut self, parent: *mut Entity) -> Result<(), crate::ecs::component::ComponentError> {
        if let Err(e) = (self.states[self.state_index].stop)(&mut *parent) {
            Err(e.into())
        } else {
            Ok(())
        }
    }
}

pub struct State {
    name: String,
    start: fn(&mut Entity) -> StateResult,
    update: fn(&mut Entity, dt: f64) -> StateResult,
    stop: fn(&mut Entity) -> StateResult,
    
}
