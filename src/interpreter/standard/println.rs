use crate::backtrace::Backtrace;
use crate::interpreter::context::Context;
use crate::interpreter::signal::Signal;
use crate::interpreter::variant::null::Null;
use crate::interpreter::variant::represent::Represent;
use crate::interpreter::variant::Variant;
use crate::parser::atom::Atom;

pub fn println(context: &mut Context, body: &[Atom]) -> Result<Signal, Backtrace> {
    for atom in body.iter().skip(1) {
        let value = context.resolve_variant(atom)?;
        print!("{}", value.represent()?);
    }
    print!("\n");
    Ok(Signal::COMPLETE(Variant::NULL(Null())))
}