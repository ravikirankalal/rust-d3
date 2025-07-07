// Ordinal scale
pub struct OrdinalScale<T: Clone, U: Clone> {
    domain: Vec<T>,
    range: Vec<U>,
}

impl<T: PartialEq + Clone, U: PartialEq + Clone> OrdinalScale<T, U> {
    pub fn new(domain: Vec<T>, range: Vec<U>) -> Self {
        Self { domain, range }
    }
    pub fn scale(&self, value: &T) -> Option<U> {
        self.domain.iter().position(|d| d == value).and_then(|i| self.range.get(i).cloned())
    }
    pub fn domain(&self) -> &Vec<T> {
        &self.domain
    }
    pub fn range(&self) -> &Vec<U> {
        &self.range
    }
    pub fn set_domain(&mut self, domain: Vec<T>) {
        self.domain = domain;
    }
    pub fn set_range(&mut self, range: Vec<U>) {
        self.range = range;
    }
    pub fn invert(&self, value: &U) -> Option<T> {
        self.range.iter().position(|r| r == value).and_then(|i| self.domain.get(i).cloned())
    }
}
