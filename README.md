# Lem-Ipc

[![travis-badge][]][travis] [![docs-badge][]][docs] [![license-badge][]][license]

#### Screenshot:

![Screen Shot][display-screenshot]

#### How to build:
```shell
git clone --recursive -b master https://github.com/adjivas/lemipc.git lemipc && cd lemipc

- cargo build // Without feature.
- cargo build --features signal // With the signal' feature.
```

#### How to run:
```shell
Without feature:
- cargo run.
With the signal' feature:
- cargo run --features signal
```

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
|__ lib
|   |__ shm@
|   |__ sem@
|   |__ msh@
|   |__ sig@
|   \__ io@
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
*lemipc*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license][license].

[travis-badge]: https://travis-ci.org/adjivas/lemipc.svg?style=flat-square
[travis]: https://travis-ci.org/adjivas/lemipc
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://adjivas.github.io/lemipc/lemipc
[license-badge]: http://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square
[license]: https://github.com/adjivas/lemipc/blob/master/LICENSE
[display-screenshot]: https://raw.githubusercontent.com/adjivas/lemipc/gh-pages/screenshot.gif
[display]: https://github.com/adjivas/lemipc/tree/display
