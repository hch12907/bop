//! buffer manipulation, storing and manipulating long string with
//! [Rope](https://en.wikipedia.org/wiki/Rope_(data_structure)).

// std dependencies
use std::fs::File;
use std::io::{Stdin, Read};
use std::path::PathBuf;

// dependencies
use rope::Rope;
// use tempdir::TempDir;

/// Buffer stores the content of the files which is loaded into bop
/// In this case, we use rope to insert and delete text efficiently
pub struct Buffer {
    /// text buffer
    buff: Rope,
    /// File path on disk
    path: Option<PathBuf>,
    /// Neme of the buffer
    name: String,
}

impl Buffer {
    /// Create a new, empty `Buffer`.
    pub fn new() -> Buffer {
        Buffer {
            buff: Rope::new(),
            path: None,
            name: String::new(),
        }
    }
}
