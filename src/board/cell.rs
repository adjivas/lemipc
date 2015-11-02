// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Cell {
    Empty,
    Pawn(i32, bool),
}

impl Cell {
    pub fn get <'a> (
        &'a self,
    ) -> Option<(i32, bool)> {
        match self {
            &Cell::Empty => None,
            &Cell::Pawn(pid, team) => Some((pid, team)),
        }
    }

    pub fn get_pid <'a> (
        &'a self,
    ) -> Option<i32> {
        match self {
            &Cell::Empty => None,
            &Cell::Pawn(pid, _) => Some(pid),
        }
    }

    pub fn get_team <'a> (
        &'a self,
    ) -> Option<bool> {
        match self {
            &Cell::Empty => None,
            &Cell::Pawn(_, team) => Some(team),
        }
    }

    pub fn set <'a> (
        &'a mut self,
        pid: i32,
        team: bool,
    ) -> bool {
        match self {
            &mut Cell::Empty => {
                *self = Cell::Pawn(pid, team);
                true
            },
            _ => false,
        }

    }

    pub fn unset <'a> (
        &'a mut self,
    ) -> bool {
        match self {
            &mut Cell::Empty => false,
            _ => {
                *self = Cell::Empty;
                true
            },
        }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Empty
    }
}
