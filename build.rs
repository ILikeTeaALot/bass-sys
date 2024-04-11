mod src_build;
use src_build::*;

use std::error::Error;
// use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    generate_bindings();
	prepare_docs()
}
