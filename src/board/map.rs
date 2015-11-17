// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use board::Cell;
use command::Compass;

use super::WIDTH;
use super::HEIGHT;
use super::VISION;

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

    pub fn get_team <'a> (
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

    /// The `to_next` function updates the turn of player.

    fn to_next <'a> (
        &'a mut self,
        pid: i32,
    ) {
        let mut after: i32 = 0;
        let mut first: i32 = 0;

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if let Some((id, _)) = self.get(x, y) {
                    if id < after
                    && id > pid
                    || after == 0
                    && id > pid {
                        after = id;
                    }
                    if first > id
                    || first == 0 {
                        first = id;
                    }
                }
            }
        }
        self.turn = match (first, after) {
            (first, after) if pid == after => first,
            (first, 0) if first > 0 => first,
            (_, after) if after > 0 => after,
            (_, _) => unimplemented!(),
        }
    }

    /// The `play_pawn` function reverses a empty
    /// position with a pawn.

    fn swap_with_team <'a> (
        &'a mut self,
        (x, y): (usize, usize),
        (to_x, to_y, pid, team): (usize, usize, i32, bool),
    ) -> Result<bool, &'static str> {
        if self.get_team(to_x, to_y).is_none() {
            self.to_next(pid);
            Ok(self.unset(x, y) && self.set(to_x, to_y, pid, team))
        }
        else {
            Err("forbidden movement!\n\0")
        }
    }

    /// The `play_pawn` function moves the pawn.

    pub fn play_pawn <'a> (
        &'a mut self,
        pid: i32,
        compass: Compass,
    ) -> Result<bool, &'static str> {
        if self.turn == pid {
            match (compass, self.search_pawn(pid)) {
                (Compass::NORTH, Some((team, x, y))) if y > 0 =>
                    self.swap_with_team((x, y), (x, y-1, pid, team)),
                (Compass::EAST, Some((team, x, y))) if x < {WIDTH-1} =>
                    self.swap_with_team((x, y), (x+1, y, pid, team)),
                (Compass::SOUTH, Some((team, x, y))) if y < {HEIGHT-1} =>
                    self.swap_with_team((x, y), (x, y+1, pid, team)),
                (Compass::WEST, Some((team, x, y))) if x > 0 =>
                    self.swap_with_team((x, y), (x-1, y, pid, team)),
                _ => Err("unvalid movement.\n\0"),
            }
        }
        else {
            Err("It's yet your turn.\n\0")
        }
    }

    /// The `spawn_pawn` function spawns a new pawn.

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

    /// The `dead_pawn` function removes a pawn
    /// and updates the turn of next pawn.

    pub fn dead_pawn <'a> (
        &'a mut self,
        pid: i32,
    ) -> bool {
        match self.search_pawn(pid) {
            Some((team, x, y)) => {
                self.set_score(team);
                self.to_next(pid);
                self.unset(x, y)
            },
            None => false,
        }
    }

    /// The `count_enemy` function returns the
    /// number of enemy around one pawn.

    pub fn count_enemy <'a> (
        &'a self,
        pid: i32,
    ) -> Option<usize> {
        let mut enemy: usize = 0;

        match self.search_pawn(pid) {
            Some((team, x, y)) => {
                for y in ({(y as isize)-VISION}..{(y as isize)+VISION+1}).filter(|&y|
                    y >= 0 && y < self.grid.len() as isize
                ) {
                    for x in ({(x as isize)-VISION}..{(x as isize)+VISION+1}).filter(|&x|
                        x >= 0 && x < self.grid[y as usize].len() as isize
                    ) {
                        if self.get_team(x as usize, y as usize) == Some(!team) {
                            enemy += 1;
                        }
                    }
                }
                Some(enemy)
            },
            None => None,
        }
    }

    /// The `search_pawn` founds information
    /// on pawn according to pid.

    fn search_pawn <'a> (
        &'a self,
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

    /// The `found_team` returns the team
    /// of pawn according to pid.

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

    /// The `len_pawn` counts the number of pawn.

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

    /// The `put_grid_team` prints the grid
    /// with a limited vision of team.

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
