// Tests for d3-shape symbol generator
use rust_d3::shape::{Symbol, SymbolType};

#[cfg(test)]
pub mod shape_symbol_tests {
    use super::*;

    #[test]
    fn test_symbol_circle() {
        let sym = Symbol::new().symbol_type(SymbolType::Circle).size(100.0);
        let path = sym.to_path();
        assert!(path.starts_with("M"));
        assert!(path.contains("A"));
    }

    #[test]
    fn test_symbol_square() {
        let sym = Symbol::new().symbol_type(SymbolType::Square).size(100.0);
        let path = sym.to_path();
        assert!(path.starts_with("M"));
        assert!(path.contains("H"));
    }

    #[test]
    fn test_symbol_triangle() {
        let sym = Symbol::new().symbol_type(SymbolType::Triangle).size(100.0);
        let path = sym.to_path();
        assert!(path.starts_with("M"));
        assert!(path.contains("L"));
    }

    #[test]
    fn test_symbol_diamond() {
        let sym = Symbol::new().symbol_type(SymbolType::Diamond).size(100.0);
        let path = sym.to_path();
        assert!(path.starts_with("M"));
        assert!(path.contains("L"));
    }

    #[test]
    fn test_symbol_cross() {
        let sym = Symbol::new().symbol_type(SymbolType::Cross).size(100.0);
        let path = sym.to_path();
        assert!(path.starts_with("M"));
        assert!(path.contains("H"));
    }

    #[test]
    fn test_symbol_star() {
        let sym = Symbol::new().symbol_type(SymbolType::Star).size(100.0);
        let path = sym.to_path();
        assert!(path.starts_with("M"));
        assert!(path.contains("L"));
    }

    #[test]
    fn test_symbol_wye() {
        let sym = Symbol::new().symbol_type(SymbolType::Wye).size(100.0);
        let path = sym.to_path();
        assert!(path.starts_with("M"));
        assert!(path.contains("L"));
    }
}
