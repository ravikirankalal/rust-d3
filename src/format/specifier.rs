#[derive(Debug, Default, PartialEq, Clone)]
pub struct FormatSpecifier {
    pub fill: Option<char>,
    pub align: Option<char>,
    pub sign: Option<char>,
    pub symbol: Option<char>,
    pub zero: bool,
    pub width: Option<usize>,
    pub comma: bool,
    pub grouping: Option<char>, // e.g. '_'
    pub parentheses: bool,      // accounting negative numbers
    pub precision: Option<usize>,
    pub ty: char,
    pub locale: Option<String>, // for locale support
    pub currency: Option<(String, String)>, // (prefix, suffix)
}
