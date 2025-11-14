use std::fs;
use std::fs::OpenOptions; // Import OpenOptions for appending
use std::io::{self, Write}; // Import Write for writeln!
use std::path::PathBuf;

pub struct GenFile {
    pub filename: String,
    pub file_path: PathBuf,
    pub generated_content: String,
}

impl GenFile {
    // new
    pub fn new() -> Self {
        GenFile {
            filename: "SafeCode.txt".to_string(),
            file_path: PathBuf::from("./"),
            generated_content: "".to_string(),
        }
    }

    /// Appends the `generated_content` to the file specified by `file_path` and `filename`.
    ///
    /// This function will:
    /// 1. Create the parent directory (e.g., `self.file_path`) if it doesn't already exist.
    /// 2. Open the file for appending, creating it if it doesn't exist.
    /// 3. Write the `self.generated_content` string into that file, followed by a newline.
    ///
    /// Returns an `io::Result<()>` to indicate success or failure
    pub fn create_file(&self) -> io::Result<()> {
        // Build full path to the file
        let full_path = self.file_path.join(&self.filename);

        // Ensure parent directory exists
        if !self.file_path.exists() {
            fs::create_dir_all(&self.file_path)?;
        }

        // Open file for appending (create if doesn't exist)
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&full_path)?;

        // Write the content followed by newline
        writeln!(file, "{}", &self.generated_content)?;
        Ok(())
    }
}

impl Default for GenFile {
    fn default() -> Self {
        GenFile {
            filename: "test.txt".to_string(),
            file_path: PathBuf::from("./"),
            generated_content: "Hello world!".to_string(),
        }
    }
}
// NOTE: generates code
// fill the file with generated content
// then (optional)
// encrypt for online storing
