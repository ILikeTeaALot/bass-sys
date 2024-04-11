use html5ever::tree_builder::TreeSink;
use scraper::*;
use std::{env, error::Error, ffi::OsString, fs, path::PathBuf};

pub fn prepare_docs() -> Result<(), Box<dyn Error>> {
	let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("doc");
	println!(
		"cargo:warning=Doc root path: {:?}",
		root
	);
	{
		println!("cargo:warning=Content of OUT_DIR {:?}", fs::read_dir(env::var("OUT_DIR").unwrap())?)
	}
	for entry in fs::read_dir(root)? {
		let path = entry?.path();
		if path.is_dir() {
			let dir = path;
			for entry in fs::read_dir(dir.clone())? {
				let path = entry?.path();
				if !path.is_file() {
					continue;
				}
				let extension = path.extension();
				match extension {
					Some(ext) => {
						if ext != OsString::from("html") {
							continue;
						}
					}
					None => continue,
				}
				process_file(&dir, &path)?;
			}
		}
	}
	Ok(())
}

fn process_file(dir: &PathBuf, path: &PathBuf) -> Result<(), Box<dyn Error>> {
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	if let Ok(file) = fs::read_to_string(path) {
		let mut file = if let Some(index) = file.find('\n') {
			file[index + 1..]
				.replace('\r', "")
				.replace("<p>", "\n")
				.replace("<h1>", "# ")
				.replace("</h1>", "\n")
				.replace("<h2>", "## ")
				.replace("</h2>", "\n")
				// Fix markdown in tables
				.replace("<td>", "\n\n<td>\n\n")
				.replace("<tr>", "\n\n<tr>\n\n")
				// Remove excessive newlines
				.replace("\n\n\n\n", "\n\n")
				.replace("\n\n\n", "\n\n")
				.replace("\n\n\n", "\n\n")
				.replace(">\n\n<", "><")
				.to_owned()
		} else {
			file
		};
		// println!("{}", file);
		// {
		// 	if let Ok(mut dom) = tl::parse(&file, tl::ParserOptions::default()) {
		// 		if let Some(anchor) = dom.query_selector("a[href]")
		// 			.expect("Failed to parse query selector")
		// 			.next() {
		// 			let parser_mut = dom.parser_mut();
		//
		// 			let anchor = anchor.get_mut(parser_mut)
		// 				.expect("Failed to resolve node")
		// 				.as_tag_mut()
		// 				.expect("Failed to cast Node to HTMLTag");
		//
		// 			let item_name = anchor.inner_text(parser_mut);
		//
		// 			let attributes = anchor.attributes_mut();
		//
		// 			attributes.get_mut("href")
		// 				.flatten()
		// 				.expect("Attribute not found or malformed")
		// 				.set("http://localhost/about");
		// 		}
		// 	}
		// }
		let mut document = Html::parse_fragment(file.as_str());
		{
			let selector = Selector::parse("a").unwrap();
			// let node_ids: Vec<_> = document.select(&selector).map(|x| x.id()).collect();
			// for id in node_ids {
			// 	document.remove_from_parent(&id);
			// }
			let anchors: Vec<_> = document.select(&selector).collect();
			for a in anchors {
				let item: Vec<_> = a.text().collect();
				let item = item.join("");
				println!("cargo:warning=Item text: {}", item);
				let raw = a.html();
				if item.starts_with("B") {
					// This is horrible, but it works! and it's a build script so not the end of the world...
					file = file.replace(raw.as_str(), format!("[`{}`](crate::{})", &item, &item).as_str());
				} else {
					file = file.replace(raw.as_str(), item.as_str());
				}
			}
			let t_selector = Selector::parse("table").unwrap();
			let tables: Vec<_> = document.select(&t_selector).collect();
		}
		// println!("cargo:warning=Parent: {:?}", path.parent());
		let final_path = out_path
			.join("doc")
			// Should-be directory-name
			.join(dir.file_name().unwrap())
			// File name
			.join(path.with_extension("md").file_name().unwrap());
		// println!(
		// 	"cargo:warning=Hopefully a correct destination?: {:?}",
		// 	final_path.as_path()
		// );
		// println!(
		// 	"cargo:warning=Hopefully a complete directory?: {:?}",
		// 	&final_path.parent().unwrap()
		// );
		fs::create_dir_all(&final_path.parent().unwrap())?;
		// println!("cargo:warning=Successfully created dir?");
		fs::File::create(&final_path)?;
		// println!("cargo:warning=Successfully created file?");
		fs::write(
			&final_path,
			// document.html()
			file
		)?;
		// println!("cargo:warning=Successfully wrote file?");
	}
	Ok(())
}
