/// Byte range in the source.
use std::fmt;
pub type Span = core::ops::Range<usize>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SourceLocation {
    Range(Range),
    LineCol(LineCol),
    End,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LineCol {
    /// Line number (1-based) in case of Relative mode
    /// Start posision in case of absolute location.
    pub line: usize,
    /// Column number (1-based) in case of relative mode
    /// end position in case of absolute location.
    pub col: usize,
}

impl LineCol {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceLocationInfo {
    pub filename: String,
    pub location: SourceLocation,
}

impl SourceLocationInfo {
    pub fn new(line: usize, col: usize) -> Self {
        Self {
            location: SourceLocation::LineCol(LineCol::new(line, col)),
            filename: "noname.j2tpl".to_string(),
        }
    }
    pub fn new_at_the_end() -> Self {
        Self {
            filename: "noname.j2tpl".to_string(),
            location: SourceLocation::End,
        }
    }
    pub fn new_with_range(start: usize, end: usize) -> Self {
        Self {
            filename: "noname.j2tpl".to_string(),
            location: SourceLocation::Range(Range::new(start, end)),
        }
    }
    pub fn position_log(&self) -> String {
        match &self.location {
            SourceLocation::End => format!("{}:", self.filename),
            SourceLocation::LineCol(linecol) => {
                format!("{}:{}:{}:", self.filename, linecol.line, linecol.col)
            }
            SourceLocation::Range(range) => {
                format!("{}:{}-{}:", self.filename, range.start, range.end)
            }
        }
    }
    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }
}

impl fmt::Display for SourceLocationInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.position_log())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn size(&self) -> usize {
        self.end - self.start
    }

    pub fn span(&self) -> Span {
        self.start..self.end
    }
}
