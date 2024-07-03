use std::{env, fs};
use std::error::Error;
use std::path::PathBuf;
use bindgen::callbacks::{IntKind, ParseCallbacks};
use bindgen::{AliasVariation, EnumVariation};

#[derive(Debug)]
struct TypeCallbackParser;

impl ParseCallbacks for TypeCallbackParser {
	fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
		match name {
			"BASSVERSION" => {
				Some(IntKind::NewType { name: "DWORD", is_signed: false })
			}
			"BASS_OK" => {
				None
			}
			_ => {
				if name.starts_with("BASS_ERROR") {
					None
				} else {
					Some(IntKind::NewType { name: "DWORD", is_signed: false })
				}
			}
		}
	}
}

pub fn generate_bindings() -> Result<(), Box<dyn Error>> {
	// Write the bindings to the $OUT_DIR/bindings.rs file.
	// let out_path = PathBuf::from(env::var("OUT_DIR")?);
	// fs::create_dir_all(out_path.join("bindings"))?;
	// Nope, for Windows-users sakes just do it to src.
	let out_path = PathBuf::from("./src");

	let callback_parser = Box::new(TypeCallbackParser);

	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
		.dynamic_link_require_all(true)
		.array_pointers_in_arguments(true)
		.default_enum_style(EnumVariation::ModuleConsts)
		.generate_cstr(true)
		.derive_default(true)
		.parse_callbacks(callback_parser)
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
		// .type_alias(".*BYTE.*")
		// .type_alias(".*WORD.*")
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
		// Include the bits
		.raw_line(include_str!("./include/top.rs"))
		// Tell cargo to invalidate the built crate whenever any of the
		// included header files changed.
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

	bindings
		.clone()
		.dynamic_library_name("BASS")
		// The input header we would like to generate
		// bindings for.
		.header("bass.h")
		.raw_line(include_str!("./include/top-ex-bass.rs"))
		// We override the BOOL definition
		.raw_line(include_str!("./include/bool.rs"))
		// And add the necessary STREAMPROC definitions (They require some finangling because Rust doesn't like them.)
		.raw_line(include_str!("./include/streamproc.rs"))
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

	bindings
		.clone()
		.dynamic_library_name("BASS_WASAPI")
		.header("basswasapi.h")
		// Only Allow BASSLoud-specific items
		.allowlist_item(".*WASAPI.*")
		.allowlist_item("BASS_DEVICE_ENABLED")
		.allowlist_item("BASS_DEVICE_DEFAULT")
		.allowlist_item("BASS_DEVICE_INIT")
		.allowlist_item("BASS_DEVICE_LOOPBACK")
		.allowlist_item("BASS_DEVICE_INPUT")
		.allowlist_item("BASS_DEVICE_UNPLUGGED")
		.allowlist_item("BASS_DEVICE_DISABLED")
		// Finish the builder and generate the bindings.
		.generate()
		// Unwrap the Result and panic on failure.
		.expect("Unable to generate bindings")
		.write_to_file(out_path.join("bindings/bass_wasapi.rs"))
		.expect("Couldn't write bindings!");

	Ok(())
}