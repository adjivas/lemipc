// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate console;

use console::shm;
use console::msg;
use console::sig;
use console::io;
use console::board;

fn main () {
    signal!(sig::ffi::Sig::KILL, console::command::quit);
    signal!(sig::ffi::Sig::INT, console::command::quit);

    let id = shm_getboard!().unwrap();
    let addr = shmat!(id).unwrap();
    let board: &mut console::board::Board = {
        unsafe {
            std::mem::transmute(addr)
        }
    };

    let pid = getpid!();

    board.spawn_pawn(pid);
    loop {
        match read_command!() {
            Some(c) if c == 27 => {/*println!("{}", board)*/}, // map
            Some(c) if c == 62 || c == 36 => {
                match read_number!() {
                    Some(at) => match read!() {
                        Some((_, line)) => {
                            println!("snd: {}", at);
                        },
                        None => println!("msg: invalid message."),
                    },
                    None => println!("msg: invalid number."),
                }
                println!("end");
            }, // msg, message
            Some(c) if c == 50 => {}, // play
            Some(c) if c == 63 => console::command::quit(0), // quit
            Some(c) if c == 37 => {
                println!("/quit, /q - exit the program.
                         /to, /t <number> <line> - send a message.
                         /map, /m - print team map.
                         /cheat, /c - print all the map.
                         /play, /p <x> <y> - play the pawn.
                         /help, /h - print command information.");
            }, // help
            _ => {
                 println!("Command not found.");
            },
        }
    }
}
