// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use board::Board;

pub fn hello (_: i32) {
}

pub fn play (_: i32) {
}

pub fn map (_: i32) {
}

pub fn mail (_: i32) {
    match read_number!() {
        Some(at) => match read!() {
            Some((_, _)) => {
                println!("snd: {}", at);
            },
            None => println!("msg: invalid message."),
        },
        None => println!("msg: invalid number."),
    }
}

pub fn cheat (_: i32) {
}

/// Dead dead dead !

pub fn quit (_: i32) {
    let id = shm_getboard_if_created!().unwrap();
    let addr = shmat!(id).unwrap();
    let board: &mut Board = {
        unsafe {
            std::mem::transmute(addr)
        }
    };

    board.dead_pawn(
        getpid!()
    );
    if board.len_pawn() == 0 {
        shmctl!(id, shm::ffi::Ipc::RMID);
        msgctl!(id, msg::ffi::Ipc::RMID);
    }
    shmdt!(addr);
    exit!();
}

pub fn help (_: i32) {
    println!("/quit, /q - exit the program.
             /to, /t <number> <line> - send a message.
             /map, /m - print team map.
             /cheat, /c - print all the map.
             /play, /p <x> <y> - play the pawn.
             /help, /h - print command information.");
}
