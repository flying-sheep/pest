// pest. Smart PEGs in Rust
// Copyright (C) 2016  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// A `struct` representing tokens generated by a parser.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Token<Rule> {
    /// matched `Rule`
    pub rule:  Rule,
    /// starting position in `Input`
    pub start: usize,
    /// ending position in `Input`
    pub end:   usize
}
