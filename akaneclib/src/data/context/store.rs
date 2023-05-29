use std::{
    collections::HashMap,
    hash::Hash,
    rc::Rc,
};
use anyhow::{
    bail,
    Result,
};
use crate::data::semantics::Sem;

pub struct Store<Key, Val>
where
    Key: Clone + Eq + Hash + Sem,
{
    id_map: HashMap<Key, usize>,
    vals: Vec<Rc<Val>>,
}

impl<Key, Val> Store<Key, Val>
where
    Key: Clone + Eq + Hash + Sem,
{
    pub fn new() -> Self {
        Self {
            id_map: HashMap::new(),
            vals: Vec::new(),
        }
    }

    pub fn next_id(&self) -> usize {
        self.vals.len()
    }

    pub fn insert(&mut self, key: Key, val: Rc<Val>) -> Result<Rc<Val>> {
        let id = self.next_id();
        if self.id_map.contains_key(&key) {
            bail!(format!("Registration duplicated: `{}`", key.description()))
        }
        self.id_map.insert(key, id);
        self.vals.push(val.clone());
        Ok(val)
    }

    pub fn insert_or_get(&mut self, key: Key, val: Rc<Val>) -> Rc<Val> {
        let id = self.next_id();
        if self.id_map.contains_key(&key) {
            self.id_map.get(&key).map(|id| self.vals[*id].clone()).unwrap()
        }
        else {
            self.id_map.insert(key, id);
            self.vals.push(val.clone());
            val
        }
    }

    pub fn force_insert(&mut self, key: Key, val: Rc<Val>) -> Rc<Val> {
        let id = self.next_id();
        self.id_map.insert(key, id);
        self.vals.push(val.clone());
        val
    }

    pub fn get(&self, key: &Key) -> Result<Rc<Val>> {
        match self.id_map.get(key).map(|id| self.vals[*id].clone()) {
            Some(val) => Ok(val),
            None => bail!(format!("Key not found: `{}`", key.description())),
        }
    }

    pub fn vals<'a>(&'a self) -> impl Iterator<Item = Rc<Val>> + 'a {
        self.vals.iter().cloned()
    }
}
