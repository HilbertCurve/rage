use std::collections::HashMap;
use std::error::Error;

use crate::prelude::RageResult;

#[derive(Error)]
pub struct AssetError {
    pub what: String
}

pub trait Asset: 'static {
    fn new() -> Self where Self: Sized;
    fn clear(&mut self) -> RageResult;
    fn as_any(&self) -> &dyn std::any::Any where Self: Sized {
        self as &dyn std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any where Self: Sized {
        self as &mut dyn std::any::Any
    }
    // TODO: flawed: use better type checking
    fn type_str() -> &'static str where Self: Sized;
}

pub(crate) struct AssetManager {
    assets: HashMap<String, Box<dyn Asset>>,
}

impl AssetManager {
    pub fn new() -> AssetManager {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn insert<T: Asset + Clone + 'static>(&mut self, key: String, asset: T) {
        self.assets.insert(key, Box::new(asset.clone()));
    }

    pub fn get(&self, key: String) -> Result<&dyn std::any::Any, AssetError> {
        match self.assets.get(&key) {
            Some(v) => Ok(v as &dyn std::any::Any),
            None => Err(AssetError { what: format!("Asset of key: {} not found", key) })
        }
    }

    pub fn get_mut(&mut self, key: String) -> Result<&mut dyn std::any::Any, AssetError> {
        match self.assets.get_mut(&key) {
            Some(v) => Ok(v as &mut dyn std::any::Any),
            None => Err(AssetError { what: format!("Asset of key: {} not found", key) })
        }
    }

    pub fn remove(&mut self, key: String) -> Result<(), AssetError> {
        match self.assets.remove(&key) {
            Some(mut v) => {
                if let Err(err) = v.clear() {
                    Err(AssetError { what: format!("Error removing asset: {}: {}", key, err)})
                } else {
                    Ok(())
                }
            }
            None => Err(AssetError { what: format!("Asset of key: {} not found", key) })
        }
    }
}
