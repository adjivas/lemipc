// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/lemipc
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `C` extern is list of libc functions required
/// by the project.

#[cfg(any(unix))]
extern "C" {
    pub fn _exit(status: i32);
}
