use crate::rail_machine::Dictionary;
use std::collections::HashMap;

mod bool;
mod choice;
mod command;
mod display;
mod filesystem;
mod math;
mod process;
mod reflect;
mod repeat;
mod sequence;
mod shuffle;
mod string;

pub fn new_dictionary() -> Dictionary {
    let ops = bool::builtins()
        .into_iter()
        .chain(choice::builtins())
        .chain(command::builtins())
        .chain(display::builtins())
        .chain(filesystem::builtins())
        .chain(math::builtins())
        .chain(process::builtins())
        .chain(reflect::builtins())
        .chain(repeat::builtins())
        .chain(shuffle::builtins())
        .chain(sequence::builtins())
        .chain(string::builtins())
        .map(|op| (op.name.clone(), op));

    HashMap::from_iter(ops)
}
