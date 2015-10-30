// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate lemipc;

mod board;

#[allow(unused_unsafe, unused_assignments)]
#[cfg(feature = "signal")]
fn main () {
    let pid: i32 = getpid!();
    let msg_id: i32 = msgget! (
        ftok!().unwrap()
    ).unwrap();
    let map: &mut lemipc::board::Map = {
        let id = shm_getboard!().unwrap();
        let addr = shmat!(id).unwrap();

        unsafe {
            std::mem::transmute(addr)
        }
    };

    signal!(sig::ffi::Sig::USR1, lemipc::command::receive);
    signal!(sig::ffi::Sig::KILL, lemipc::command::quit);
    signal!(sig::ffi::Sig::INT, lemipc::command::quit);
    map.spawn_pawn(pid);
    lemipc::command::start(&map, pid);
    loop {
        match read_command!() {
            Some(c) if c == 28 => lemipc::command::start(&map, pid),
            Some(c) if c == 25 => lemipc::command::play(c as i32),
            Some(c) if c == 14 => lemipc::command::email(msg_id),
            Some(c) if c == 27 => lemipc::command::receive(c as i32),
            Some(c) if c == 22 => lemipc::command::map(&map, pid),
            Some(c) if c == 12 => lemipc::command::cheat(&map),
            Some(c) if c == 32 => lemipc::command::whoiam(pid),
            Some(c) if c == 24 => lemipc::command::score(&map),
            Some(c) if c == 17 => lemipc::command::help(c as i32),
            Some(c) if c == 26 => lemipc::command::quit(c as i32),
            _ => {},
        }
    }
}

#[allow(unused_unsafe, unused_assignments)]
#[cfg(not(feature = "signal"))]
fn main () {
    println!("The feature isn't activated :/");
}
