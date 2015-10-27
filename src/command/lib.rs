// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use board;
use std;
use sig;
use shm;
use msg;

/// Dead dead dead !

pub fn quit (_: i32) {
    let pid = getpid!();

    let id = shm_getboard_if_created!().unwrap();
    let addr = shmat!(id).unwrap();
    let board: &mut board::Board = {
        unsafe {
            std::mem::transmute(addr)
        }
    };

    board.dead_pawn(pid);
    if board.len_pawn() == 0 {
        shmctl!(id, shm::ffi::Ipc::RMID);
        msgctl!(id, msg::ffi::Ipc::RMID);
    }
    shmdt!(addr);
    exit!();
}
