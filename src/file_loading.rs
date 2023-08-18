use std::fs;

pub fn load_file(input_file: &str) -> Result<String, String> {
    match fs::read_to_string(input_file) {
        Err(e) => {
            // Err(if input_defaulted {
            //     format!(
            //             "Error reading file '{}' - {}. You did not specify a file so '{}' was used as a default.",
            //             input_file,
            //             e.to_string(),
            //             input_file
            //         )
            // } else {
            Err(format!("Error reading file '{}' - {}", input_file, e))
            // })
        }
        Ok(value) => Ok(value),
    }
}
