pub mod qual_sem;
pub mod scope_sem;
pub mod ty_sem;
pub mod ty2_sem;
pub mod ty1_sem;
pub mod fn_sem;
mod macros;

use std::rc::Rc;
use anyhow::Result;
use super::context::SemContext;

pub trait Sem {
    fn logical_name(&self) -> String;
    fn description(&self) -> String;
}

pub trait SemVal<Key> {
    fn to_key(&self) -> Key;
}

pub trait SemKey<Val> {
    fn get_val(&self, ctx: &SemContext) -> Result<Rc<Val>>;
}

impl Sem for String {
    fn logical_name(&self) -> String {
        self.clone()
    }

    fn description(&self) -> String {
        self.clone()
    }
}
