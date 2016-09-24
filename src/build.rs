extern crate rust_mvc;

use rust_mvc::build;

fn main(){
	let mut config = build::Config::new();
	config.root_dir = format!("{}/src/", env!("CARGO_MANIFEST_DIR"));

	build::build(config);
}