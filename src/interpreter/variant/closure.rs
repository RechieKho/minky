use super::represent::Represent;
use super::table::Table;
use super::variant_ops::{VariantAdd, VariantDiv, VariantMul, VariantSub};
use super::Variant;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::mark::Mark;
use crate::parser::atom::Atom;
use crate::raise_error;
use std::fmt::Debug;
use std::mem;

#[derive(Clone)]
pub struct Closure {
    pub mark: Mark,
    pub commands: Vec<Atom>,
    pub parent_scopes: Vec<Table>,
}

impl VariantAdd for Closure {
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

impl VariantSub for Closure {
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

impl VariantMul for Closure {
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

impl VariantDiv for Closure {
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

impl Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("closure") // TODO
    }
}

impl Represent for Closure {
    fn represent(&self, _mark: Option<Mark>) -> Result<String, Backtrace> {
        Ok(String::from("closure")) // TODO
    }
}

impl Closure {
    pub fn call_mut(&mut self, context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
        let mut slots: Vec<Variant> = Vec::new();
        for atom in body.iter() {
            let value = context.resolve_variant(atom)?;
            slots.push(value);
        }
        let mut closure_context = Context::default();
        closure_context.slots = slots;
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes); // Install parent scopes into the context.
        let result = closure_context.run_statements(&self.commands, Table::default());
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes); // Retrieve parent scopes back.
        result
    }

    pub fn new(mark: Mark, commands: Vec<Atom>, parent_scopes: Vec<Table>) -> Self {
        Closure {
            mark,
            commands,
            parent_scopes,
        }
    }
}
