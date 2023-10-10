use crate::assert_atoms_count_min;
use crate::atom_as_identifier;
use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::scope::Scope;
use crate::interpreter::variant::table::Table;
use crate::interpreter::variant::Variant;
use crate::mutex_lock_unwrap;
use crate::parser::atom::Atom;
use crate::parser::atom::AtomValue;
use crate::raise_error;

pub fn rep(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    assert_atoms_count_min!(body, 5);

    let first_atom = body.first().unwrap();

    let index_identifier = atom_as_identifier!(&body[1]);
    let start = context.resolve_number(&body[2])?;
    let end = context.resolve_number(&body[3])?;
    let commands = &body[5..];
    if commands.len() == 0 {
        return Ok(Signal::COMPLETE(Variant::NULL(Null())));
    }

    let step = context.resolve_number(&body[4])?;
    if step.is_sign_negative() {
        raise_error!(
            Some(first_atom.mark.clone()),
            "Step of repetition must be positive, system will increment/decrement to approach end value for you."
        );
    }

    let mut index = start;

    loop {
        let scope = Scope::wrap_arc_mutex();
        {
            let mut scope = mutex_lock_unwrap!(scope, Some(first_atom.mark.clone()));
            scope.insert(index_identifier.clone(), Variant::NUMBER(index));
        }

        let signal = context.run_statements(commands, scope)?;
        match signal {
            Signal::RETURN(_, _) => {
                return Ok(signal);
            }
            Signal::BREAK(_) => {
                break;
            }
            Signal::CONTINUE(_) => {
                continue;
            }
            _ => {}
        }

        if start < end {
            index += step;
            if index >= end {
                break;
            }
        } else if start > end {
            index -= step;
            if index <= end {
                break;
            }
        } else {
            break;
        }
    }

    Ok(Signal::COMPLETE(Variant::NULL(Null())))
}
