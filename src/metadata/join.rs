pub trait Joinable<T> {
    fn join(&self, other: T) -> T;
}

impl Joinable<Option<Vec<String>>> for Option<Vec<String>> {
    fn join(&self, other: Option<Vec<String>>) -> Option<Vec<String>> {
        if self.is_some() && other.is_some() {
            let mut vec = Vec::new();
            vec.extend(self.as_ref().unwrap().clone());
            vec.extend(other.as_ref().unwrap().clone());
            Some(vec)
        } else if self.is_some() {
            self.clone()
        } else if other.is_some() {
            other.clone()
        } else {
            None
        }
    }
}
