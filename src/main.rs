// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate lemipc;

mod board;

fn main () {
    let id = shm_getboard!().unwrap();
    let addr = shmat!(id).unwrap();
    let board: &mut lemipc::board::Board = {
        unsafe {
            std::mem::transmute(addr)
        }
    };

    board.spawn_pawn (
        getpid!()
    );
    signal!(sig::ffi::Sig::KILL, lemipc::command::quit);
    signal!(sig::ffi::Sig::INT, lemipc::command::quit);
    loop {
        match read_command!() {
            Some(c) if c == 50 => lemipc::command::hello(c as i32),
            Some(c) if c == 51 => lemipc::command::play(c as i32),
            Some(c) if c == 27 => lemipc::command::map(c as i32),
            Some(c) if c == 62 => lemipc::command::mail(c as i32),
            Some(c) if c == 27 => lemipc::command::cheat(c as i32),
            Some(c) if c == 63 => lemipc::command::quit(c as i32),
            Some(c) if c == 37 => lemipc::command::help(c as i32),
            _ => {
                 println!("Command not found.");
            },
        }
    }
}
