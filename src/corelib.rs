use crate::rail_machine::Dictionary;
use std::collections::HashMap;

mod bool;
mod choice;
mod display;
mod function;
mod interact;
mod math;
mod process;
mod reflect;
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
        .chain(interact::builtins())
        .chain(math::builtins())
        .chain(process::builtins())
        .chain(reflect::builtins())
        .chain(repeat::builtins())
        .chain(shuffle::builtins())
        .chain(stack::builtins())
        .chain(string::builtins())
        .map(|op| (op.name.clone(), op));

    HashMap::from_iter(ops)
}
