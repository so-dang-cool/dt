use crate::{RailState, RailVal, Stack};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

mod bool;
mod choice;
mod display;
mod function;
mod math;
mod process;
mod repeat;
mod shuffle;
mod stack;
mod string;

pub fn new_dictionary() -> Dictionary {
    let ops = bool::builtins()
        .into_iter()
        .chain(choice::builtins())
        .chain(display::builtins())
        .chain(function::builtins())
        .chain(math::builtins())
        .chain(process::builtins())
        .chain(repeat::builtins())
        .chain(shuffle::builtins())
        .chain(stack::builtins())
        .chain(string::builtins())
        .map(|op| (op.name.clone(), op));

    HashMap::from_iter(ops)
}

fn run_quot(quot: &Stack, state: RailState) -> RailState {
    quot.terms.iter().fold(state, |state, rail_val| {
        let mut stack = state.stack.clone();
        match rail_val {
            RailVal::Operator(op) => {
                let mut op = op.clone();
                return op.go(state);
            }
            _ => stack.push(rail_val.clone()),
        }
        state.update_stack(stack)
    })
}

pub type Dictionary = HashMap<String, RailOp<'static>>;

#[derive(Clone)]
pub struct RailOp<'a> {
    pub name: String,
    consumes: &'a [&'a str],
    produces: &'a [&'a str],
    action: RailAction<'a>,
}

#[derive(Clone)]
pub enum RailAction<'a> {
    Builtin(Arc<dyn Fn(RailState) -> RailState + 'a>),
    Quotation(Stack),
}

impl RailOp<'_> {
    fn new<'a, F>(name: &str, consumes: &'a [&'a str], produces: &'a [&'a str], op: F) -> RailOp<'a>
    where
        F: Fn(RailState) -> RailState + 'a,
    {
        RailOp {
            name: name.to_string(),
            consumes,
            produces,
            action: RailAction::Builtin(Arc::new(op)),
        }
    }

    fn from_quot<'a>(name: &str, quot: Stack) -> RailOp<'a> {
        // TODO: Infer stack effects
        RailOp {
            name: name.to_string(),
            consumes: &[],
            produces: &[],
            action: RailAction::Quotation(quot),
        }
    }

    pub fn go(&mut self, state: RailState) -> RailState {
        if state.stack.len() < self.consumes.len() {
            // TODO: At some point will want source context here like line/column number.
            eprintln!(
                "Derailed: stack underflow for \"{}\" ({:?} -> {:?}): stack only had {}",
                self.name,
                self.consumes,
                self.produces,
                state.stack.len()
            );
            std::process::exit(1);
        }

        // TODO: Type checks

        match &self.action {
            RailAction::Builtin(op) => op(state),
            RailAction::Quotation(quot) => run_quot(quot, state),
        }
    }
}

impl Debug for RailOp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            ": {} ({:?} -- {:?}) ... ;",
            self.name, self.consumes, self.produces
        )
    }
}
