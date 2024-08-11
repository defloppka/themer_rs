mod core;

pub fn main() {
    
}

#[cfg(test)]
mod tests {
    use std::{cell::LazyCell, path::PathBuf};

    use crate::core::create_program;

    const DIR_PATH: LazyCell<PathBuf> = LazyCell::new(|| {
        PathBuf::from("D:/projects/themer_rs/examples")
    });

    const FILE_PATH: LazyCell<PathBuf> = LazyCell::new(|| {
        PathBuf::from("D:/projects/themer_rs/examples/theme.th")
    });

    #[test]
    fn test_themer_program() {
        let program = create_program(&FILE_PATH);
    }
}