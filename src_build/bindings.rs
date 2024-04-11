use std::{env, fs};
use std::error::Error;
use std::path::PathBuf;
use bindgen::{AliasVariation, EnumVariation};

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
	// Write the bindings to the $OUT_DIR/bindings.rs file.
	let out_path = PathBuf::from(env::var("OUT_DIR")?);
	fs::create_dir_all(out_path.join("bindings"))?;
	// let out_path = PathBuf::from("./src");

	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
		.dynamic_link_require_all(true)
		.array_pointers_in_arguments(true)
		.default_enum_style(EnumVariation::ModuleConsts)
		.generate_cstr(true)
		.derive_default(true)
		// Filter just BASS items
		// .allowlist_function("BASS.*")
		// .allowlist_item("BASS.*")
		// .allowlist_var("BASS.*")
		// BASS_INFO flags (from DSOUND.H)
		// .allowlist_item("DSCAPS.*")
		// BASS_RECORDINFO flags (from DSOUND.H)
		// .allowlist_item("DSCCAPS.*")
		// defines for formats field of BASS_RECORDINFO (from MMSYSTEM.H)
		// .allowlist_item("WAVE.*")
		// Unsure whether this should be set...
		.detect_include_paths(false)
		// Tags aren't done yet.
		.blocklist_item("TAG.*")
		.blocklist_item("TRUE")
		.blocklist_item("FALSE")
		.blocklist_item("BOOL")
		// .default_visibility(FieldVisibilityKind::Private)
		.no_copy(".*PROC.*")
		.type_alias(".*BYTE.*")
		.type_alias(".*WORD.*")
		.type_alias(".*PROC.*")
		// .new_type_alias_deref(".*STREAMPROC.*")
		.new_type_alias(".*STREAMPROC.*")
		// .type_alias("BOOL")
		// Impls and Derives
		.derive_partialeq(true)
		.derive_eq(true)
		.derive_hash(true)
		.impl_debug(true)
		.clang_args(["-fparse-all-comments"])
		.default_alias_style(AliasVariation::NewTypeDeref)
		// Tell cargo to invalidate the built crate whenever any of the
		// included header files changed.
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

	bindings
		.clone()
		.dynamic_library_name("BASS")
		// The input header we would like to generate
		// bindings for.
		.header("bass.h")
		// We override the BOOL definition
		.raw_line(include_str!("./bool.rs"))
		.raw_line(include_str!("./streamproc.rs"))
		// .raw_line(include_str!("./makelong.rs"))
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings")
		.write_to_file(out_path.join("bindings/bass.rs"))
		.expect("Couldn't write bindings!");

	let bindings = bindings
		// Block bass.h items
		.blocklist_item(".*WORD.*")
		.blocklist_item("BYTE")
		// For non-core files
		.raw_line("use super::bass::*;");

	bindings
		.clone()
		.dynamic_library_name("BASS_Mix")
		.header("bassmix.h")
		// Block bass.h items
		.blocklist_item(".*PROC.*")
		.blocklist_item("H.*")
		// Same as above, but `allow` because... reasons.
		.allowlist_item(".*Mixer.*")
		.allowlist_item(".*Split.*")
		.allowlist_item(".*MIXER.*")
		.allowlist_item(".*SPLIT.*")
		.allowlist_item(".*BASSMIX.*")
		.allowlist_item("BASS_ACTIVE_WAITING")
		.allowlist_item("BASS_ACTIVE_QUEUED")
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings")
		.write_to_file(out_path.join("bindings/bass_mix.rs"))
		.expect("Couldn't write bindings!");

	bindings
		.clone()
		.dynamic_library_name("BASS_CD")
		.header("basscd.h")
		// Block bass.h items
		.blocklist_item("H.*")
		.blocklist_item("BASS_INPUT_TYPE_CD")
		// Filter to only BASS_CD items
		.allowlist_item(".*CD.*")
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings")
		.write_to_file(out_path.join("bindings/bass_cd.rs"))
		.expect("Couldn't write bindings!");

	bindings
		.clone()
		.dynamic_library_name("BASS_Loud")
		.header("bassloud.h")
		// Only Allow BASSLoud-specific items
		.allowlist_item(".*LOUD.*")
		.allowlist_item(".*Loud.*")
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings")
		.write_to_file(out_path.join("bindings/bass_loud.rs"))
		.expect("Couldn't write bindings!");

	Ok(())
}