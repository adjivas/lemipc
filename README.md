# Lem-Ipc

[![travis-badge][]][travis] [![license-badge][]][license]

#### How to build:
```shell
git clone --recursive -b display https://github.com/adjivas/lemipc.git lemipc && cd lemipc
cargo build
```

#### How to run:
```shell
cargo run
```

#### Cargo'git-Dependencies:
```shell
  LemIpc Shm  Sem
     \   /   /
 LemIpc-display
```

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
|__ lib
|   |__ sem@
|   |__ Shm@
|   \__ lemipc@
\__ src
    |__ option.rs
    |__ lib.rs
    \__ main.rs
```

#### License:
*lemipc*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license][license].

[travis-badge]: https://travis-ci.org/adjivas/lemipc.svg?style=flat-square
[travis]: https://travis-ci.org/adjivas/lemipc?branch=display
[license-badge]: http://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square
[license]: https://github.com/adjivas/lemipc/blob/display/LICENSE
