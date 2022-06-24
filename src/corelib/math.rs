use crate::rail_machine::RailOp;

pub fn builtins() -> Vec<RailOp<'static>> {
    vec![
        unary_i64_op("abs", |a| a.abs()),
        binary_i64_op("+", |a, b| a + b),
        binary_i64_op("-", |a, b| a - b),
        binary_i64_op("*", |a, b| a * b),
        binary_i64_op("/", |a, b| a / b),
        binary_i64_op("%", |a, b| a % b),
        binary_i64_op("max", |a, b| if a >= b { a } else { b }),
        binary_i64_op("min", |a, b| if a <= b { a } else { b }),
    ]
}

fn unary_i64_op<'a, F>(name: &'a str, op: F) -> RailOp<'a>
where
    F: Fn(i64) -> i64 + Sized + 'a,
{
    RailOp::new(name, &["i64"], &["i64"], move |state| {
        let mut stack = state.stack.clone();
        let a = stack.pop_i64(name);
        stack.push_i64(op(a));
        state.update_stack(stack)
    })
}

fn binary_i64_op<'a, F>(name: &'a str, op: F) -> RailOp<'a>
where
    F: Fn(i64, i64) -> i64 + Sized + 'a,
{
    RailOp::new(name, &["i64", "i64"], &["i64"], move |state| {
        let mut stack = state.stack.clone();
        let b = stack.pop_i64(name);
        let a = stack.pop_i64(name);
        stack.push_i64(op(a, b));
        state.update_stack(stack)
    })
}
