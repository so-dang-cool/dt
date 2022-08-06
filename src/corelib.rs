use crate::rail_machine::{self, Dictionary};

mod bool;
mod choice;
mod command;
mod display;
mod filesystem;
mod math;
mod meta;
mod process;
mod repeat;
mod sequence;
mod shuffle;
mod stab;
mod string;
mod test;

pub fn corelib_dictionary() -> Dictionary {
    rail_machine::dictionary_of(
        bool::builtins()
            .into_iter()
            .chain(choice::builtins())
            .chain(command::builtins())
            .chain(display::builtins())
            .chain(filesystem::builtins())
            .chain(math::builtins())
            .chain(meta::builtins())
            .chain(process::builtins())
            .chain(repeat::builtins())
            .chain(shuffle::builtins())
            .chain(sequence::builtins())
            .chain(stab::builtins())
            .chain(string::builtins())
            .chain(test::builtins())
            .map(|op| (op.name.clone(), op)),
    )
}
