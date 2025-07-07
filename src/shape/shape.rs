// LineGenerator implementation

pub struct LineGenerator;

impl LineGenerator {
    pub fn generate<T, F>(data: &[T], mut accessor: F) -> Vec<(f64, f64)>
    where
        F: FnMut(&T) -> (f64, f64),
    {
        data.iter().map(|d| accessor(d)).collect()
    }

    /// Placeholder for d3-shape API parity.
    /// See: https://github.com/d3/d3-shape#api-reference
    /// TODO: Implement full API parity with d3-shape (area, arc, pie, line, radialArea, radialLine, symbol, etc.)

    // D3.js area generator: returns a vector of points outlining the area under the curve
    pub fn area<T, F>(data: &[T], mut accessor: F) -> Vec<(f64, f64)>
    where
        F: FnMut(&T) -> (f64, f64),
    {
        // For a simple area, return the points in order, then the baseline in reverse
        let mut points: Vec<(f64, f64)> = data.iter().map(|d| accessor(d)).collect();
        if points.is_empty() { return points; }
        // Close the area by adding baseline (x, 0) in reverse order
        let baseline: Vec<(f64, f64)> = data.iter().rev().map(|d| {
            let (x, _) = accessor(d);
            (x, 0.0)
        }).collect();
        points.extend(baseline);
        points
    }
    // D3.js arc generator: returns an SVG path string for an arc/sector
    pub fn arc(inner_radius: f64, outer_radius: f64, start_angle: f64, end_angle: f64) -> String {
        // SVG arc path for a donut/pie sector
        if (end_angle - start_angle).abs() < 1e-10 {
            return String::new();
        }
        let (x0, y0) = (inner_radius * start_angle.cos(), inner_radius * start_angle.sin());
        let (x1, y1) = (outer_radius * start_angle.cos(), outer_radius * start_angle.sin());
        let (x2, y2) = (outer_radius * end_angle.cos(), outer_radius * end_angle.sin());
        let (x3, y3) = (inner_radius * end_angle.cos(), inner_radius * end_angle.sin());
        let large_arc = if (end_angle - start_angle).abs() > std::f64::consts::PI { 1 } else { 0 };
        format!(
            "M{x0},{y0} L{x1},{y1} A{outer_radius},{outer_radius} 0 {large_arc},1 {x2},{y2} L{x3},{y3} A{inner_radius},{inner_radius} 0 {large_arc},0 {x0},{y0} Z",
            x0 = x0, y0 = y0, x1 = x1, y1 = y1, x2 = x2, y2 = y2, x3 = x3, y3 = y3,
            outer_radius = outer_radius, inner_radius = inner_radius, large_arc = large_arc
        )
    }
    // D3.js pie generator: returns (start_angle, end_angle) for each slice
    pub fn pie<T: Into<f64> + Copy>(data: &[T]) -> Vec<(f64, f64)> {
        let sum: f64 = data.iter().map(|v| (*v).into()).sum();
        if sum == 0.0 || data.is_empty() {
            return vec![];
        }
        let mut angles = Vec::with_capacity(data.len());
        let mut current = 0.0;
        for v in data {
            let value = (*v).into();
            let start = current;
            let end = start + value / sum * std::f64::consts::TAU;
            angles.push((start, end));
            current = end;
        }
        angles
    }
    // D3.js symbol generator: returns SVG path for basic shapes
    pub fn symbol(symbol_type: &str, size: f64) -> String {
        let r = (size / std::f64::consts::PI).sqrt();
        match symbol_type {
            "circle" => format!("M{},0A{},{} 0 1,0 {},0A{},{} 0 1,0 {},0Z", r, r, r, -r, r, r, r),
            "square" => format!("M{},{}h{}v{}h{}Z", -r, -r, 2.0*r, 2.0*r, -2.0*r),
            "triangle" => {
                let h = (3.0_f64).sqrt() * r;
                format!("M0,{}L{},{}L{},{}Z", -h/2.0, r, h/2.0, -r, h/2.0)
            },
            _ => String::new(),
        }
    }
    // D3.js radial area generator: like area, but polar coordinates
    pub fn radial_area<T, F>(data: &[T], mut accessor: F) -> Vec<(f64, f64)>
    where
        F: FnMut(&T) -> (f64, f64),
    {
        let mut points: Vec<(f64, f64)> = data.iter().map(|d| accessor(d)).collect();
        if points.is_empty() { return points; }
        // Close the area by adding baseline (r, 0) in reverse order
        let baseline: Vec<(f64, f64)> = data.iter().rev().map(|d| {
            let (r, _) = accessor(d);
            (r, 0.0)
        }).collect();
        points.extend(baseline);
        points
    }
    // D3.js radial line generator: like line, but polar coordinates
    pub fn radial_line<T, F>(data: &[T], mut accessor: F) -> Vec<(f64, f64)>
    where
        F: FnMut(&T) -> (f64, f64),
    {
        data.iter().map(|d| accessor(d)).collect()
    }
}

// Moved struct definitions to module scope to resolve errors.
#[allow(dead_code)]
pub struct ArcGenerator {
    inner_radius: f64,
    outer_radius: f64,
    start_angle: f64,
    end_angle: f64,
}

impl ArcGenerator {
    pub fn new() -> Self {
        Self {
            inner_radius: 0.0,
            outer_radius: 1.0,
            start_angle: 0.0,
            end_angle: std::f64::consts::TAU,
        }
    }
    pub fn inner_radius(mut self, r: f64) -> Self {
        self.inner_radius = r;
        self
    }
    pub fn outer_radius(mut self, r: f64) -> Self {
        self.outer_radius = r;
        self
    }
    pub fn start_angle(mut self, a: f64) -> Self {
        self.start_angle = a;
        self
    }
    pub fn end_angle(mut self, a: f64) -> Self {
        self.end_angle = a;
        self
    }
    pub fn generate(&self) -> String {
        if (self.end_angle - self.start_angle).abs() < 1e-10 {
            return String::new();
        }
        let (x0, y0) = (self.inner_radius * self.start_angle.cos(), self.inner_radius * self.start_angle.sin());
        let (x1, y1) = (self.outer_radius * self.start_angle.cos(), self.outer_radius * self.start_angle.sin());
        let (x2, y2) = (self.outer_radius * self.end_angle.cos(), self.outer_radius * self.end_angle.sin());
        let (x3, y3) = (self.inner_radius * self.end_angle.cos(), self.inner_radius * self.end_angle.sin());
        let large_arc = if (self.end_angle - self.start_angle).abs() > std::f64::consts::PI { 1 } else { 0 };
        format!(
            "M{x0},{y0} L{x1},{y1} A{outer},{outer} 0 {large_arc},1 {x2},{y2} L{x3},{y3} A{inner},{inner} 0 {large_arc},0 {x0},{y0} Z",
            x0 = x0, y0 = y0, x1 = x1, y1 = y1, x2 = x2, y2 = y2, x3 = x3, y3 = y3,
            outer = self.outer_radius, inner = self.inner_radius, large_arc = large_arc
        )
    }
}
#[allow(dead_code)]
pub struct PieGenerator<T, F> {
    value_fn: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> PieGenerator<T, F>
where
    F: Fn(&T) -> f64,
{
    pub fn new(value_fn: F) -> Self {
        Self { value_fn, _phantom: std::marker::PhantomData }
    }
    pub fn generate(&self, data: &[T]) -> Vec<(f64, f64)> {
        let sum: f64 = data.iter().map(&self.value_fn).sum();
        if sum == 0.0 || data.is_empty() {
            return vec![];
        }
        let mut angles = Vec::with_capacity(data.len());
        let mut current = 0.0;
        for d in data {
            let value = (self.value_fn)(d);
            let start = current;
            let end = start + value / sum * std::f64::consts::TAU;
            angles.push((start, end));
            current = end;
        }
        angles
    }
}
#[allow(dead_code)]
pub struct SymbolGenerator {
    symbol_type: String,
    size: f64,
}

impl SymbolGenerator {
    pub fn new() -> Self {
        Self {
            symbol_type: "circle".to_string(),
            size: 64.0,
        }
    }
    pub fn symbol_type<S: Into<String>>(mut self, symbol_type: S) -> Self {
        self.symbol_type = symbol_type.into();
        self
    }
    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }
    pub fn generate(&self) -> String {
        let r = (self.size / std::f64::consts::PI).sqrt();
        match self.symbol_type.as_str() {
            "circle" => format!("M{},0A{},{} 0 1,0 {},0A{},{} 0 1,0 {},0Z", r, r, r, -r, r, r, r),
            "square" => format!("M{},{}h{}v{}h{}Z", -r, -r, 2.0*r, 2.0*r, -2.0*r),
            "triangle" => {
                let h = (3.0_f64).sqrt() * r;
                format!("M0,{}L{},{}L{},{}Z", -h/2.0, r, h/2.0, -r, h/2.0)
            },
            _ => String::new(),
        }
    }
}

pub struct AreaGenerator<T, F> {
    y_fn: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> AreaGenerator<T, F>
where
    F: Fn(&T) -> (f64, f64),
{
    pub fn new(y_fn: F) -> Self {
        Self { y_fn, _phantom: std::marker::PhantomData }
    }
    pub fn generate(&self, data: &[T]) -> Vec<(f64, f64)> {
        let mut points: Vec<(f64, f64)> = data.iter().map(|d| (self.y_fn)(d)).collect();
        if points.is_empty() { return points; }
        let baseline: Vec<(f64, f64)> = data.iter().rev().map(|d| {
            let (x, _) = (self.y_fn)(d);
            (x, 0.0)
        }).collect();
        points.extend(baseline);
        points
    }
}

pub struct RadialAreaGenerator<T, F> {
    accessor: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> RadialAreaGenerator<T, F>
where
    F: Fn(&T) -> (f64, f64),
{
    pub fn new(accessor: F) -> Self {
        Self { accessor, _phantom: std::marker::PhantomData }
    }
    pub fn generate(&self, data: &[T]) -> Vec<(f64, f64)> {
        let mut points: Vec<(f64, f64)> = data.iter().map(|d| (self.accessor)(d)).collect();
        if points.is_empty() { return points; }
        let baseline: Vec<(f64, f64)> = data.iter().rev().map(|d| {
            let (r, _) = (self.accessor)(d);
            (r, 0.0)
        }).collect();
        points.extend(baseline);
        points
    }
}
pub struct RadialLineGenerator<T, F> {
    accessor: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> RadialLineGenerator<T, F>
where
    F: Fn(&T) -> (f64, f64),
{
    pub fn new(accessor: F) -> Self {
        Self { accessor, _phantom: std::marker::PhantomData }
    }
    pub fn generate(&self, data: &[T]) -> Vec<(f64, f64)> {
        data.iter().map(|d| (self.accessor)(d)).collect()
    }
}
