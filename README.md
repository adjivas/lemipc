# Lem-Ipc

[![Build Status](https://travis-ci.org/adjivas/lemipc.svg)](https://travis-ci.org/adjivas/lemipc)
[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)

#### How to build:
```shell
git clone https://github.com/adjivas/lemipc.git lem-ipc && cd lem-ipc
- cargo run // Without feature.
- cargo build --features signal // With the signal' feature.
```

#### How to run:
Without feature:
- cargo run.
With the signal' feature:
- cargo run --features signal

#### Cargo'git-Dependencies:
```shell
Shm   Sem Msg Sig   Io
  \____ \  |  / __ /
        LemIpc
```

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
\__ src
    |__ board
    |   |__ cell.rs
    |   |__ map.rs
    |   \__ mod.rs
    |__ command
    |   |__ compass.rs
    |   |__ run.rs
    |   \__ mod.rs
    |__ macros.rs
    |__ ffi.rs
    \__ lib.rs
```

#### Function-used:
```shell
$ nm target/debug/lemipc  | grep " U "
U _Unwind_Backtrace@@GCC_3.3
U _Unwind_DeleteException@@GCC_3.0
U _Unwind_FindEnclosingFunction@@GCC_3.3
U _Unwind_GetIPInfo@@GCC_4.2.0
U _Unwind_RaiseException@@GCC_3.0
U _Unwind_Resume@@GCC_3.0
U __cxa_atexit@@GLIBC_2.2.5
U __errno_location@@GLIBC_2.2.5
U __gcc_personality_v0@@GCC_3.3.1
U __libc_start_main@@GLIBC_2.2.5
U __rawmemchr@@GLIBC_2.2.5
U __register_atfork@@GLIBC_2.3.2
U __tls_get_addr@@GLIBC_2.3
U _exit@@GLIBC_2.2.5
U abort@@GLIBC_2.2.5
U bsearch@@GLIBC_2.2.5
U close@@GLIBC_2.2.5
U dl_iterate_phdr@@GLIBC_2.2.5
U fcntl@@GLIBC_2.2.5
U ftok@@GLIBC_2.2.5
U getenv@@GLIBC_2.2.5
U getpagesize@@GLIBC_2.2.5
U getpid@@GLIBC_2.2.5
U kill@@GLIBC_2.2.5
U madvise@@GLIBC_2.2.5
U memcmp@@GLIBC_2.2.5
U memcpy@@GLIBC_2.14
U memmove@@GLIBC_2.2.5
U memset@@GLIBC_2.2.5
U mmap@@GLIBC_2.2.5
U msgctl@@GLIBC_2.2.5
U msgget@@GLIBC_2.2.5
U msgrcv@@GLIBC_2.2.5
U msgsnd@@GLIBC_2.2.5
U munmap@@GLIBC_2.2.5
U open@@GLIBC_2.2.5
U pthread_attr_destroy@@GLIBC_2.2.5
U pthread_attr_getstack@@GLIBC_2.2.5
U pthread_attr_init@@GLIBC_2.2.5
U pthread_cond_destroy@@GLIBC_2.3.2
U pthread_getattr_np@@GLIBC_2.2.5
U pthread_getspecific@@GLIBC_2.2.5
U pthread_key_create@@GLIBC_2.2.5
U pthread_key_delete@@GLIBC_2.2.5
U pthread_mutex_destroy@@GLIBC_2.2.5
U pthread_mutex_init@@GLIBC_2.2.5
U pthread_mutex_lock@@GLIBC_2.2.5
U pthread_mutex_unlock@@GLIBC_2.2.5
U pthread_mutexattr_destroy@@GLIBC_2.2.5
U pthread_mutexattr_init@@GLIBC_2.2.5
U pthread_mutexattr_settype@@GLIBC_2.2.5
U pthread_self@@GLIBC_2.2.5
U pthread_setspecific@@GLIBC_2.2.5
U read@@GLIBC_2.2.5
U readlink@@GLIBC_2.2.5
U sbrk@@GLIBC_2.2.5
U shmat@@GLIBC_2.2.5
U shmctl@@GLIBC_2.2.5
U shmdt@@GLIBC_2.2.5
U shmget@@GLIBC_2.2.5
U sigaction@@GLIBC_2.2.5
U sigaltstack@@GLIBC_2.2.5
U signal@@GLIBC_2.2.5
U snprintf@@GLIBC_2.2.5
U strchr@@GLIBC_2.2.5
U strcmp@@GLIBC_2.2.5
U strerror_r@@GLIBC_2.2.5
U strlen@@GLIBC_2.2.5
U strncmp@@GLIBC_2.2.5
U strncpy@@GLIBC_2.2.5
U strnlen@@GLIBC_2.2.5
U strtol@@GLIBC_2.2.5
U syscall@@GLIBC_2.2.5
U sysconf@@GLIBC_2.2.5
U write@@GLIBC_2.2.5
```

#### License:
*lemipc*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/lemipc/blob/master/LICENSE).
