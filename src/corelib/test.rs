use colored::Colorize;

use crate::dt_machine::{Definition, DtType};

use DtType::*;

// TODO: More forms, optional messages, etc. Input as stab? Output as stab or quote of failures?
pub fn builtins() -> Vec<Definition<'static>> {
    vec![Definition::on_state(
        "assert-true",
        &[Boolean, String],
        &[],
        |quote| {
            let (msg, quote) = quote.pop_string("assert-true");
            let (b, quote) = quote.pop_bool("assert-true");

            if !b {
                let msg = format!("Assertion failed: {}", msg).red();
                eprintln!("{}", msg);
                std::process::exit(1);
            }

            quote
        },
    )]
}
