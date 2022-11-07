/*
 * ls' millennial cousin.
 * program made in order to learn how rust works.
 * warning: it probably contains nonsense to a senior Rust developer; the transition from 4 years of coding EXCLUSIVELLY in Typescript to Rust is HARD.
 *
 * - Giuseppe D'Amico
 */

extern crate colored;

mod file;
mod path;

use std::{env, io::Error};
use path::Path;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let default_path = String::from("./");
    let to_folder: &String = match args.get(1) { 
        Some(e) => e,
        None => &default_path
    };

    let mut path = Path {..Default::default()};
    path.show_path(to_folder);

    Ok(())
}
