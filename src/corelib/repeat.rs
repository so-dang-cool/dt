use crate::{RailOp, RailVal};

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("times", &["quot", "i64"], &[], |state| {
            let mut stack = state.stack.clone();
            let n = match stack.pop().unwrap() {
                RailVal::I64(n) => n,
                rail_val => panic!("def requires an integer, but got {:?}", rail_val),
            };
            let quot = match stack.pop().unwrap() {
                RailVal::Quotation(quot) => quot,
                rail_val => panic!("def requires a quotation, but got {:?}", rail_val),
            };
            (0..n).fold(state.update_stack(stack), |state, _n| {
                run_quot(&quot, state)
            })
        }),
    ]
}