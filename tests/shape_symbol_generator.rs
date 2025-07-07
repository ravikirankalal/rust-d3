use rust_d3::shape::SymbolGenerator;

#[test]
fn test_symbol_generator() {
    let circle = SymbolGenerator::new().symbol_type("circle").size(100.0).generate();
    assert!(circle.contains("A"));
    let square = SymbolGenerator::new().symbol_type("square").size(100.0).generate();
    assert!(square.contains("h"));
    let triangle = SymbolGenerator::new().symbol_type("triangle").size(100.0).generate();
    assert!(triangle.contains("L"));
}
