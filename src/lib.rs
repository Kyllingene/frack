//! Utilities for creating `rustc`-like error messages.
//!
//! Note that all implementations of [`Display`](fmt::Display), as well as
//! methods named `display`, utilize ANSI escape sequences. There's currently no
//! way to change this.

use std::fmt;
use std::ops::RangeInclusive;

mod util;

/// An error in `rustc` style.
///
/// To display using ANSI escape codes, use the [`Display`](fmt::Display) impl.
pub struct Error {
    /// The `E0502` in `error[E0502]: ...`.
    pub error_code: String,

    /// The message to display after `error[...]: `.
    pub message: String,

    /// The file the error is in.
    pub file: File,

    /// The code the error is about.
    pub code: Code,

    /// Any number of help messages.
    ///
    /// These get displayed before the notes.
    pub helps: Vec<Help>,

    /// Any number of notes.
    ///
    /// These get displayed after the help messages.
    pub notes: Vec<Note>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        bold(f)?;
        color(9, f)?;
        write!(f, "error[{}]", self.error_code)?;
        normal(f)?;

        bold(f)?;
        writeln!(f, ": {}", self.message)?;

        color(12, f)?;
        write!(f, " --> ")?;
        normal(f)?;
        writeln!(f, "{}:{}:{}", self.file.path, self.file.line, self.file.col)?;

        self.code.display(true, f)?;

        for help in &self.helps {
            help.display(false, f)?;
        }

        for note in &self.notes {
            write!(f, "{note}")?;
        }

        Ok(())
    }
}

/// A warning in `rustc` style.
///
/// To display using ANSI escape codes, use the [`Display`](fmt::Display) impl.
pub struct Warning {
    /// The message to display after `warning: `.
    pub message: String,

    /// The file the warning is in.
    pub file: File,

    /// The code the warning is about.
    pub code: Code,

    /// Any number of help messages.
    ///
    /// These get displayed before the notes.
    pub helps: Vec<Help>,

    /// Any number of notes.
    ///
    /// These get displayed after the help messages.
    pub notes: Vec<Note>,
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        bold(f)?;
        color(3, f)?;
        write!(f, "warning")?;
        normal(f)?;

        bold(f)?;
        writeln!(f, ": {}", self.message)?;

        color(12, f)?;
        write!(f, " --> ")?;
        normal(f)?;
        writeln!(f, "{}:{}:{}", self.file.path, self.file.line, self.file.col)?;

        self.code.display(true, f)?;

        for help in &self.helps {
            help.display(false, f)?;
        }

        for note in &self.notes {
            bold(f)?;
            color(12, f)?;
            write!(
                f,
                "{: >width$}",
                " = ",
                width = self.code.line_number_width() + 3
            )?;
            normal(f)?;

            write!(f, "{note}")?;
        }

        Ok(())
    }
}

/// A code block for a [`Help`], [`Warning`], or [`Error`].
pub struct Code {
    /// A single line of code.
    ///
    /// If this is multiple lines, the output *will* be degraded.
    pub code: String,

    /// The line number of the line of code.
    pub line_number: usize,

    /// An underline to apply to the line of code.
    pub marker: Option<Marker>,
}

impl Code {
    /// Returns the number of digits in the line number of the code.
    pub fn line_number_width(&self) -> usize {
        width(self.line_number)
    }

    /// Write out with ANSI escape codes. Behaves like an impl for
    /// [`Display`](fmt::Display).
    ///
    /// If `extend == false` and this code has a [marker](Marker), prints out an
    /// extra line; to mimic `rustc`, `extend` should be true iff it's the main
    /// code block for the warning/error.
    pub fn display(&self, extend: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.line_number_width();

        bold(f)?;
        color(12, f)?;
        writeln!(f, "{: >width$}", " |", width = width + 2)?;
        normal(f)?;

        let mut line = || {
            bold(f)?;
            color(12, f)?;
            write!(f, "{: <width$} | ", self.line_number, width = width)?;
            normal(f)?;
            writeln!(f, "{}", self.code)?;

            Ok(())
        };

        if let Some(m) = &self.marker {
            if m.color_span && *m.range.end() <= self.code.len() {
                bold(f)?;
                color(12, f)?;
                write!(f, "{: <width$} | ", self.line_number, width = width)?;
                normal(f)?;

                let (start, rest) = self.code.split_at(*m.range.start());
                let (mid, end) = rest.split_at(m.range.end().saturating_sub(*m.range.start()) + 1);

                write!(f, "{start}")?;

                color(m.color, f)?;
                write!(f, "{mid}")?;

                normal(f)?;
                writeln!(f, "{end}")?;
            } else {
                line()?;
            }

            bold(f)?;
            color(12, f)?;
            write!(f, "{: >width$}", " | ", width = width + 3)?;
            normal(f)?;
            write!(f, "{m}")?;
        } else {
            line()?;
        }

        if extend || self.marker.is_none() {
            writeln!(f, "{: >width$}", " |", width = width + 2)?;
        }

        Ok(())
    }
}

/// A help message for an [`Warning`] or [`Error`].
pub struct Help {
    pub message: String,

    /// A suggested revision.
    pub suggestion: Option<Code>,
}

impl Help {
    /// Write out with ANSI escape codes. Behaves like an impl for
    /// [`Display`](fmt::Display).
    ///
    /// If `extend == false` and this code has a [marker](Marker), prints out an
    /// extra line; to mimic `rustc`, `extend` should be true iff it's the main
    /// code block for the warning/error.
    pub fn display(&self, extend: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        bold(f)?;
        color(14, f)?;
        write!(f, "help")?;
        normal(f)?;

        writeln!(f, ": {}", self.message)?;

        if let Some(s) = &self.suggestion {
            s.display(extend, f)?;
        }

        Ok(())
    }
}

/// A note for a [`Warning`] or [`Error`].
pub struct Note(pub String);

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        bold(f)?;
        write!(f, "note")?;

        normal(f)?;
        writeln!(f, ": {}", self.0)?;

        Ok(())
    }
}

/// An underline for a piece of [`Code`].
pub struct Marker {
    /// The code the marker should underline.
    pub range: RangeInclusive<usize>,

    /// The symbol to underline with.
    ///
    /// `rustc` uses `~` for modifications,
    /// and `^` for everything else.
    pub symbol: char,

    /// The color of the underline, as an
    /// [ANSI escape color](https://wikipedia.org/wiki/ANSI_escape_code#8-bit).
    ///
    /// `rustc` uses `10` for modifications, `3` for warnings, and `9` for errors.
    pub color: u8,

    /// A message to display after the underline, in the same color.
    pub message: Option<String>,

    /// Whether or not to apply the coloring to the underlined code.
    ///
    /// `rustc` only does this for modifications.
    pub color_span: bool,
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        bold(f)?;
        color(self.color, f)?;

        let mark = Repeat(
            self.range.end().saturating_sub(*self.range.start()) + 1,
            self.symbol,
        );
        write!(f, "{: >start$}{mark}", "", start = self.range.start(),)?;

        if let Some(m) = &self.message {
            write!(f, " {m}")?;
        }

        normal(f)?;
        writeln!(f)
    }
}

/// The path, line, and column of a piece of [`Code`].
pub struct File {
    pub path: String,
    pub line: usize,
    pub col: usize,
}

fn color(c: u8, f: &mut impl fmt::Write) -> fmt::Result {
    write!(f, "\x1b[38;5;{c}m")
}

fn bold(f: &mut impl fmt::Write) -> fmt::Result {
    write!(f, "\x1b[1m")
}

fn normal(f: &mut impl fmt::Write) -> fmt::Result {
    write!(f, "\x1b[0m")
}

fn width(x: usize) -> usize {
    x.checked_ilog10().unwrap_or(1) as usize + 1
}

struct Repeat<T>(usize, T);
impl<T: fmt::Display> fmt::Display for Repeat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.0 {
            write!(f, "{}", self.1)?;
        }
        Ok(())
    }
}
