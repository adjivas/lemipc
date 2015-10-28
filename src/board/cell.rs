// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Empty,
    Pawn(i32, bool),
}

impl Cell {
    pub fn get (
        &self
    ) -> Option<(i32, bool)> {
        match self {
            &Cell::Empty => None,
            &Cell::Pawn(pid, team) => Some((pid, team)),
        }
    }

    pub fn get_pid (
        &self
    ) -> Option<i32> {
        match self {
            &Cell::Empty => None,
            &Cell::Pawn(pid, _) => Some(pid),
        }
    }

    pub fn get_team (
        &self
    ) -> Option<bool> {
        match self {
            &Cell::Empty => None,
            &Cell::Pawn(_, team) => Some(team),
        }
    }

    pub fn set (
        &mut self,
        pid: i32,
        team: bool,
    ) {
        *self = Cell::Pawn(pid, team)
    }

    pub fn unset (
        &mut self,
    ) {
        *self = Cell::Empty
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Empty
    }
}
