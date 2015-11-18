// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate lemipc;

#[allow(unused_unsafe, unused_assignments)]
#[cfg(feature = "signal")]
fn main () {
    let pid: i32 = getpid!();
    let (msg_id, sem_id, ref mut shm_map):(
        i32,
        i32,
        &mut lemipc::board::Map
    ) = ipc_getlem!();

    signal!(sig::ffi::Sig::USR1, lemipc::command::receive);
    signal!(sig::ffi::Sig::USR2, lemipc::command::turn);
    signal!(sig::ffi::Sig::KILL, lemipc::command::quit);
    signal!(sig::ffi::Sig::INT, lemipc::command::quit);
    shm_map.spawn_pawn(pid);
    lemipc::command::start(&shm_map, pid);
    loop {
        let command = read_command!();

        semop_lock!(sem_id);
        match (shm_map.count_enemy(pid), command) {
            (Some(e), _) if e >=  2 => lemipc::command::quit(e as i32),
            (_, Some(c)) if c == 28 => lemipc::command::start(shm_map, pid),
            (_, Some(c)) if c == 29 => lemipc::command::turn(c as i32),
            (_, Some(c)) if c == 25 => lemipc::command::play(*shm_map, pid),
            (_, Some(c)) if c == 14 => lemipc::command::email(msg_id),
            (_, Some(c)) if c == 27 => lemipc::command::receive(c as i32),
            (_, Some(c)) if c == 22 => lemipc::command::map(shm_map, pid),
            (_, Some(c)) if c == 12 => lemipc::command::cheat(shm_map),
            (_, Some(c)) if c == 32 => lemipc::command::whoiam(pid),
            (_, Some(c)) if c == 24 => lemipc::command::score(shm_map),
            (_, Some(c)) if c == 17 => lemipc::command::help(c as i32),
            (_, Some(c)) if c == 26 => lemipc::command::quit(c as i32),
            _ => {},
        };
        semop_unlock!(sem_id);
    }
}

#[allow(unused_unsafe, unused_assignments)]
#[cfg(not(feature = "signal"))]
fn main () {
    let pid: i32 = getpid!();
    let (msg_id, sem_id, ref mut shm_map):(
        i32,
        i32,
        &mut lemipc::board::Map
    ) = ipc_getlem!();

    signal!(sig::ffi::Sig::KILL, lemipc::command::quit);
    signal!(sig::ffi::Sig::INT, lemipc::command::quit);
    shm_map.spawn_pawn(pid);
    lemipc::command::start(shm_map, pid);
    loop {
        let command = read_command!();

        semop_lock!(sem_id);
        match (shm_map.count_enemy(pid), command) {
            (Some(e), _) if e >=  2 => lemipc::command::quit(e as i32),
            (_, Some(c)) if c == 28 => lemipc::command::start(shm_map, pid),
            (_, Some(c)) if c == 29 => lemipc::command::turn(shm_map),
            (_, Some(c)) if c == 25 => lemipc::command::play(*shm_map, pid),
            (_, Some(c)) if c == 14 => lemipc::command::email(msg_id),
            (_, Some(c)) if c == 27 => lemipc::command::receive(c as i32),
            (_, Some(c)) if c == 22 => lemipc::command::map(shm_map, pid),
            (_, Some(c)) if c == 12 => lemipc::command::cheat(shm_map),
            (_, Some(c)) if c == 32 => lemipc::command::whoiam(pid),
            (_, Some(c)) if c == 24 => lemipc::command::score(shm_map),
            (_, Some(c)) if c == 17 => lemipc::command::help(c as i32),
            (_, Some(c)) if c == 26 => lemipc::command::quit(c as i32),
            _ => {},
        };
        semop_unlock!(sem_id);
    }
}
