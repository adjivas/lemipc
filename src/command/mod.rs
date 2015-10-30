// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

mod lib;

pub use ffi;
pub use self::lib::{
    start,
    play,
    email,
    receive,
    map,
    cheat,
    whoiam,
    score,
    help,
    quit,
};
