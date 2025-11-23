//! Ready-made progress bar status segment.

use std::sync::Mutex;

use crate::widgets::{ProgressBar, Widget};
use crate::{DrawCtx, StatusSegment, StatusSegmentRef};

/// Progress bar status segment.
#[derive(Debug)]
pub struct ProgressBarSegment {
    state: Mutex<ProgressBarState>,
}

impl ProgressBarSegment {
    pub fn new(total_steps: u64) -> StatusSegmentRef<Self> {
        crate::add_status(Self {
            state: Mutex::new(ProgressBarState {
                current_step: 0,
                total_steps,
            }),
        })
    }

    pub fn set_step(&self, step: u64) {
        self.state.lock().unwrap().current_step = step;
    }
}

#[derive(Debug)]
struct ProgressBarState {
    current_step: u64,
    total_steps: u64,
}

impl StatusSegment for ProgressBarSegment {
    fn draw(&self, ctx: &mut DrawCtx) {
        let state = self.state.lock().unwrap();
        ProgressBar::new(state.current_step, state.total_steps).draw(ctx);
    }
}
