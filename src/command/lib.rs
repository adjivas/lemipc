// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use board::Map;

/// The `start` function prints the first message of game.

pub fn start (map: &Map, pid: i32) {
    let team = map.found_team(pid);
    let out = {
        " _                   ___\n\
        | |    ___ _ __ ___ |_ _|_ __   ___\n\
        | |   / _ \\ '_ ` _ \\ | || '_ \\ / __|\n\
        | |__|  __/ | | | | || || |_) | (__ \n\
        |_____\\___|_| |_| |_|___| .__/ \\___|
                        |_|\n\
        Welcome "
    };

    write!(out.as_ptr(), out.len());
    write_number!(pid);
    write!(", your team is \0".as_ptr(), 16);
    match team {
        Some(true) => write!("<A>".as_ptr(), 3),
        Some(false) => write!("<B>".as_ptr(), 3),
        None => unimplemented!(),
    };
    write!(".\n\n".as_ptr(), 3);
    help(0);
}

/// .

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

/// The `map` function prints the map according to
/// the team of pid.

pub fn map (map: &Map, pid: i32) {
    map.put_grid_team(pid);
}

/// The `cheat` function prints all the map.

pub fn cheat (map: &Map) {
    map.put_grid_team(0);
}

/// The `whoiam` function prints the pid.

pub fn whoiam (pid: i32) {
    writeln_number!(pid);
}

/// The `score` function prints the result of team.

pub fn score (map: &Map) {
    let [a, b] = map.get_score();

    write!("A-B ".as_ptr(), 4);
    write_number!(a);
    write_character!('-');
    writeln_number!(b);
}

/// The `help` function prints all commands of game.

pub fn help (_: i32) {
    let out = {
        "/h, (hello) - Say hello!\n\
        /p <compass>, (play) - advance to a direction.\n\
        /e <pid> <message>, (email) - send a message to a friend.\n\
        /r, (receive) - read a message from a friend.\n\
        /m, (map) - print your team map.\n\
        /c, (cheat) - print all the map.\n\
        /w, (who i am) - print my name.\n\
        /o, (score) - print my score.\n\
        /h, (help) - print all commands.\n\
        /q, (quit) - exit the game.\n\n"
    };

    write!(out.as_ptr(), out.len());
}

/// Dead dead dead !

pub fn quit (_: i32) {
    let pid: i32 = getpid!();
    let id = shm_getboard_if_created!().unwrap();
    let addr = shmat!(id).unwrap();
    let board: &mut Map = {
        unsafe {
            std::mem::transmute(addr)
        }
    };

    board.dead_pawn(pid);
    score(&board);
    if board.len_pawn() == 0 {
        shmctl!(id, shm::ffi::Ipc::RMID);
        msgctl!(id, msg::ffi::Ipc::RMID);
    }
    shmdt!(addr);
    exit!();
}
