use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // Tell Cargo that if the contents of scripts/ changes, rerun this build script.
    println!("cargo::rerun-if-changed=scripts/");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let destination = Path::new(&out_dir).join("generated_tests.rs");
    let mut test_file =
        fs::File::create(&destination).expect("Failed to create generated_tests.rs");

    let scripts_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts");

    // Read all entries in the scripts directory
    let entries = fs::read_dir(&scripts_dir).expect("Failed to read scripts directory");

    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        // Only process files
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();

            // Create a valid Rust identifier for the test function name
            let test_fn_name = format!(
                "script_{}",
                file_name
                    .strip_suffix(".fellow")
                    .unwrap()
                    .replace(".", "_")
                    .replace("-", "_")
                    .replace(" ", "_")
                    .to_lowercase()
                    .trim_end_matches('_') // Remove trailing underscores from original extension
            );
            // Write the formatted test function string to the file
            // // Define the path to the script file. Using r#"{}"# inside r## "{}" ##
            // correctly handles potential quotes or backslashes in the path.
            writeln!(
                test_file,
                r##"
#[test]
fn {}() {{
    let script_path = std::path::Path::new(r#"{}"#);
    run_single_script_test(script_path);
}}"##,
                test_fn_name,   // Fills the first {} (function name)
                path.display()  // Fills the second {} (script path)
            )
            .expect("Failed to write test function");
        }
    }
}
