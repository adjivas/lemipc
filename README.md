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

#### License:
*lemipc*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/lemipc/blob/master/LICENSE).
