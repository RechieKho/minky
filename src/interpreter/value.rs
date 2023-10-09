pub mod closure;
pub mod scope;
pub mod table;

use super::context::Context;
use super::signal::Signal;
use crate::backtrace::Backtrace;
use crate::parser::command::Atom;
use closure::Closure;
use table::Table;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

#[macro_export]
macro_rules! mutex_lock_unwrap {
    ($mutex:expr, $mark:expr) => {
        match $mutex.lock() {
            Ok(guard) => guard,
            Err(_) => {
                use crate::raise_bug;
                raise_bug!(Some($mark), "Thread is poisoned while locking mutex.");
            }
        }
    };
}

#[derive(Clone)]
pub enum Value {
    NULL,
    BOOL(bool),
    NUMBER(f64),
    STRING(String),
    LIST(Vec<Value>),
    TABLE(Arc<Mutex<dyn Table>>),
    FUNCTION(Arc<dyn Fn(&mut Context, &[Atom]) -> Result<Signal, Backtrace>>),
    CLOSURE(Arc<Mutex<Closure>>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::NULL => f.write_str("NULL"),
            Value::BOOL(boolean) => f.write_fmt(format_args!("{:?}", boolean)),
            Value::NUMBER(number) => f.write_fmt(format_args!("{:?}", number)),
            Value::STRING(string) => f.write_str(string),
            Value::LIST(list) => f.write_fmt(format_args!(
                "[{}]",
                list.iter()
                    .map(|x| if let Value::STRING(string) = x {
                        format!("\"{:?}\"", string)
                    } else {
                        format!("{:?}", x)
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            Value::TABLE(_) => f.write_str("<Table>"),
            Value::FUNCTION(_) => f.write_str("<Function>"),
            Value::CLOSURE(_) => f.write_str("<Closure>"),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::NULL
    }
}
