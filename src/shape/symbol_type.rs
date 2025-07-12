// symbol_type.rs
// Implements d3-shape's symbolType, symbolAsterisk, symbolWye

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Circle,
    Cross,
    Diamond,
    Square,
    Star,
    Triangle,
    Wye,
    Asterisk,
}

impl SymbolType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SymbolType::Circle => "circle",
            SymbolType::Cross => "cross",
            SymbolType::Diamond => "diamond",
            SymbolType::Square => "square",
            SymbolType::Star => "star",
            SymbolType::Triangle => "triangle",
            SymbolType::Wye => "wye",
            SymbolType::Asterisk => "asterisk",
        }
    }
    pub fn path(&self, size: f64) -> String {
        match self {
            SymbolType::Asterisk => format!("M0,-{s}L0,{s}M-{s},0L{s},0M-{h},-{h}L{h},{h}M-{h},{h}L{h},-{h}", s=size, h=size/1.414),
            SymbolType::Wye => format!("M0,-{s}L0,{s}M-{w},{h}L{w},-{h}M-{w},-{h}L{w},{h}", s=size, w=size*0.5, h=size*0.866),
            _ => format!("M0,0"), // Placeholder for other symbols
        }
    }
}
