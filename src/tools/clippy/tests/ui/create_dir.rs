#![allow(unused_must_use)]
#![warn(clippy::create_dir)]

use std::fs::create_dir_all;

fn create_dir() {}

fn main() {
    // Should be warned
    std::fs::create_dir("foo");
    //~^ create_dir
    std::fs::create_dir("bar").unwrap();
    //~^ create_dir

    // Shouldn't be warned
    create_dir();
    std::fs::create_dir_all("foobar");
}

mod issue14994 {
    fn with_no_prefix() {
        use std::fs::create_dir;
        create_dir("some/dir").unwrap();
        //~^ create_dir
    }

    fn with_fs_prefix() {
        use std::fs;
        fs::create_dir("/some/dir").unwrap();
        //~^ create_dir
    }

    fn with_full_prefix() {
        std::fs::create_dir("/some/dir").unwrap();
        //~^ create_dir
    }
}
