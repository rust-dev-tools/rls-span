// Copyright 2016 The RLS Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(proc_macro)]

extern crate serde;
#[macro_use]
extern crate serde_derive;

use serde::{Serialize, Deserialize};

use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Column<I: Indexed>(pub u32, PhantomData<I>);

impl<I: Indexed> Column<I> {
    pub fn new(c: u32) -> Column<I> {
        Column(c, PhantomData)
    }
}

impl<I: Indexed> Clone for Column<I> {
    fn clone(&self) -> Column<I> {
        *self
    }
}

impl<I: Indexed> Copy for Column<I> {}

impl Column<OneIndexed> {
    pub fn zero_indexed(self) -> Column<ZeroIndexed> {
        Column(self.0 - 1, PhantomData)
    }
}

impl Column<ZeroIndexed> {
    pub fn one_indexed(self) -> Column<OneIndexed> {
        Column(self.0 + 1, PhantomData)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Row<I: Indexed>(pub u32, PhantomData<I>);

impl<I: Indexed> Row<I> {
    pub fn new(c: u32) -> Row<I> {
        Row(c, PhantomData)
    }
}

impl<I: Indexed> Clone for Row<I> {
    fn clone(&self) -> Row<I> {
        *self
    }
}

impl<I: Indexed> Copy for Row<I> {}

impl Row<OneIndexed> {
    pub fn zero_indexed(self) -> Row<ZeroIndexed> {
        Row(self.0 - 1, PhantomData)
    }
}

impl Row<ZeroIndexed> {
    pub fn one_indexed(self) -> Row<OneIndexed> {
        Row(self.0 + 1, PhantomData)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Position<I: Indexed> {
    pub row: Row<I>,
    pub col: Column<I>,
}

impl<I: Indexed> Position<I> {
    pub fn new(row: Row<I>,
               col: Column<I>)
               -> Position<I> {
        Position {
            row: row,
            col: col,
        }
    }
}

impl<I: Indexed> Clone for Position<I> {
    fn clone(&self) -> Position<I> {
        *self
    }
}

impl<I: Indexed> Copy for Position<I> {}

impl Position<OneIndexed> {
    pub fn zero_indexed(self) -> Position<ZeroIndexed> {
        Position {
            row: self.row.zero_indexed(),
            col: self.col.zero_indexed(),
        }
    }
}

impl Position<ZeroIndexed> {
    pub fn one_indexed(self) -> Position<OneIndexed> {
        Position {
            row: self.row.one_indexed(),
            col: self.col.one_indexed(),
        }
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Range<I: Indexed> {
    pub row_start: Row<I>,
    pub row_end: Row<I>,
    pub col_start: Column<I>,
    pub col_end: Column<I>,
}

impl<I: Indexed> Range<I> {
    pub fn new(row_start: Row<I>,
               row_end: Row<I>,
               col_start: Column<I>,
               col_end: Column<I>)
               -> Range<I> {
        Range {
            row_start: row_start,
            row_end: row_end,
            col_start: col_start,
            col_end: col_end,
        }
    }

    pub fn from_positions(start: Position<I>,
                          end: Position<I>)
                          -> Range<I> {
        Range {
            row_start: start.row,
            row_end: end.row,
            col_start: start.col,
            col_end: start.col,
        }
    }
}

impl<I: Indexed> Clone for Range<I> {
    fn clone(&self) -> Range<I> {
        *self
    }
}

impl<I: Indexed> Copy for Range<I> {}

impl Range<OneIndexed> {
    pub fn zero_indexed(self) -> Range<ZeroIndexed> {
        Range {
            row_start: self.row_start.zero_indexed(),
            row_end: self.row_end.zero_indexed(),
            col_start: self.col_start.zero_indexed(),
            col_end: self.col_end.zero_indexed(),
        }
    }
}

impl Range<ZeroIndexed> {
    pub fn one_indexed(self) -> Range<OneIndexed> {
        Range {
            row_start: self.row_start.one_indexed(),
            row_end: self.row_end.one_indexed(),
            col_start: self.col_start.one_indexed(),
            col_end: self.col_end.one_indexed(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Location<I: Indexed> {
    pub file: PathBuf,
    pub position: Position<I>,
}

impl<I: Indexed> Location<I> {
    pub fn new<F: Into<PathBuf>>(row: Row<I>,
                                 col: Column<I>,
                                 file: F)
                                 -> Location<I> {
        Location {
            position: Position {
                row: row,
                col: col,
            },
            file: file.into(),
        }
    }

    pub fn from_position<F: Into<PathBuf>>(position: Position<I>,
                                           file: F)
                                           -> Location<I> {
        Location {
            position: position,
            file: file.into(),
        }
    }
}

impl<I: Indexed> Clone for Location<I> {
    fn clone(&self) -> Location<I> {
        Location {
            position: self.position,
            file: self.file.clone(),
        }
    }
}

impl Location<OneIndexed> {
    pub fn zero_indexed(&self) -> Location<ZeroIndexed> {
        Location {
            position: self.position.zero_indexed(),
            file: self.file.clone(),
        }
    }
}

impl Location<ZeroIndexed> {
    pub fn one_indexed(&self) -> Location<OneIndexed> {
        Location {
            position: self.position.one_indexed(),
            file: self.file.clone(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Span<I: Indexed> {
    pub file: PathBuf,
    pub range: Range<I>,
}

impl<I: Indexed> Span<I> {
    pub fn new<F: Into<PathBuf>>(row_start: Row<I>,
                                 row_end: Row<I>,
                                 col_start: Column<I>,
                                 col_end: Column<I>,
                                 file: F)
                                 -> Span<I> {
        Span {
            range: Range {
                row_start: row_start,
                row_end: row_end,
                col_start: col_start,
                col_end: col_end,
            },
            file: file.into(),
        }
    }

    pub fn from_range<F: Into<PathBuf>>(range: Range<I>, file: F) -> Span<I> {
        Span {
            range: range,
            file: file.into(),
        }
    }

    pub fn from_positions<F: Into<PathBuf>>(start: Position<I>,
                                            end: Position<I>,
                                            file: F)
                                            -> Span<I> {
        Span {
            range: Range::from_positions(start, end),
            file: file.into(),
        }
    }
}

impl<I: Indexed> Clone for Span<I> {
    fn clone(&self) -> Span<I> {
        Span {
            range: self.range,
            file: self.file.clone(),
        }
    }
}

impl Span<OneIndexed> {
    pub fn zero_indexed(&self) -> Span<ZeroIndexed> {
        Span {
            range: self.range.zero_indexed(),
            file: self.file.clone(),
        }
    }
}

impl Span<ZeroIndexed> {
    pub fn one_indexed(&self) -> Span<OneIndexed> {
        Span {
            range: self.range.one_indexed(),
            file: self.file.clone(),
        }
    }
}

pub trait Indexed: Deserialize + Serialize {}
#[derive(Hash, PartialEq, Eq, Debug, Deserialize, Serialize, PartialOrd, Ord)]
pub struct ZeroIndexed;
impl Indexed for ZeroIndexed {}
#[derive(Hash, PartialEq, Eq, Debug, Deserialize, Serialize, PartialOrd, Ord)]
pub struct OneIndexed;
impl Indexed for OneIndexed {}


#[cfg(test)]
mod test {
}
