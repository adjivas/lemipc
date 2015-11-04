// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

mod map;
mod cell;

const VISION: isize = 1;

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

pub use self::map::Map;
pub use self::cell::Cell;
