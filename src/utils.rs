#[derive(Debug, Clone)]
pub struct CodeLoc {
    file: String,
    line: u32,
}

impl CodeLoc {
    pub fn new<T: ToString>(file: T, line: u32) -> Self {
        Self {
            file: file.to_string(),
            line,
        }
    }
}

impl std::fmt::Display for CodeLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.file, self.line)
    }
}

#[macro_export]
macro_rules! code_location {
    () => { crate::utils::CodeLoc::new(file!(), line!()) }
}
