//! Unit tests for d3 voronoi (placeholder)
use rust_d3::voronoi::{VoronoiDiagram, VoronoiCell};

#[test]
fn test_voronoi_diagram() {
    let sites = vec![(0.0, 0.0), (1.0, 1.0)];
    let diagram = VoronoiDiagram::new(&sites);
    assert_eq!(diagram.cells.len(), 2);
    assert_eq!(diagram.cells[0].site, (0.0, 0.0));
    assert_eq!(diagram.cells[1].site, (1.0, 1.0));
    for cell in &diagram.cells {
        assert!(!cell.region.is_empty());
    }
}

#[test]
fn test_voronoi_find() {
    let sites = vec![(0.0, 0.0), (10.0, 0.0)];
    let diagram = VoronoiDiagram::new(&sites);
    let found = diagram.find(0.1, 0.1).unwrap();
    assert_eq!(found.site, (0.0, 0.0));
    let found = diagram.find(9.9, 0.1).unwrap();
    assert_eq!(found.site, (10.0, 0.0));
}

#[test]
fn test_voronoi_cell_polygons() {
    let sites = vec![(0.0, 0.0), (10.0, 0.0)];
    let diagram = VoronoiDiagram::new(&sites);
    let polys = diagram.cell_polygons();
    assert_eq!(polys.len(), 2);
    for poly in polys {
        assert!(!poly.is_empty());
    }
}

#[test]
fn test_voronoi_empty() {
    let sites: Vec<(f64, f64)> = vec![];
    let diagram = VoronoiDiagram::new(&sites);
    assert_eq!(diagram.cells.len(), 0);
}
