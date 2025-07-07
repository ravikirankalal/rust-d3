// Ordinal scale
pub struct OrdinalScale<T: Clone, U: Clone> {
    domain: Vec<T>,
    range: Vec<U>,
}

impl<T: PartialEq + Clone, U: Clone> OrdinalScale<T, U> {
    pub fn new(domain: Vec<T>, range: Vec<U>) -> Self {
        Self { domain, range }
    }
    pub fn scale(&self, value: &T) -> Option<U> {
        self.domain.iter().position(|d| d == value).and_then(|i| self.range.get(i).cloned())
    }
}
