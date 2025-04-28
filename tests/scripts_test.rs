use std::fs;
use std::path::Path;

use fellow::interpret;

// This is the helper function that runs the test logic for a single script.
// It needs to be public so the generated test functions can call it.
pub fn run_single_script_test(script_path: &Path) {
    let script_content = fs::read_to_string(script_path)
        .unwrap_or_else(|_| panic!("Failed to read script file: {}", script_path.display()));

    // Extract the expected output from the first line comment
    let mut lines = script_content.lines();
    let first_line = lines
        .next()
        .unwrap_or_else(|| panic!("Script file is empty: {}", script_path.display()));

    let expected_prefix = "// expected: ";
    let expected_output = if let Some(output) = first_line.strip_prefix(expected_prefix) {
        output.trim().to_string()
    } else {
        // If the first line doesn't have the expected format, panic the test
        panic!(
            "Error: First line of {} does not start with '{}'",
            script_path.display(),
            expected_prefix
        );
    };

    let actual_output = match interpret(&script_content) {
        Ok(output) => format!("{}", output),
        Err(err) => {
            // If your interpreter returns an error, treat it as an interpretation failure
            // and potentially include the error message in the "actual output" for assertion,
            // or panic directly. Panicking is simpler for tests.
            panic!(
                "Interpreter Error running script {}: {:?}",
                script_path.display(),
                err
            );
        }
    };

    // Compare actual output with expected output
    // Use assert_eq! directly; it will cause the calling #[test] function to fail
    assert_eq!(
        actual_output,
        expected_output,
        "Mismatch for script: {}",
        script_path.display()
    );
}

// Include the generated test functions.
// build.rs writes the #[test] functions to OUT_DIR/generated_tests.rs
include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
