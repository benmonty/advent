use std::path::PathBuf;
use std::error::Error;

pub fn get_data_path(rel_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pb.push("data/");
    pb.push(&rel_path);
    match pb.is_file() {
        true => Ok(pb),
        false => Err(Box::from(format!("could not resolve path: {}", rel_path))),
    }
}

pub fn get_test_data_path(rel_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let mut pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pb.push("tests/data/");
    pb.push(&rel_path);
    match pb.is_file() {
        true => Ok(pb),
        false => Err(Box::from(format!("could not resolve test path: {}", rel_path))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_path_not_a_file() {
        let result = get_data_path("asdf/foo/bar.txt");
        result.expect_err("should return a file error");
    }

    #[test]
    fn test_data_path_not_a_file() {
        let result = get_test_data_path("asdf/foo/bar.txt");
        result.expect_err("should return a file error");
    }
}
