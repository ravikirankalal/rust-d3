//! D3 Brush module
//! Provides interactive brushing for selections (see d3-brush in JS).

use std::ops::Range;

pub struct Brush {
    pub extent: Range<f64>,
    pub selection: Option<Range<f64>>,
    // Internal fields for advanced features
    _filter: Option<Box<dyn Fn() -> bool>>,
    _handle_size: Option<f64>,
    _listeners: Vec<(String, Box<dyn Fn(BrushEvent)>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrushEvent {
    pub event_type: String,
    pub selection: Option<Range<f64>>,
}

impl Brush {
    pub fn new(extent: Range<f64>) -> Self {
        Self {
            extent,
            selection: None,
            _filter: None,
            _handle_size: None,
            _listeners: Vec::new(),
        }
    }

    pub fn select(&mut self, range: Range<f64>) {
        if range.start >= self.extent.start && range.end <= self.extent.end {
            self.selection = Some(range);
        } else {
            self.selection = None;
        }
    }

    pub fn clear(&mut self) {
        self.selection = None;
    }

    pub fn is_active(&self) -> bool {
        self.selection.is_some()
    }

    /// Placeholder for brush.extent([extent])
    pub fn extent(&self) -> &Range<f64> {
        &self.extent
    }
    // TODO: Implement brushX and brushY with full API parity
    pub fn filter<F>(&mut self, filter: F) -> &mut Self
    where
        F: Fn() -> bool + 'static,
    {
        self._filter = Some(Box::new(filter));
        self
    }
    pub fn handle_size(&mut self, size: f64) -> &mut Self {
        self._handle_size = Some(size);
        self
    }
    pub fn on<F>(&mut self, typenames: &str, listener: F) -> &mut Self
    where
        F: Fn(BrushEvent) + 'static,
    {
        self._listeners.push((typenames.to_string(), Box::new(listener)));
        self
    }
    pub fn r#move(&mut self, range: Range<f64>) {
        self.select(range.clone());
        let event = BrushEvent {
            event_type: "move".to_string(),
            selection: self.selection.clone(),
        };
        for (typ, cb) in &self._listeners {
            if typ == "move" {
                cb(event.clone());
            }
        }
    }
    pub fn brush_selection(&self) -> Option<Range<f64>> {
        self.selection.clone()
    }
}

/// Placeholder for d3.brushX and d3.brushY
pub fn brush_x() -> Brush {
    Brush::new(0.0..1.0)
}

pub fn brush_y() -> Brush {
    Brush::new(0.0..1.0)
}
