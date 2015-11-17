// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate clap;

const DEFAULT_PLAYER: &'static str = "one";
const DEFAULT_X: &'static str = "520";
const DEFAULT_Y: &'static str = "480";

/// The `Player` enum defines the team of our player.

#[derive(Debug)]
pub enum Player {
    One,
    Two,
}

impl Player {

    /// The `from_str` contructor function returns the
    /// Player enum.

	pub fn from_str(s: &str) -> Self {
	    match s {
	        "one" => Player::One,
	        "two" => Player::Two,
	        _     => panic!("Player cli option must be `one` or `two` team"),
	    }
	}
}

/// The `Command` structure describes our program
/// arguments of game.

#[derive(Debug)]
pub struct Command {
    player: Player,
    pub width: u32,
    pub height: u32,
}

impl Command {

    /// The `Player` enum defines the team of our player.

	pub fn parse (
        m: &clap::ArgMatches
    ) -> Self {
		let playero = m.value_of("player").unwrap_or(DEFAULT_PLAYER);
		let xo = m.value_of("x").unwrap_or(DEFAULT_X);
        let yo = m.value_of("y").unwrap_or(DEFAULT_Y);

		if xo.parse::<u32>().is_err() {
			panic!("The <x> coordinate option must be an unsigned interger.");
		}
        if yo.parse::<u32>().is_err() {
			panic!("The <y> coordinate option must be an unsigned interger.");
		}
		Command {
			player: Player::from_str(playero),
			width: xo.parse::<u32>().unwrap(),
            height: yo.parse::<u32>().unwrap(),
		}
	}
}
