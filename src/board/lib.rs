// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

const WIDTH : usize = 8;
const HEIGHT: usize = 8;

/// The `Board` structure defines the grid and the score.

pub struct Board {
    grid: [[i32; WIDTH]; HEIGHT],
    score: [usize; 2],
}

impl Board {

    /// The `get` function returns the value cell.

    pub fn get (
        &self,
        x: usize,
        y: usize,
    ) -> i32 {
        self.grid[x][y]
    }

    /// The `set` function writes the value at [X; Y] cell.

    pub fn set (
        &mut self,
        x: usize,
        y: usize,
        value: i32,
    ) -> bool {
        self.grid[x][y] = value;
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
            true => self.set(x, y, pid),
            false => self.set(x, y, -pid),
        }
    }

    pub fn spawn_pawn (
        &mut self,
        pid: i32,
    ) -> bool {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                return match self.get(x, y) {
                    0i32 => self.add_pawn(pid, x, y),
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
                self.set(x, y, 0i32)
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
                    value if value ==  pid => Some((true, x, y)),
                    value if value == -pid => Some((false, x, y)),
                    _ => continue,
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
                if self.get(x, y) != 0i32 {
                    len += 1;
                }
            }
        }
        len
    }
}

impl Default for Board {

    /// The `default` constructor function returns
    /// a empty board.

    fn default() -> Self {
        Board {
            grid: [[0i32; WIDTH]; HEIGHT],
            score: [0usize; 2],
        }
    }
}
/*
impl std::fmt::Display for Board {

    /// The `fmt` function interfaces the print
    /// of structure'Board.

	fn fmt (
        &self,
        f: &mut std::fmt::Formatter
    ) -> Result<(), std::fmt::Error> {
		let mut to_return = Ok(());

		for y in (0..WIDTH) {
			for x in (0..HEIGHT) {
				to_return = to_return.and(write!(f, "{} ", self.get(x, y)));
			}
			to_return = to_return.and(write!(f, "\n"));
		}
		to_return
	}
}
*/
