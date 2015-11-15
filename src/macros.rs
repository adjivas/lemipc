// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `exit` macro leaves the program.

#[macro_export]
macro_rules! exit {
    () => ({
        exit!(0)
    });
    ($status: expr) => ({
        unsafe {
            super::ffi::_exit($status)
        }
    });
}

/// The `shm_getboard` returns the shared board,
/// and create the shared board if necessary.

#[macro_export]
macro_rules! shm_getboard {
    () => ({
        match ftok!() {
            Some(key) => shm_getboard!(key),
            None => None,
        }
    });
    ($key: expr) => ({
        extern crate lemipc;
        extern crate std;
        shmget_id! (
            $key,
            std::mem::size_of::<lemipc::board::Map>()
        )
    });
}

/// The `shm_getboard_if_created` returns the shared board
/// if exist.

#[macro_export]
macro_rules! shm_getboard_if_created {
    () => ({
        match ftok!() {
            Some(key) => shm_getboard_if_created!(key),
            None => None,
        }
    });
    ($key: expr) => ({
        extern crate std;

        shmget! (
            $key,
            0o0666,
            std::mem::size_of::<Map>()
        )
    });
}

/// The `ipc_getlem` returns and inits the tuple (msg, sem, shm).

#[macro_export]
macro_rules! ipc_getlem {
    () => ({
        (
            {
                msgget! (
                    ftok!().expect("ftok! fail")
                ).expect("msgget! fail")
            },
            {
                let sem_id: i32 = semget_id! (
                    ftok!().expect("ftok! fail")
                ).expect("semget! fail");

                semctl_init!(sem_id);
                sem_id
            },
            {
                let shm_id = shm_getboard!().expect("shm_getboard! fail");
                let addr = shmat!(shm_id).expect("shmat! fail");
                unsafe {
                    std::mem::transmute(addr)
                }
            }
        )
    })
}
