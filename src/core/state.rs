use std::error::Error;

use crate::ecs::entity::Entity; 
use crate::ecs::component::{Component, DynComponent, ComponentError};

#[derive(Error)]
pub struct StateError {
    what: String
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

pub type StateResult = crate::prelude::RageResult;

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
            states: states.to_vec(),
            state_index: 0,
        }
    }
}

impl StateMachine {
    pub fn change_state(&mut self, name: &str) -> StateResult {
        for i in 0..self.states.len() {
            if self.states[i].name == name {
                self.state_index = i;
                return Ok(())
            }
        }

        Err(Box::new(StateError::from(format!("No state named {} found in StateMachine.", name))))
    }

    pub fn current_state(&self) -> &State {
        &self.states[self.state_index]
    }
}

impl DynComponent for StateMachine {
    unsafe fn start(&mut self, _parent: *mut Entity) -> Result<(), crate::ecs::component::ComponentError> {
        Ok(())
    }

    unsafe fn update(&mut self, dt: f64, parent: *mut Entity) -> Result<(), crate::ecs::component::ComponentError> {
        if let Err(e) = (self.states[self.state_index].update)(&mut *parent, dt) {
            Err(ComponentError::BadUpdate(e.to_string()))
        } else {
            Ok(())
        }
    }

    unsafe fn stop(&mut self, _parent: *mut Entity) -> Result<(), crate::ecs::component::ComponentError> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct State {
    pub name: String,
    pub update: fn(&mut Entity, dt: f64) -> crate::prelude::RageResult,
}

impl State {
    pub fn from(
        name: &str,
        update: fn(&mut Entity, dt: f64) -> crate::prelude::RageResult,
    ) -> State {
        State { name: name.to_owned(), update }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
