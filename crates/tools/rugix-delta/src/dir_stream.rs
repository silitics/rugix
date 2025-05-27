//! A [`DirStream`] provides a deterministic, sequential view on a directory structure.
//!
//! [`DirStream`] represents a directory structure as a sequence fo *entries* ordered by
//! their path. Entries are represented by [`DirEntry`] and [`DirStream`] allows moving
//! through entries forward and backward as well as to seek to arbitrary entries.
//!
//! Being able to represent a directory structure as a sequence is at the core of the
//! Deltar algorithm.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use tracing::{Level, trace};

/// Sequential view on a directory structure.
#[derive(Debug, Clone)]
pub struct DirStream {
    /// Root directory.
    root: PathBuf,
    /// Stack used to move through the sequence.
    stack: Vec<DirStreamStackItem>,
    /// Current path.
    path: PathBuf,
    /// Current position.
    position: PathBuf,
}

impl DirStream {
    /// Creates a new [`DirStream`] for the given path.
    pub fn new(path: &Path) -> Self {
        let root = path.to_path_buf();
        let stack = vec![DirStreamStackItem::from_path(path)];
        let path = root.clone();
        let position = PathBuf::new();
        Self {
            root,
            stack,
            path,
            position,
        }
    }

    pub fn current_position(&self) -> &Path {
        &self.position
    }

    pub fn current_path(&self) -> &Path {
        &self.path
    }

    /// Seek to a specific path.
    pub fn seek_to(&mut self, path: &Path) -> bool {
        self.path = self.root.clone();
        self.position = PathBuf::new();
        self.stack = vec![DirStreamStackItem::from_path(&self.path)];
        for component in path.components() {
            let component = component.as_os_str();
            let top = self.stack.last_mut().unwrap();
            let entry_idx = top.entries.partition_point(|e| e.filename < component);
            if entry_idx > top.entries.len() - 1 {
                top.current = top.entries.len();
                return self.next();
            } else {
                top.current = entry_idx + 1;
                let current = top.current().unwrap();
                let exact_match = current.filename == component;
                self.path.push(&current.filename);
                self.position.push(&current.filename);
                if matches!(current.kind, DirEntryKind::Directory) {
                    self.stack.push(DirStreamStackItem::from_path(&self.path));
                }
                if !exact_match {
                    break;
                }
            }
        }
        true
    }

    /// Move forward to the next entry.
    ///
    /// Returns [`false`] iff the end has been reached.
    pub fn next(&mut self) -> bool {
        let Some((idx, _)) = self
            .stack
            .iter()
            .enumerate()
            .rev()
            .find(|(_, i)| i.can_move_forward())
        else {
            trace!(?self.path, "reached end of directory stream");
            return false;
        };
        // Truncate the stack.
        for _ in 0..self.stack.len() - idx - 1 {
            trace!(?self.path, "popping last component from path");
            self.path.pop();
            self.position.pop();
        }
        self.stack.truncate(idx + 1);
        if self.stack[idx].current().is_some() {
            trace!(?self.path, current=?self.stack[idx].current(), "popping current component from path");
            self.path.pop();
            self.position.pop();
        }
        self.stack[idx].move_forward();
        let top = &self.stack[idx];
        let current = top.current().unwrap();
        self.path.push(&current.filename);
        self.position.push(&current.filename);
        if matches!(current.kind, DirEntryKind::Directory) {
            let item = DirStreamStackItem::from_path(&self.path);
            if !item.entries.is_empty() {
                self.stack.push(item);
            }
        }
        true
    }
}

///
/// Parent
/// Entry A
/// Entry B
/// ...
#[derive(Debug, Clone)]
struct DirStreamStackItem {
    entries: Vec<DirEntry>,
    current: usize,
}

impl DirStreamStackItem {
    pub fn from_path(path: &Path) -> Self {
        let mut entries = std::fs::read_dir(path)
            .unwrap_or_else(|error| panic!("Failed to read directory: {:?}: {error}", path))
            .filter_map(Result::ok)
            .map(|entry| {
                let file_type = entry.file_type().unwrap();
                DirEntry {
                    filename: entry.file_name(),
                    kind: if file_type.is_file() {
                        DirEntryKind::File
                    } else if file_type.is_dir() {
                        DirEntryKind::Directory
                    } else {
                        DirEntryKind::Other
                    },
                }
            })
            .collect::<Vec<_>>();
        entries.sort_by(|a, b| a.filename.cmp(&b.filename));
        Self {
            entries,
            current: 0,
        }
    }

    pub fn is_parent(&self) -> bool {
        self.current == 0
    }

    fn current(&self) -> Option<&DirEntry> {
        self.current
            .checked_sub(1)
            .and_then(|idx| self.entries.get(idx))
    }

    pub fn can_move_forward(&self) -> bool {
        self.current < self.entries.len()
    }

    fn move_forward(&mut self) {
        assert!(self.can_move_forward());
        self.current += 1;
    }
}

#[derive(Debug, Clone)]
pub struct DirEntry {
    filename: OsString,
    kind: DirEntryKind,
}

impl DirEntry {
    pub fn filename(&self) -> &OsStr {
        &self.filename
    }
}

#[derive(Debug, Clone)]
enum DirEntryKind {
    Directory,
    File,
    Other,
}
