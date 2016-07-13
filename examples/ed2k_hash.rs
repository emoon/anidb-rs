extern crate anidb;

use std::env;
use anidb::ed2k::Ed2kHash;

fn main () {
    let filename = env::args().nth(1).unwrap();
    Ed2kHash::hash_file(&filename).unwrap();
}
