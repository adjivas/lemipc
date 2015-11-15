// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use board::Map;
use command::Compass;

/// The `start` function prints the first message of game.

pub fn start <'a> (
    board: &'a Map,
    pid: i32,
) {
    let team = board.found_team(pid);
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
    write!(", your team is ".as_ptr(), 15);
    match team {
        Some(true) => write!("<A>".as_ptr(), 3),
        Some(false) => write!("<B>".as_ptr(), 3),
        None => unimplemented!(),
    };
    write!(".\n\n".as_ptr(), 3);
    help(0);
}

/// The `turn` function informs the user who must play.

#[cfg(not(feature = "signal"))]
pub fn turn <'a> (
    board: &'a Map,
) {
    let pid: i32 = board.get_turn();

    write_number!(pid);
    write!(".\n".as_ptr(), 2);
}

/// The `turn` function informs the user who must play.

#[cfg(feature = "signal")]
pub fn turn (_: i32) {
    let pid: i32 = getpid!();

    write_number!(pid);
    write!(" must play.\n".as_ptr(), 12);
}

/// The `play` function checks if the user can play,
/// if yes the pawn is moved.

#[cfg(not(feature = "signal"))]
pub fn play <'a> (
    board: &'a mut Map,
    pid: i32,
) {
    if let Some(character) = read_character!() {
        match Compass::new(character) {
            Some(compass) => match board.play_pawn(pid, compass) {
                Ok(_) => {
                    map(board, pid);
                },
                Err(why) => { write_err!(why.as_ptr()); },
            },
            _ => { write_err!("play: invalid compass.\n\0".as_ptr()); },
        }
    }
}

/// The `play` function checks if the user can play,
/// if yes the pawn is moved.

#[cfg(feature = "signal")]
pub fn play <'a> (
    board: &'a mut Map,
    pid: i32,
) {
    if let Some(character) = read_character!() {
        match Compass::new(character) {
            Some(compass) => match board.play_pawn(pid, compass) {
                Ok(true) => {
                    map(board, pid);
                    kill!(board.get_turn(), sig::ffi::Sig::USR2);
                },
                Ok(false) => { kill!(board.get_turn(), sig::ffi::Sig::USR2); },
                Err(why) => { write_err!(why.as_ptr()); },
            },
            _ => { write_err!("play: invalid compass.\n\0".as_ptr()); },
        }
    }
}

/// The `email` function sends a message.

#[allow(unused_assignments)]
pub fn email (id: i32) {
    match read_number!() {
        Some(at) => match read!() {
            #[cfg(feature = "signal")]
            Some((_, line)) => {
                msgsnd!(id, at, &line);
                kill!(at, sig::ffi::Sig::USR1);
            },
            #[cfg(not(feature = "signal"))]
            Some((_, line)) => {
                msgsnd!(id, at, &line);
            },
            None => { write_err!("email: invalid message.\n\0".as_ptr()); },
        },
        None => { write_err!("email: invalid number.\n\0".as_ptr()); },
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

pub fn map <'a> (
    board: &'a Map,
    pid: i32,
) {
    board.put_grid_team(pid);
}

/// The `cheat` function prints all the map.

pub fn cheat <'a> (board: &'a Map) {
    board.put_grid_team(0);
}

/// The `whoiam` function prints the pid.

pub fn whoiam (pid: i32) {
    writeln_number!(pid);
}

/// The `score` function prints the result of team.

pub fn score <'a> (board: &'a Map) {
    let [a, b] = board.get_score();

    write!("\nA-B ".as_ptr(), 5);
    write_number!(a);
    write_character!('-');
    writeln_number!(b);
}

/// The `help` function prints all commands of game.

pub fn help (_: i32) {
    let out = {
        "/h, (hello) - Say hello!\n\
        /t, (turn)  - inform about who must play.\n\
        /p <compass>, (play [n, e, s, w])  - advance to a direction.\n\
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

#[allow(unused_unsafe)]
pub fn quit (_: i32) {
    let pid: i32 = getpid!();
    let shm_id = shm_getboard_if_created!().expect (
        "shm_getboard_if_created! fail"
    );
    let shm_addr = shmat!(shm_id).expect("shmat! fail");
    let shm_map: &mut Map = {
        unsafe {
            std::mem::transmute(shm_addr)
        }
    };
    let sem_id: i32 = semget_id! (
        ftok!().expect("ftok! fail")
    ).expect("semget! fail");

    shm_map.dead_pawn(pid);
    score(&shm_map);
    if shm_map.len_pawn() == 0 {
        shmctl!(shm_id, shm::ffi::Ipc::RMID);
        msgctl!(shm_id, msg::ffi::Ipc::RMID);
        semctl!(sem_id, sem::ffi::Ipc::RMID);
    }
    shmdt!(shm_addr);
    exit!();
}
