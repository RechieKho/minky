use super::represent::Represent;
use super::variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub};
use crate::backtrace::Backtrace;
use crate::interpreter::variant::Variant;
use crate::mark::Mark;
use crate::{mutex_lock_unwrap, raise_error};
use hashbrown::HashMap;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Table(Arc<Mutex<HashMap<String, Variant>>>);

impl Default for Table {
    fn default() -> Self {
        Table(Arc::new(Mutex::new(HashMap::default())))
    }
}

impl VariantAdd for Table {
    fn add(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be added with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl VariantSub for Table {
    fn sub(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be subtracted with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl VariantMul for Table {
    fn mul(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be multiplied with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl VariantDiv for Table {
    fn div(&self, rhs: &Variant, mark: Option<Mark>) -> Result<Variant, Backtrace> {
        match rhs {
            _ => {
                raise_error!(
                    mark.clone(),
                    "`{}` cannot be divided with `{}`.",
                    self.represent(mark.clone())?,
                    rhs.represent(mark.clone())?
                );
            }
        }
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("table") // TODO
    }
}

impl Represent for Table {
    fn represent(&self, _mark: Option<Mark>) -> Result<String, Backtrace> {
        Ok(String::from("table")) // TODO
    }
}

impl Table {
    pub fn insert(
        &mut self,
        key: String,
        value: Variant,
        mark: Option<Mark>,
    ) -> Result<Option<Variant>, Backtrace> {
        let mut guard = mutex_lock_unwrap!(self.0, mark);
        Ok(guard.insert(key, value))
    }

    pub fn get(&self, key: &String, mark: Option<Mark>) -> Result<Option<Variant>, Backtrace> {
        let guard = mutex_lock_unwrap!(self.0, mark);
        let variant = guard.get(key);
        Ok(if variant.is_none() {
            None
        } else {
            Some(variant.unwrap().clone())
        })
    }

    pub fn contains_key(&self, key: &String, mark: Option<Mark>) -> Result<bool, Backtrace> {
        let guard = mutex_lock_unwrap!(self.0, mark);
        Ok(guard.contains_key(key))
    }
}
