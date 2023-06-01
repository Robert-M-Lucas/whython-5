/*
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::AtomicBool;
    use ignore_result::Ignore;
    use walkdir::WalkDir;
    use whython_5::execution::execute;
    use whython_5::memory_manager::MemoryManager;
    use whython_5::translator::translate;

    #[test]
    fn test_instruction_implementation() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/processing/instructions");
        let count = WalkDir::new(d)
            .into_iter()
            .filter(|f| {
                if f.is_err() {
                    return false;
                }
                if f.as_ref().unwrap().file_type().is_dir() {
                    return false;
                }
                true
            })
            .count() as u16;

        println!("Found {} instructions", count);

        let mut dummy_data: [u8; 100] = [0; 100];

        for i in 0..count {
            let encoded_code = i.to_le_bytes();
            for (j, b) in encoded_code.iter().enumerate() {
                dummy_data[j] = *b;
            }

            translate(&dummy_data, true);
        }

        let atomic_bool = AtomicBool::from(true);

        for i in 0..count {
            // ! Skip input instruction test
            if i == 15 {
                continue;
            }

            let encoded_code = i.to_le_bytes();
            for (j, b) in encoded_code.iter().enumerate() {
                dummy_data[j] = *b;
            }

            execute(
                &mut MemoryManagers {
                    variable_memory: MemoryManager::from_vec(Vec::from(dummy_data)),
                    program_memory: MemoryManager::from_vec(Vec::from(dummy_data)),
                },
                &atomic_bool,
            )
            .ignore();
        }
    }
}
*/
