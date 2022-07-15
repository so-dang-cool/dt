use crate::rail_machine::RailDef;

pub fn builtins() -> Vec<RailDef<'static>> {
    vec![
        RailDef::on_quote("drop", &["a"], &[], |quote| quote.pop().1),
        RailDef::on_quote("dup", &["a"], &["a", "a"], |quote| {
            let (a, quote) = quote.pop();
            quote.push(a.clone()).push(a)
        }),
        RailDef::on_quote("dup2", &["a", "b"], &["a", "b", "a", "b"], |quote| {
            let (b, quote) = quote.pop();
            let (a, quote) = quote.pop();
            quote.push(a.clone()).push(b.clone()).push(a).push(b)
        }),
        RailDef::on_quote("swap", &["a", "b"], &["b", "a"], |quote| {
            let (a, quote) = quote.pop();
            let (b, quote) = quote.pop();
            quote.push(a).push(b)
        }),
        RailDef::on_quote("rot", &["a", "b", "c"], &["c", "a", "b"], |quote| {
            let (a, quote) = quote.pop();
            let (b, quote) = quote.pop();
            let (c, quote) = quote.pop();
            quote.push(a).push(c).push(b)
        }),
    ]
}
