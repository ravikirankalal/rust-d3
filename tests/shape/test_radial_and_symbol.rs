use rust_d3::shape::symbol_type::SymbolType;
use rust_d3::shape::{link_radial::LinkRadial, radial_area::RadialArea, radial_line::RadialLine};

#[test]
fn test_link_radial_path() {
    let link = LinkRadial::new();
    let d = link.path((10.0, 0.0), (10.0, std::f64::consts::FRAC_PI_2));
    assert!(d.starts_with("M10.000,0.000L0.000,10.000"));
}

#[test]
fn test_radial_area_path() {
    let area = RadialArea::new();
    let pts = vec![
        (10.0, 0.0),
        (10.0, std::f64::consts::FRAC_PI_2),
        (10.0, std::f64::consts::PI),
    ];
    let d = area.path(&pts);
    assert!(d.starts_with("M10.000,0.000L0.000,10.000L-10.000,0.000Z"));
}

#[test]
fn test_radial_line_path() {
    let line = RadialLine::new();
    let pts = vec![(10.0, 0.0), (10.0, std::f64::consts::FRAC_PI_2)];
    let d = line.path(&pts);
    assert!(d.starts_with("M10.000,0.000L0.000,10.000"));
}

#[test]
fn test_symbol_type_paths() {
    let asterisk = SymbolType::Asterisk;
    let wye = SymbolType::Wye;
    assert!(asterisk.path(10.0).contains("M0,-10"));
    assert!(wye.path(10.0).contains("M0,-10"));
}
