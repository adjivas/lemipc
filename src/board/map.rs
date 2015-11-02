// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]

extern crate std;

use board::Cell;
use command::Compass;

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

pub const NORTH: i8 = 110;
pub const EAST: i8 = 101;
pub const SOUTH: i8 = 115;
pub const WEST: i8 = 119;

/// The `Map` structure defines the grid and the score.

pub struct Map {
    grid: [[Cell; WIDTH]; HEIGHT],
    score: [usize; 2],
    turn: i32,
}

impl Map {

    /// The `get` function returns the (pid, team) cell.

    pub fn get <'a> (
        &'a self,
        x: usize,
        y: usize,
    ) -> Option<(i32, bool)> {
        self.grid[x][y].get()
    }

    /// The `get_pid` function returns the pid cell.

    pub fn get_pid <'a> (
        &'a self,
        x: usize,
        y: usize,
    ) -> Option<i32> {
        self.grid[x][y].get_pid()
    }

    /// The `get_team` function returns the team cell.

    fn get_team <'a> (
        &'a self,
        x: usize,
        y: usize,
    ) -> Option<bool> {
        self.grid[x][y].get_team()
    }

    /// The `get_score` function returns the score.

    pub fn get_score <'a> (
        &'a self,
    ) -> [usize; 2] {
        self.score
    }

    /// The `get_turn` function returns who must play.

    pub fn get_turn <'a> (
        &'a self,
    ) -> i32 {
        self.turn
    }

    /// The `set` function writes the value at [X; Y] cell.

    pub fn set <'a> (
        &'a mut self,
        x: usize,
        y: usize,
        pid: i32,
        team: bool,
    ) -> bool {
        self.grid[x][y].set(pid, team)
    }

    /// The `unset` function writes the value at [X; Y] cell.

    pub fn unset <'a> (
        &'a mut self,
        x: usize,
        y: usize,
    ) -> bool {
        self.grid[x][y].unset()
    }

    /// The `set_score` function add +1 point to
    /// team one or two

    fn set_score <'a> (
        &'a mut self,
        team: bool,
    ) {
        match team {
            true  => self.score[1] += 1,
            false => self.score[0] += 1,
        }
    }

    fn to_next <'a> (
        &'a mut self,
        pid: i32,
    ) -> Option<i32> {
        let mut min_after_pid: Option<i32> = None;

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                min_after_pid = match (min_after_pid, self.get(x, y)) {
                    (None, Some((id, _))) if pid < id => Some(id),
                    (
                        Some(max),
                        Some((id, _))
                    ) if pid < id && id < max => Some(id),
                    _ => continue ,
                }
            }
        }
        if let Some(pid_next) = min_after_pid {
            self.turn = pid_next;
        }
        min_after_pid
    }

    pub fn play_pawn <'a> (
        &'a mut self,
        pid: i32,
        compass: Compass,
    ) -> Option<i32> {
        if self.turn == pid {
            /*if let Some ((
                (team, to_x, to_y), (x, y)
            )) = match (compass, self.search_pawn(pid)) {
                (command::Compass::NORTH, Some((team, x, y))) if y > 0 => Some ((
                    (team, x, y-1), (x, y)
                )),
                (command::Compass::EAST, Some((team, x, y))) if x > 0 => Some ((
                    (team, x-1, y), (x, y)
                )),
                (command::Compass::SOUTH, Some((team, x, y))) if x < {WIDTH-1} => Some ((
                    (team, x+1, y), (x, y)
                )),
                (command::Compass::WEST, Some((team, x, y))) if y < {HEIGHT-1} => Some ((
                    (team, x, y+1), (x, y)
                )),
                _ => None,
            } {
                self.unset(x, y);
                self.set(to_x, to_y, pid, team);
                let pid = self.to_next(pid);
                println!("local {:?}", pid);
                return pid
            }*/
        }
        None
    }

    pub fn spawn_pawn <'a> (
        &'a mut self,
        pid: i32,
    ) -> bool {
        let team = self.len_pawn() % 2 == 0;
        let iter_y = match team {
            true => (0..self.grid.len()).collect::<Vec<_>>(),
            false => (0..self.grid.len()).rev().collect::<Vec<_>>(),
        };

        if self.turn == 0 {
            self.turn = pid;
        }
        for y in iter_y {
            let iter_x = match team {
                true => (0..self.grid[y].len()).collect::<Vec<_>>(),
                false => (0..self.grid[y].len()).rev().collect::<Vec<_>>(),
            };

            for x in iter_x {
                return match self.get_pid(x, y) {
                    None => self.set(x, y, pid, team),
                    _ => continue ,
                }
            }
        }
        false
    }

    pub fn dead_pawn <'a> (
        &'a mut self,
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

    fn search_pawn <'a> (
        &'a self,
        pid: i32,
    ) -> Option<(bool, usize, usize)> {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                return match self.get(x, y) {
                    Some((id, team)) if id == pid => Some((
                        team,
                        x,
                        y
                    )),
                    _ => continue ,
                }
            }
        }
        None
    }

    pub fn len_pawn <'a> (
        &'a self,
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

    pub fn found_team <'a> (
        &'a self,
        pid: i32,
    ) -> Option<bool> {
        match pid {
            0 => None,
            _ => {
                match self.search_pawn(pid) {
                    Some((team, _, _)) => Some(team),
                    None => None,
                }
            },
        }
    }

    pub fn put_grid_team <'a> (
        &'a self,
        pid: i32,
    ) {
        let team = self.found_team(pid);

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                match self.get(x, y) {
                    Some((id, t)) if Some(t) == team ||
                                     None == team => {
                        write_number!(id);
                    },
                    Some((_, _)) => {
                        write_character!('0');
                    },
                    None => {
                        write_character!('_');
                    },
                };
                write_character!(' ');
            }
            write_character!('\n');
        }
    }

}

impl Default for Map {

    /// The `default` constructor function returns
    /// a empty map.

    fn default() -> Self {
        Map {
            grid: [[Default::default(); WIDTH]; HEIGHT],
            score: [0usize; 2],
            turn: 0i32,
        }
    }
}
