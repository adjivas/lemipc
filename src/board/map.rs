// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
extern crate std;

use board::cell::Cell;

const WIDTH : usize = 4;
const HEIGHT: usize = 4;

/// The `Map` structure defines the grid and the score.

pub struct Map {
    grid: [[Cell; WIDTH]; HEIGHT],
    score: [usize; 2],
}

impl Map {

    /// The `get_pid` function returns the (pid, team) cell.

    pub fn get (
        &self,
        x: usize,
        y: usize,
    ) -> Option<(i32, bool)> {
        self.grid[x][y].get()
    }

    /// The `get_pid` function returns the pid cell.

    pub fn get_pid (
        &self,
        x: usize,
        y: usize,
    ) -> Option<i32> {
        self.grid[x][y].get_pid()
    }

    /// The `get_team` function returns the team cell.

    fn get_team (
        &self,
        x: usize,
        y: usize,
    ) -> Option<bool> {
        self.grid[x][y].get_team()
    }

    /// The `set` function writes the value at [X; Y] cell.

    pub fn set (
        &mut self,
        x: usize,
        y: usize,
        pid: i32,
        team: bool,
    ) -> bool {
        self.grid[x][y].set(pid, team);
        true
    }

    /// The `unset` function writes the value at [X; Y] cell.

    pub fn unset (
        &mut self,
        x: usize,
        y: usize,
    ) -> bool {
        self.grid[x][y].unset();
        true
    }

    /// The `get_score` function returns the score.

    pub fn get_score (
        &self,
    ) -> [usize; 2] {
        self.score
    }

    /// The `set_score` function add +1 point to
    /// team one or two

    fn set_score (
        &mut self,
        team: bool,
    ) {
        match team {
            true  => self.score[0] += 1,
            false => self.score[1] += 1,
        }
    }

    fn add_pawn (
        &mut self,
        pid: i32,
        x: usize,
        y: usize,
    ) -> bool {
        match self.len_pawn() % 2 == 0 {
            team => self.set(x, y, pid, team),
        }
    }

    pub fn spawn_pawn (
        &mut self,
        pid: i32,
    ) -> bool {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                return match self.get_pid(x, y) {
                    None => self.add_pawn(pid, x, y),
                    _ => continue ,
                }
            }
        }
        false
    }

    pub fn dead_pawn (
        &mut self,
        pid: i32,
    ) -> bool {
        match self.search_pawn(pid) {
            Some((team, x, y)) => {
                self.set_score(team);
                self.unset(x, y)
            },
            None => false,
        }
    }

    fn search_pawn (
        &mut self,
        pid: i32,
    ) -> Option<(bool, usize, usize)> {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                return match self.get(x, y) {
                    Some((id, team)) if id == pid => Some((team, x, y)),
                    _ => continue ,
                }
            }
        }
        None
    }

    pub fn len_pawn (
        &self,
    ) -> usize {
        let mut len: usize = 0;

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if let Some(_) = self.get_pid(x, y) {
                    len += 1;
                }
            }
        }
        len
    }
}

impl Default for Map {

    /// The `default` constructor function returns
    /// a empty map.

    fn default() -> Self {
        Map {
            grid: [[Default::default(); WIDTH]; HEIGHT],
            score: [0usize; 2],
        }
    }
}
