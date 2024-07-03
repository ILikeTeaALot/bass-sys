mod src_build;
use src_build::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	#[cfg(not(target = "windows"))]
    generate_bindings()?;
	#[cfg(not(target = "windows"))]
	prepare_docs()?;
	Ok(())
}
