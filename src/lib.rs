// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(macro_reexport)]

#[macro_use]
#[macro_reexport (
    ftok,
    shmget_id,
    shmat,
    shmget,
    shmdt,
    shmctl
)]
extern crate shm as shm_crate;

#[macro_use]
#[macro_reexport (
    msgctl,
    msgget,
    msgsnd,
    msgrcv
)]
extern crate msg as msg_crate;

#[macro_use]
#[macro_reexport (
    kill,
    signal,
    getpid
)]
extern crate sig as sig_crate;

#[macro_use]
#[macro_reexport (
    read,
    read_command,
    read_character,
    read_number,
    write,
    write_number,
    write_character,
    writeln_number,
)]
extern crate io as io_crate;

pub mod shm {
    pub use shm_crate::*;
}

pub mod msg {
    pub use msg_crate::*;
}

pub mod sig {
    pub use sig_crate::*;
}

pub mod io {
    pub use io_crate::*;
}

#[macro_use]
mod macros;
pub mod board;
pub mod command;
pub mod ffi;
