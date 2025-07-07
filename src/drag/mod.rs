//! D3 Drag module
//! Provides drag behavior for selections (see d3-drag in JS).

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragState {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub active: bool,
}

impl DragState {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, dx: 0.0, dy: 0.0, active: false }
    }

    pub fn start(&mut self) {
        self.active = true;
        self.dx = 0.0;
        self.dy = 0.0;
    }

    pub fn drag_by(&mut self, dx: f64, dy: f64) {
        if self.active {
            self.x += dx;
            self.y += dy;
            self.dx = dx;
            self.dy = dy;
        }
    }

    pub fn end(&mut self) {
        self.active = false;
        self.dx = 0.0;
        self.dy = 0.0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DragEvent {
    pub event_type: &'static str,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
}


