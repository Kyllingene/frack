use std::fmt;
use std::ops::RangeInclusive;

pub mod util;

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

pub struct Error {
    pub error_code: String,
    pub message: String,
    pub file: File,

    pub code: Code,
    pub helps: Vec<Help>,
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

        let last = self.helps.is_empty() && self.notes.is_empty();
        self.code.display(last, f)?;

        for help in &self.helps {
            help.display(true, f)?;
        }

        for note in &self.notes {
            write!(f, "{note}")?;
        }

        Ok(())
    }
}

pub struct Warning {
    pub message: String,
    pub file: File,

    pub code: Code,
    pub helps: Vec<Help>,
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

        let last = self.helps.is_empty() && self.notes.is_empty();
        self.code.display(last, f)?;

        for help in &self.helps {
            help.display(true, f)?;
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

pub struct Code {
    pub code: String,
    pub line_number: usize,
    pub marker: Option<Marker>,
}

impl Code {
    pub fn line_number_width(&self) -> usize {
        width(self.line_number + self.code.chars().filter(|ch| *ch == '\n').count())
    }
}

impl Code {
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

                let (start, rest) = self.code.split_at(m.range.start().saturating_sub(1));
                let (mid, end) = rest.split_at(m.range.end().saturating_sub(*m.range.start()));

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

        if !extend && self.marker.is_some() {
            writeln!(f, "{: >width$}", " |", width = width + 2)?;
        }

        Ok(())
    }
}

pub struct Help {
    pub message: String,
    pub suggestion: Option<Code>,
}

impl Help {
    pub fn display(&self, last: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        bold(f)?;
        color(14, f)?;
        write!(f, "help")?;
        normal(f)?;

        writeln!(f, ": {}", self.message)?;

        if let Some(s) = &self.suggestion {
            s.display(last, f)?;
        }

        Ok(())
    }
}

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

pub struct Marker {
    pub range: RangeInclusive<usize>,
    pub symbol: char,
    pub color: u8,
    pub message: Option<String>,
    pub color_span: bool,
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        bold(f)?;
        color(self.color, f)?;

        let mark = Repeat(
            self.range.end().saturating_sub(*self.range.start()),
            self.symbol,
        );
        write!(
            f,
            "{: >start$}{mark}",
            "",
            start = self.range.start().saturating_sub(1),
        )?;

        if let Some(m) = &self.message {
            write!(f, " {m}")?;
        }

        normal(f)?;
        writeln!(f)
    }
}

pub struct File {
    pub path: String,
    pub line: usize,
    pub col: usize,
}
