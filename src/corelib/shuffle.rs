use crate::rail_machine::RailDef;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_stack("drop", &["a"], &[], |stack| stack.pop().1),
        RailDef::on_stack("dup", &["a"], &["a", "a"], |stack| {
            let (a, stack) = stack.pop();
            stack.push(a.clone()).push(a)
        }),
        RailDef::on_stack("swap", &["b", "a"], &["a", "b"], |stack| {
            let (a, stack) = stack.pop();
            let (b, stack) = stack.pop();
            stack.push(a).push(b)
        }),
        RailDef::on_stack("rot", &["c", "b", "a"], &["a", "c", "b"], |stack| {
            let (a, stack) = stack.pop();
            let (b, stack) = stack.pop();
            let (c, stack) = stack.pop();
            stack.push(a).push(c).push(b)
        }),
    ]
}
