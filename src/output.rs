use serde::Serialize;

const EXIT_JSON_SERIALIZATION_ERROR: i32 = 22;

pub fn print<T: Serialize>(output: &T) -> Result<(), i32> {
    match serde_json::to_string_pretty(output) {
        Ok(json) => {
            println!("{}", json);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error: Failed to serialize output to JSON.");
            eprintln!("Error: {}", err);
            Err(EXIT_JSON_SERIALIZATION_ERROR)
        }
    }
}
