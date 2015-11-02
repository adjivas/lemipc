// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//use command::Compass::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Compass {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Compass {
    pub fn new (character: i8) -> Option<Self> {
        return match character {
            110 => Some(Compass::NORTH),
            101 => Some(Compass::EAST),
            115 => Some(Compass::SOUTH),
            119 => Some(Compass::WEST),
            _ => None,
        }
    }
}
