use crate::rail_machine::RailOp;

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        RailOp::new("drop", &["a"], &[], |state| {
            state.update_stack(|stack| stack.pop().1)
        }),
        RailOp::new("dup", &["a"], &["a", "a"], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                stack.push(a.clone()).push(a)
            })
        }),
        RailOp::new("swap", &["b", "a"], &["a", "b"], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                let (b, stack) = stack.pop();
                stack.push(a).push(b)
            })
        }),
        RailOp::new("rot", &["c", "b", "a"], &["a", "c", "b"], |state| {
            state.update_stack(|stack| {
                let (a, stack) = stack.pop();
                let (b, stack) = stack.pop();
                let (c, stack) = stack.pop();
                stack.push(a).push(c).push(b)
            })
        }),
    ]
}
