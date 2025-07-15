// d3-scale: ScaleOrdinal
// Maps discrete domain values to discrete range values
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct ScaleOrdinal<T, R> 
where 
    T: Clone + Hash + Eq,
    R: Clone,
{
    pub domain: Vec<T>,
    pub range: Vec<R>,
    pub unknown: Option<R>,
    mapping: HashMap<T, R>,
}

impl<T, R> ScaleOrdinal<T, R>
where 
    T: Clone + Hash + Eq,
    R: Clone,
{
    pub fn new(domain: Vec<T>, range: Vec<R>) -> Self {
        let mut mapping = HashMap::new();
        
        // Build mapping from domain to range
        for (i, domain_val) in domain.iter().enumerate() {
            if let Some(range_val) = range.get(i % range.len()) {
                mapping.insert(domain_val.clone(), range_val.clone());
            }
        }
        
        Self { 
            domain, 
            range, 
            unknown: None,
            mapping
        }
    }
    
    pub fn scale(&self, x: &T) -> Option<R> {
        if let Some(value) = self.mapping.get(x) {
            Some(value.clone())
        } else {
            self.unknown.clone()
        }
    }
    
    pub fn domain(&self) -> &Vec<T> {
        &self.domain
    }
    
    pub fn range(&self) -> &Vec<R> {
        &self.range
    }
    
    pub fn unknown(mut self, value: R) -> Self {
        self.unknown = Some(value);
        self
    }
    
    pub fn copy(&self) -> Self {
        Self {
            domain: self.domain.clone(),
            range: self.range.clone(),
            unknown: self.unknown.clone(),
            mapping: self.mapping.clone(),
        }
    }
    
    // Implicit domain extension - adds new domain values as they are encountered
    pub fn scale_implicit(&mut self, x: &T) -> R 
    where 
        R: Default,
    {
        if let Some(value) = self.mapping.get(x) {
            value.clone()
        } else {
            // Add to domain and create mapping
            let range_index = self.domain.len() % self.range.len();
            let range_value = self.range.get(range_index).cloned().unwrap_or_default();
            
            self.domain.push(x.clone());
            self.mapping.insert(x.clone(), range_value.clone());
            
            range_value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ordinal_scale() {
        let scale = ScaleOrdinal::new(
            vec!["a", "b", "c"],
            vec!["red", "green", "blue"]
        );
        
        assert_eq!(scale.scale(&"a"), Some("red"));
        assert_eq!(scale.scale(&"b"), Some("green"));
        assert_eq!(scale.scale(&"c"), Some("blue"));
        assert_eq!(scale.scale(&"d"), None);
    }
    
    #[test]
    fn test_ordinal_scale_cycle() {
        let scale = ScaleOrdinal::new(
            vec!["a", "b", "c", "d"],
            vec!["red", "green", "blue"]
        );
        
        assert_eq!(scale.scale(&"a"), Some("red"));
        assert_eq!(scale.scale(&"b"), Some("green"));
        assert_eq!(scale.scale(&"c"), Some("blue"));
        assert_eq!(scale.scale(&"d"), Some("red")); // Cycles back
    }
    
    #[test]
    fn test_ordinal_scale_unknown() {
        let scale = ScaleOrdinal::new(
            vec!["a", "b", "c"],
            vec!["red", "green", "blue"]
        ).unknown("black");
        
        assert_eq!(scale.scale(&"a"), Some("red"));
        assert_eq!(scale.scale(&"unknown"), Some("black"));
    }
    
    #[test]
    fn test_ordinal_scale_implicit() {
        let mut scale = ScaleOrdinal::new(
            vec!["a", "b"],
            vec!["red", "green", "blue"]
        );
        
        assert_eq!(scale.scale_implicit(&"a"), "red");
        assert_eq!(scale.scale_implicit(&"b"), "green");
        assert_eq!(scale.scale_implicit(&"c"), "blue"); // Implicit extension
        assert_eq!(scale.scale_implicit(&"d"), "red");  // Cycles
        
        // Domain should now include the new values
        assert_eq!(scale.domain().len(), 4);
    }
}
