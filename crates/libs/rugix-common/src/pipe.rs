//! In memory pipe.

use std::collections::VecDeque;
use std::io::{Read, Write};
use std::sync::{Arc, Condvar, Mutex};

use tracing::trace;

#[derive(Debug, Default)]
struct PipeState {
    buffer: VecDeque<u8>,
    writer_closed: bool,
    reader_closed: bool,
}

impl PipeState {
    pub fn free_space(&self) -> usize {
        self.buffer.capacity() - self.buffer.len()
    }
}

#[derive(Debug, Default)]
struct PipeShared {
    state: Mutex<PipeState>,
    condvar: Condvar,
}

#[derive(Debug)]
pub struct PipeReader {
    shared: Arc<PipeShared>,
}

impl Read for PipeReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut state = self.shared.state.lock().unwrap();
        loop {
            if state.buffer.is_empty() && !state.writer_closed {
                // Wait for the buffer to be filled or the writer to be closed.
                state = self.shared.condvar.wait(state).unwrap();
            } else {
                let size = buf.len().min(state.buffer.len());
                for i in 0..size {
                    buf[i] = state.buffer.pop_front().unwrap();
                }
                self.shared.condvar.notify_all();
                break Ok(size);
            }
        }
    }
}

impl Drop for PipeReader {
    fn drop(&mut self) {
        let mut state = self.shared.state.lock().unwrap();
        state.reader_closed = true;
        self.shared.condvar.notify_all();
        trace!("pipe reader closed");
    }
}

#[derive(Debug)]
pub struct PipeWriter {
    shared: Arc<PipeShared>,
}

impl Write for PipeWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut state = self.shared.state.lock().unwrap();
        let free_space = loop {
            if state.reader_closed {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "'PipeReader' has been closed",
                ));
            }
            let free_space = state.free_space();
            if free_space > 0 {
                break free_space;
            } else {
                // Wait for buffer space to become available.
                state = self.shared.condvar.wait(state).unwrap();
            }
        };
        let size = buf.len().min(free_space);
        state.buffer.extend(&buf[..size]);
        self.shared.condvar.notify_all();
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut state = self.shared.state.lock().unwrap();
        while !state.buffer.is_empty() {
            if state.reader_closed {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "'PipeReader' has been closed",
                ));
            }
            state = self.shared.condvar.wait(state).unwrap();
        }
        Ok(())
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
        let mut state = self.shared.state.lock().unwrap();
        state.writer_closed = true;
        self.shared.condvar.notify_all();
        trace!("pipe writer closed");
    }
}

pub fn buffered_pipe(buffer_size: usize) -> (PipeReader, PipeWriter) {
    let shared = Arc::new(PipeShared {
        state: Mutex::new(PipeState {
            buffer: VecDeque::with_capacity(buffer_size),
            writer_closed: false,
            reader_closed: false,
        }),
        condvar: Condvar::new(),
    });
    (
        PipeReader {
            shared: shared.clone(),
        },
        PipeWriter { shared },
    )
}
