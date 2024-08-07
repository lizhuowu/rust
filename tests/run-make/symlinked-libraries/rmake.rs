// When a directory and a symlink simultaneously exist with the same name,
// setting that name as the library search path should not cause rustc
// to avoid looking in the symlink and cause an error. This test creates
// a directory and a symlink named "other", and places the library in the symlink.
// If it succeeds, the library was successfully found.
// See https://github.com/rust-lang/rust/issues/12459

//@ ignore-cross-compile
//@ needs-symlink

use run_make_support::{dynamic_lib_name, rfs, rustc};

fn main() {
    rustc().input("foo.rs").arg("-Cprefer-dynamic").run();
    rfs::create_dir_all("other");
    rfs::create_symlink(dynamic_lib_name("foo"), "other");
    rustc().input("bar.rs").library_search_path("other").run();
}
