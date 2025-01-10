use std::collections::VecDeque;

use rugpi_cli::style::Stylize;
use rugpi_cli::widgets::{Heading, Text, Widget};
use rugpi_cli::{StatusSegment, VisualHeight};

#[derive(Debug, Default)]
pub struct CliLog {
    state: std::sync::Mutex<CliLogState>,
    title: String,
    line_limit: usize,
}

impl CliLog {
    pub fn new(title: String) -> Self {
        Self {
            state: std::sync::Mutex::default(),
            title,
            line_limit: 15,
        }
    }

    pub fn push_line(&self, line: String) {
        let mut state = self.state.lock().unwrap();
        state.lines.push_back(line);
        while state.lines.len() > self.line_limit {
            state.lines.pop_front();
        }
    }
}

#[derive(Debug, Default)]
struct CliLogState {
    lines: VecDeque<String>,
}

impl StatusSegment for CliLog {
    fn draw(&self, ctx: &mut rugpi_cli::DrawCtx) {
        Heading::new(&self.title).draw(ctx);
        let state = self.state.lock().unwrap();
        let show_lines = VisualHeight::from_usize(state.lines.len())
            .min(ctx.measure_remaining_height())
            .into_u64() as usize;
        let skip_lines = state.lines.len() - show_lines;
        Text::new(state.lines.iter().skip(skip_lines))
            .prefix("> ")
            .styled()
            .dark_gray()
            .draw(ctx);
    }
}
