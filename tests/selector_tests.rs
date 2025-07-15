use rust_d3::selection::{Arena, Selection};
use std::cell::RefCell;
use std::rc::Rc;
use test_case::test_case;

#[test_case("svg", None, &[], true ; "tag selector")]
#[test_case("#chart", Some("chart"), &[], true ; "id selector")]
#[test_case(".axis", None, &["axis", "major"], true ; "class selector")]
#[test_case("svg.axis.major", None, &["axis", "major"], true ; "compound selector")]
#[test_case("*", None, &[], true ; "wildcard selector")]
fn test_selector_parsing(selector: &str, id: Option<&str>, classes: &[&str], expected: bool) {
    let arena = Rc::new(RefCell::new(Arena {
        nodes: slotmap::SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    svg.attr("id", "chart").attr("class", "axis major");

    let node = svg.node().unwrap();

    // Test tag selector
    let selector = "svg";
    let parsed_selector = rust_d3::selection::utils::parse_selector(selector);
    assert!(parsed_selector.matches(
        &node.tag,
        node.attributes.get("id").map(|s| s.as_str()),
        &vec![]
    ));

    // Test id selector
    let id_selector = "#chart";
    let parsed_selector = rust_d3::selection::utils::parse_selector(id_selector);
    assert!(parsed_selector.matches(
        &node.tag,
        node.attributes.get("id").map(|s| s.as_str()),
        &vec![]
    ));

    // Test class selector
    let class_selector = ".axis";
    let parsed_selector = rust_d3::selection::utils::parse_selector(class_selector);
    assert!(parsed_selector.matches(
        &node.tag,
        node.attributes.get("id").map(|s| s.as_str()),
        &vec!["axis".to_string(), "major".to_string()]
    ));

    // Test compound selector
    let compound_selector = "svg.axis.major";
    let parsed_selector = rust_d3::selection::utils::parse_selector(compound_selector);
    assert!(parsed_selector.matches(
        &node.tag,
        node.attributes.get("id").map(|s| s.as_str()),
        &vec!["axis".to_string(), "major".to_string()]
    ));

    // Test wildcard selector
    let wildcard_selector = "*";
    let parsed_selector = rust_d3::selection::utils::parse_selector(wildcard_selector);
    assert!(parsed_selector.matches(
        &node.tag,
        node.attributes.get("id").map(|s| s.as_str()),
        &vec!["axis".to_string(), "major".to_string()]
    ));
}

#[test_case("rect", 2; "tag selector")]
#[test_case(".axis", 2; "class selector")]
#[test_case(".axis.major", 2; "compound class selector")]
#[test_case("#chart", 1; "id selector")]
#[test_case("*", 5; "wildcard selector")]
#[test_case("rect.axis.major", 1; "compound tag and class selector")]
#[test_case("circle", 0; "non-matching selector")]
fn test_d3_selector_examples(selector: &str, expected_len: usize) {
    // Test cases mirroring D3 selector spec examples
    let arena = Rc::new(RefCell::new(Arena {
        nodes: slotmap::SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");

    // Create a structure like:
    // <svg>
    //   <g id="chart" class="axis major">
    //     <rect class="axis major" />
    //     <rect class="minor" />
    //     <line class="tick" />
    //   </g>
    // </svg>

    let mut g = svg.append("g");
    g.attr("id", "chart").attr("class", "axis major");

    let mut rect1 = g.append("rect");
    rect1.attr("class", "axis major");

    let mut rect2 = g.append("rect");
    rect2.attr("class", "minor");

    let mut line = g.append("line");
    line.attr("class", "tick");

    // Test tag selector
    let rects = svg.select_by("rect");
    assert_eq!(rects.len(), 2);

    // Test class selector
    let axis_elements = svg.select_by(".axis");
    assert_eq!(axis_elements.len(), 2); // g and rect1

    // Test compound class selector
    let major_elements = svg.select_by(".axis.major");
    assert_eq!(major_elements.len(), 2); // g and rect1

    // Test id selector
    let chart = svg.select_by("#chart");
    assert_eq!(chart.len(), 1);

    // Test compound selector with tag, class, and id
    let g_chart = svg.select_by("g#chart.axis.major");
    assert_eq!(g_chart.len(), 1);

    // Test wildcard selector
    let all_elements = svg.select_by("*");
    assert_eq!(all_elements.len(), 5); // svg, g, rect1, rect2, line

    // Test compound selector with tag and class
    let rect_major = svg.select_by("rect.axis.major");
    assert_eq!(rect_major.len(), 1);

    // Test non-matching selector
    let missing = svg.select_by("circle");
    assert_eq!(missing.len(), 0);
}

#[test]
fn test_selector_edge_cases() {
    use rust_d3::selection::utils::parse_selector;

    // Test empty selector
    let empty_selector = parse_selector("");
    assert!(empty_selector.tag.is_none());
    assert!(empty_selector.id.is_none());
    assert!(empty_selector.classes.is_empty());
    assert!(!empty_selector.is_wildcard);

    // Test class-only selector
    let class_only = parse_selector(".test");
    assert!(class_only.tag.is_none());
    assert!(class_only.id.is_none());
    assert_eq!(class_only.classes, vec!["test".to_string()]);
    assert!(!class_only.is_wildcard);

    // Test id-only selector
    let id_only = parse_selector("#myid");
    assert!(id_only.tag.is_none());
    assert_eq!(id_only.id, Some("myid".to_string()));
    assert!(id_only.classes.is_empty());
    assert!(!id_only.is_wildcard);

    // Test complex compound selector
    let complex = parse_selector("rect#myid.class1.class2");
    assert_eq!(complex.tag, Some("rect".to_string()));
    assert_eq!(complex.id, Some("myid".to_string()));
    assert_eq!(
        complex.classes,
        vec!["class1".to_string(), "class2".to_string()]
    );
    assert!(!complex.is_wildcard);

    // Test wildcard
    let wildcard = parse_selector("*");
    assert!(wildcard.tag.is_none());
    assert!(wildcard.id.is_none());
    assert!(wildcard.classes.is_empty());
    assert!(wildcard.is_wildcard);
}
