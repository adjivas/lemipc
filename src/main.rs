// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate lemipc;

mod board;

#[allow(unused_unsafe, unused_assignments)]
fn main () {
    let pid: i32 = getpid!();
    let msg_id: i32 = msgget!(
        ftok!().unwrap()
    ).unwrap();
    let board: &mut lemipc::board::Board = {
        let id = shm_getboard!().unwrap();
        let addr = shmat!(id).unwrap();

        unsafe {
            std::mem::transmute(addr)
        }
    };

    signal!(sig::ffi::Sig::USR1, lemipc::command::receive);
    signal!(sig::ffi::Sig::KILL, lemipc::command::quit);
    signal!(sig::ffi::Sig::INT, lemipc::command::quit);
    board.spawn_pawn(pid);
    lemipc::command::hello(pid);
    loop {
        match read_command!() {
            Some(c) if c == 50 => lemipc::command::hello(pid),
            Some(c) if c == 51 => lemipc::command::play(c as i32),
            Some(c) if c == 62 => lemipc::command::email(msg_id),
            Some(c) if c == 27 => lemipc::command::map(c as i32),
            Some(c) if c == 27 => lemipc::command::cheat(c as i32),
            Some(c) if c == 63 => lemipc::command::quit(c as i32),
            Some(c) if c == 37 => lemipc::command::help(c as i32),
            _ => {
                 println!("Command not found.");
            },
        }
    }
}
