// Tests for SymbolOutput trait and custom output for all symbol types
use rust_d3::shape::{Symbol, SymbolType, SymbolOutput};

struct CollectPoints(Vec<(f64, f64)>);

impl SymbolOutput for CollectPoints {
    fn move_to(&mut self, x: f64, y: f64) {
        self.0.push((x, y));
    }
    fn arc_to(&mut self, _rx: f64, _ry: f64, x: f64) {
        self.0.push((x, 0.0));
    }
    fn line_to(&mut self, x: f64, y: f64) {
        self.0.push((x, y));
    }
    fn close(&mut self) {
        self.0.push((f64::NAN, f64::NAN));
    }
}

#[cfg(test)]
pub mod shape_symbol_output_tests {
    use super::*;

    #[test]
    fn test_symbol_output_circle() {
        let sym = Symbol::new().symbol_type(SymbolType::Circle).size(100.0);
        let mut points = CollectPoints(vec![]);
        sym.to_custom(&mut points);
        assert!(points.0.len() >= 3); // move_to + 2 arc_to
    }

    #[test]
    fn test_symbol_output_square() {
        let sym = Symbol::new().symbol_type(SymbolType::Square).size(100.0);
        let mut points = CollectPoints(vec![]);
        sym.to_custom(&mut points);
        assert_eq!(points.0.len(), 5); // 4 corners + close
    }

    #[test]
    fn test_symbol_output_diamond() {
        let sym = Symbol::new().symbol_type(SymbolType::Diamond).size(100.0);
        let mut points = CollectPoints(vec![]);
        sym.to_custom(&mut points);
        assert_eq!(points.0.len(), 5);
    }

    #[test]
    fn test_symbol_output_triangle() {
        let sym = Symbol::new().symbol_type(SymbolType::Triangle).size(100.0);
        let mut points = CollectPoints(vec![]);
        sym.to_custom(&mut points);
        assert_eq!(points.0.len(), 4);
    }

    #[test]
    fn test_symbol_output_cross() {
        let sym = Symbol::new().symbol_type(SymbolType::Cross).size(100.0);
        let mut points = CollectPoints(vec![]);
        sym.to_custom(&mut points);
        assert_eq!(points.0.len(), 13);
    }

    #[test]
    fn test_symbol_output_star() {
        let sym = Symbol::new().symbol_type(SymbolType::Star).size(100.0);
        let mut points = CollectPoints(vec![]);
        sym.to_custom(&mut points);
        assert_eq!(points.0.len(), 11);
    }

    #[test]
    fn test_symbol_output_wye() {
        let sym = Symbol::new().symbol_type(SymbolType::Wye).size(100.0);
        let mut points = CollectPoints(vec![]);
        sym.to_custom(&mut points);
        assert_eq!(points.0.len(), 7); // 3 arms: move_to+line_to each, plus close
    }
}
