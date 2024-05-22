use std::env;
use std::fs;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "macos" {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dst = format!("{}/../../../../", out_dir);
        let cargo_bin = env::var("CARGO_BIN_EXE_my_executable").unwrap_or_default();

        if !cargo_bin.is_empty() {
            fs::write(
                format!("{}/{}.x86_64-apple-darwin.rustflags", dst, cargo_bin),
                "-C link-arg=-Wl,-rpath,@executable_path/../lib",
            )
            .unwrap();
        }
    }
}
