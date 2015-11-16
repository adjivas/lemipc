// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

pub const WIDTH : usize = 8;
pub const HEIGHT: usize = 8;

/// The `Board` structure defines the grid and the score.

pub struct Board {
    grid: [[i32; WIDTH]; HEIGHT],
    score: [u8; 2],
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

    /// The `get_score` function returns the score.

    pub fn get_score (
        &self,
    ) -> [u8; 2] {
        self.score
    }

    /// The `get_size_y` function returns the x number
    /// of cell.

    pub fn get_size (
        &self,
    ) -> usize {
        self.grid.len()
    }
}

impl Default for Board {

    /// The `default` constructor function returns
    /// a empty board.

    fn default() -> Self {
        Board {
            grid: [[0i32; WIDTH]; HEIGHT],
            score: [0u8; 2],
        }
    }
}

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
