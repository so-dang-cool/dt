use crate::corelib::run_quot;
use crate::{RailOp, RailState, RailVal};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("call", &["quot"], &["..."], |state| {
            let mut stack = state.stack.clone();
            let rail_val = stack.pop().unwrap();
            let state = state.update_stack(stack);
            if let RailVal::Quotation(quot) = rail_val {
                run_quot(&quot, state)
            } else {
                panic!(
                    "call is only implemented for quotations, but got {:?}",
                    rail_val
                );
            }
        }),
        RailOp::new("def", &["quot", "s"], &[], |state| {
            let mut stack = state.stack;
            let name = match stack.pop().unwrap() {
                RailVal::String(name) => name,
                rail_val => panic!("def requires a string name, but got {:?}", rail_val),
            };
            let quot = match stack.pop().unwrap() {
                RailVal::Quotation(quot) => quot,
                rail_val => panic!("def requires a quotation, but got {:?}", rail_val),
            };
            let mut dictionary = state.dictionary;
            dictionary.insert(name.clone(), RailOp::from_quot(&name, quot));
            RailState {
                stack,
                dictionary,
                context: state.context,
            }
        }),
    ]
}
