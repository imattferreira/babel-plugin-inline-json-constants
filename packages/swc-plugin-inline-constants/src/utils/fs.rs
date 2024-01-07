use std::path::Path;

pub fn is_constantify(expr: &str) -> bool {
    expr.eq("constantify")
}

pub fn to_constants_file_path(file: &str) -> String {
    "./constants/".to_owned() + file
}

pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}
