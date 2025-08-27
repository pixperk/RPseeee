use std::{collections::HashMap, sync::Arc};

use crate::handler::Handler;


#[derive(Default, Clone)]
pub struct Registry {
    handlers : HashMap<String, Arc<dyn Handler>>
}

impl Registry{
    pub fn new() -> Self{
        Self::default()
    }

    pub fn register_handler(&mut self, method : String, handler : Arc<dyn Handler>){
        self.handlers.insert(method, handler);
    }

    pub fn get_handler(&self, method : &str) -> Option<Arc<dyn Handler>>{
        self.handlers.get(method).cloned()
    }
}

