use crate::corelib::{run_quot, truthy};
use crate::{RailOp, RailVal, Stack};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![RailOp::new("opt", &["seq"], &[], |state| {
        let mut stack = state.stack.clone();

        // TODO: All conditions and all actions must have the same stack effect.
        let mut options = match stack.pop().unwrap() {
            RailVal::Quotation(seq) => seq,
            rail_val => panic!("opt requires a quotation, but got {:?}", rail_val),
        };

        let mut state = state.update_stack(stack);

        while !options.is_empty() {
            let action: Stack = match options.pop().unwrap() {
                RailVal::Quotation(action) => action,
                rail_val => panic!(
                    "each entry for opt requires a quotation of quotations, but got {:?}",
                    rail_val
                ),
            };
            let condition: Stack = match options.pop().unwrap() {
                RailVal::Quotation(action) => action,
                rail_val => panic!(
                    "each entry for opt requires a quotation of quotations, but got {:?}",
                    rail_val
                ),
            };
            state = run_quot(&condition, state);
            let mut stack = state.stack.clone();
            let success = truthy(stack.pop().unwrap());
            state = state.update_stack(stack);
            if success {
                state = run_quot(&action, state);
                break;
            }
        }

        state
    })]
}
