// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use board::Map;

/// The `hello` function prints the first message of game.

pub fn hello (c: i32) {
    let out = {
        " _                   ___\n\
        | |    ___ _ __ ___ |_ _|_ __   ___\n\
        | |   / _ \\ '_ ` _ \\ | || '_ \\ / __|\n\
        | |__|  __/ | | | | || || |_) | (__ \n\
        |_____\\___|_| |_| |_|___| .__/ \\___|
                        |_|\n\
        Welcome {pid}\n\n"
    };

    write!(out.as_ptr(), out.len());
    help(c);
}

pub fn play (_: i32) {
}

/// The `email` function sends a message.

#[allow(unused_assignments)]
pub fn email (id: i32) {
    match read_number!() {
        Some(at) => match read!() {
            Some((_, line)) => {
                msgsnd!(id, at, &line);
                kill!(at, sig::ffi::Sig::USR1);
            },
            None => { write_err!("msg: invalid message.".as_ptr()); },
        },
        None => { write_err!("msg: invalid number.".as_ptr()); },
    }
}

/// The `receive` function takes and prints the last message.

#[allow(unused_unsafe, unused_assignments)]
pub fn receive (_: i32) {
    extern crate msg;

    if let Some(key) = ftok!() {
        if let Some(id) = msgget!(key) {
            if let Some(line) = msgrcv! (id, getpid!()) {
                write!(&line, msg::ffi::MSG_BUFF);
            }
        }
    }
}

pub fn map (_: i32) {
}

pub fn cheat (_: i32) {
}

/// Dead dead dead !

pub fn quit (_: i32) {
    let id = shm_getboard_if_created!().unwrap();
    let addr = shmat!(id).unwrap();
    let board: &mut Map = {
        unsafe {
            std::mem::transmute(addr)
        }
    };

    board.dead_pawn (
        getpid!()
    );
    if board.len_pawn() == 0 {
        shmctl!(id, shm::ffi::Ipc::RMID);
        msgctl!(id, msg::ffi::Ipc::RMID);
    }
    shmdt!(addr);
    exit!();
}

/// The `help` function prints all commands of game.

pub fn help (_: i32) {
    let out = {
        "/hello, /h - Say hello!\n\
        /play, /p <compass> - advance to a direction\n\
        /email, /e <pid> - send a message to a friend\n\
        /map, /m - print your team map\n\
        /cheat, /c - print all the map\n\
        /quit, /q - exit the game\n\
        /help, /h - print all commands\n\n"
    };

    write!(out.as_ptr(), out.len());
}
