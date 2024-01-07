use swc_core::atoms::Atom;

pub fn last_index<T>(vector: &Vec<T>) -> usize {
    vector.len() - 1
}

pub fn split(path: &Atom) -> Vec<&str> {
    path.split(".").collect::<Vec<&str>>()
}
