use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("sample_codes_list.rs");

    let files = fs::read_dir("sample_codes")
        .expect("Failed to read directory")
        .filter_map(|res| {
            let res = res.expect("Failed to read directory entry");
            let path = res.path();
            if path.is_file() {
                let content =
                    fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                Some((file_name, content))
            } else {
                None
            }
        })
        .collect::<std::collections::BTreeMap<String, String>>();

    let mut f = fs::File::create(&dest_path).expect("Failed to create file");
    writeln!(
        f,
        "pub static SAMPLE_CODES_LIST: [(&str, &str); {}] = [",
        files.len()
    )
    .expect("Failed to write to file");

    files.iter().for_each(|(file_name, content)| {
        writeln!(f, "    ( \"{}\", r#\"{}\"# ),", file_name, content)
            .expect("Failed to write to file");
    });

    writeln!(f, "];").expect("Failed to write to file");

    // 将生成的文件标记为Rust源文件，以便编译器可以编译它
    println!("cargo:rerun-if-changed=sample_codes");
    println!("cargo:rerun-if-changed=src/sample_codes_list.rs");
    println!("cargo:rustc-include={}", dest_path.display());
}
