pub trait Joinable<T> {
    fn join(&mut self, other: T);
}

impl Joinable<Option<String>> for Option<String> {
    fn join(&mut self, other: Option<String>) {
        if other.is_some() {
            *self = other;
        }
    }
}

impl Joinable<Option<Vec<String>>> for Option<Vec<String>> {
    fn join(&mut self, other: Option<Vec<String>>) {
        if self.is_some() && other.is_some() {
            self.as_mut()
                .unwrap()
                .extend(other.as_ref().unwrap().clone())
        } else if other.is_some() {
            *self = other;
        }
    }
}
