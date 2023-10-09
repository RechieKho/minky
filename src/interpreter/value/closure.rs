use super::Value;
use super::table::Table;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use super::scope::Scope;
use crate::parser::command::Atom;
use crate::signal_no_loop_control;
use crate::{backtrace::Backtrace, raise_error};
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;

pub struct Closure {
    pub commands: Vec<Atom>,
    pub parent_scopes: Vec<Arc<Mutex<dyn Table>>>,
}

impl Closure {
    pub fn call_mut(&mut self, context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
        let mut slots: Vec<Value> = Vec::new();
        for atom in body.iter().skip(1) {
            let value = context.resolve_value(atom)?;
            slots.push(value);
        }

        let mut closure_context = Context::default();
        closure_context.slots = slots;
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes);
        let result = closure_context.run_commands(&self.commands, Scope::with_arc_mutex());
        mem::swap(&mut closure_context.scopes, &mut self.parent_scopes);
        let signal = result?;
        signal_no_loop_control!(signal);
        return Ok(signal);
    }

    pub fn wrap(commands: Vec<Atom>, parent_scopes: Vec<Arc<Mutex<dyn Table>>>) -> Value {
        let closure = Arc::new(Mutex::new(Closure {
            commands,
            parent_scopes,
        }));
        Value::CLOSURE(closure)
    }
}
