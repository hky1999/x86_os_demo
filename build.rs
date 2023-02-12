use std::env;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), String> {
    let out_dir = env::var("OUT_DIR").unwrap();

	Command::new("nasm")
		.args(&["src/arch/x86_64/start.asm", "-felf64", "-o"])
		.arg(&format!("{}/start.o", out_dir))
		.status()
		.unwrap();
	Command::new("ar")
		.args(&["crus", "libstart.a", "start.o"])
		.current_dir(&Path::new(&out_dir))
		.status()
		.unwrap();

	println!("cargo:rustc-link-search=native={}", out_dir);
	println!("cargo:rustc-link-lib=static=start");
    Ok(())
}
