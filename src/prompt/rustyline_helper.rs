use std::borrow::Cow;

use rustyline::completion::Completer;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::line_buffer::LineBuffer;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Context, Helper};

use crate::rail_machine::RailState;

pub struct RailTracks {
    state: RailState,
}

impl RailTracks {
    pub fn for_state(state: &RailState) -> RailTracks {
        RailTracks {
            state: state.clone(),
        }
    }
}

impl Helper for RailTracks {}

// TODO: Implement me
impl Completer for RailTracks {
    type Candidate = String;

    // TODO: Implement me
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let _ = (line, pos, ctx);
        Ok((0, Vec::with_capacity(0)))
    }

    // TODO: Implement me
    fn update(&self, line: &mut LineBuffer, start: usize, elected: &str) {
        let end = line.pos();
        line.replace(start..end, elected)
    }
}

// TODO: Implement me
impl Highlighter for RailTracks {
    // TODO: Implement me
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        let _ = pos;
        Cow::Borrowed(line)
    }

    // TODO: Implement me
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        let _ = default;
        Cow::Borrowed(prompt)
    }

    // TODO: Implement me
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Borrowed(hint)
    }

    // TODO: Implement me
    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str, // FIXME should be Completer::Candidate
        completion: rustyline::CompletionType,
    ) -> Cow<'c, str> {
        let _ = completion;
        Cow::Borrowed(candidate)
    }

    // TODO: Implement me
    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        let _ = (line, pos);
        false
    }
}

// TODO: Implement me
impl Hinter for RailTracks {
    type Hint = String;

    // TODO: Implement me
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<Self::Hint> {
        let _ = (line, pos, ctx);
        None
    }
}

// TODO: Implement me
impl Validator for RailTracks {
    // TODO: Implement me
    fn validate(&self, _ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }

    // TODO: Implement me
    fn validate_while_typing(&self) -> bool {
        false
    }
}
