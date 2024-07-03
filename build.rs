mod src_build;
use src_build::*;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	#[cfg(not(target_os = "windows"))]
    generate_bindings()?;
	prepare_docs()?;
	Ok(())
}
