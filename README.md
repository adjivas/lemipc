# Lem-Ipc

[![Build Status](https://travis-ci.org/adjivas/lemipc.svg)](https://travis-ci.org/adjivas/lemipc)
[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)

#### How to build:
```shell
git clone https://github.com/adjivas/lemipc.git lem-ipc && cd lem-ipc
- cargo run // whiteout feature.
- cargo build --features signal // white the signal' feature.
```

#### How to run:
Whiteout feature:
- cargo run.
white the signal' feature:
- cargo run --features signal

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
    |   \__ lib.rs
    |   \__ mod.rs
    |__ macros.rs
    |__ ffi.rs
    \__ lib.rs
```

#### License:
*lemipc*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/lemipc/blob/master/LICENSE).
