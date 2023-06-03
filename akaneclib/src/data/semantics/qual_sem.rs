use std::rc::Rc;
use anyhow::Result;
use crate::data::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QualSem {
    pub id: usize,
    pub scopes: Vec<ScopeSem>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QualKey {
    pub scopes: Vec<ScopeSem>,
}

impl Sem for QualSem {
    fn logical_name(&self) -> String {
        self.to_key().logical_name()
    }

    fn description(&self) -> String {
        self.to_key().description()
    }
}

impl SemVal<QualKey> for QualSem {
    fn to_key(&self) -> QualKey {
        QualKey {
            scopes: self.scopes.clone(),
        }
    }
}

impl Sem for QualKey {
    fn logical_name(&self) -> String {
        self.qualify_logical_name_self(".")
    }

    fn description(&self) -> String {
        self.qualify_description_self(".")
    }
}

impl SemKey<QualSem> for QualKey {
    fn get_val(&self, ctx: &SemContext) -> Result<Rc<QualSem>> {
        ctx.qual_store.get(self)
    }
}

impl QualSem {
    pub fn top(ctx: &mut SemContext) -> Rc<Self> {
        QualSem::new_or_get(ctx, &QualKey::top())
    }

    fn new_or_get_one(ctx: &mut SemContext, scopes: Vec<ScopeSem>) -> Rc<Self> {
        let value = Rc::new(Self {
            id: ctx.qual_store.next_id(),
            scopes,
        });
        let key = value.to_key();
        ctx.qual_store.insert_or_get(key, value)
    }

    pub fn new_or_get(ctx: &mut SemContext, key: &QualKey) -> Rc<Self> {
        for n in 1..key.scopes.len() {
            Self::new_or_get_one(ctx, key.scopes.iter().take(n).cloned().collect());
        }
        Self::new_or_get_one(ctx, key.scopes.clone())
    }
}

impl QualKey {
    pub fn top() -> Self {
        Self::new(Vec::new())
    }

    pub fn new(scopes: Vec<ScopeSem>) -> Self {
        Self { scopes }
    }

    pub fn pushed(&self, scope: ScopeSem) -> Self {
        let mut cloned = self.scopes.clone();
        cloned.push(scope);
        Self::new(cloned)
    }

    pub fn qualify_logical_name_self(&self, sep: &str) -> String {
        self.scopes.iter().map(|x| x.logical_name()).collect::<Vec<_>>().join(sep)
    }

    pub fn qualify_logical_name(&self, sep: &str) -> String {
        let q = self.qualify_logical_name_self(sep);
        if q.len() == 0 { q } else { q + sep }
    }

    pub fn qualify_description_self(&self, sep: &str) -> String {
        self.scopes.iter().map(|x| x.description()).collect::<Vec<_>>().join(sep)
    }

    pub fn qualify_description(&self, sep: &str) -> String {
        let q = self.qualify_description_self(sep);
        if q.len() == 0 { q } else { q + sep }
    }
}
