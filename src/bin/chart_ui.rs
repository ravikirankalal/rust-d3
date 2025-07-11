use rust_d3::array::extent::extent;
use rust_d3::array::max::max;
use rust_d3::selection::{Arena, Selection};
use rust_d3::shape::Area;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rust_d3::scale::ScaleLinear;
use chrono::Utc;
use rust_d3::time::format::time_parse;
use rust_d3::axis::{axis_bottom, axis_left};
use chrono::{DateTime};
use eframe::egui;
use usvg::{Tree, Options};
use resvg::render;
use tiny_skia::Pixmap;
use eframe::egui::{ColorImage, TextureHandle};

fn generate_svg_chart() -> String {
    let file = File::open("examples/aapl.csv").expect("Cannot open aapl.csv");
    let reader = BufReader::new(file);
    let mut dates = Vec::new();
    let mut closes = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        if i == 0 { continue; }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 { continue; }
        let parsed = time_parse("%Y-%m-%d", parts[0].to_string().as_str());
        dates.push(DateTime::<Utc>::from_utc(parsed.unwrap(), Utc));
        closes.push(parts[1].parse::<f32>().unwrap_or(0.0));
    }
    let width = 600;
    let height = 300;
    let margin_right = 30;
    let margin_bottom = 30;
    let margin_left = 40;
    let n = closes.len();
    let min_close = closes.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_close = closes.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let mut arena = Arena { nodes: slotmap::SlotMap::with_key() };
    let mut svg = Selection::root(&mut arena, "svg");
    svg.attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("0 0 {} {}", width, height))
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("style", "max-width: 100%; height: auto;");
    let x = ScaleLinear::new([0.0, (n - 1) as f64], [margin_left as f64, (width - margin_right) as f64]);
    let y = ScaleLinear::new([min_close as f64, max_close as f64], [(height - margin_bottom) as f64, 20.0]);
    
    // Append x-axis
    let mut x_axis_group = svg.append("g");
    x_axis_group.attr("transform", &format!("translate(0,{})", height - margin_bottom));
    x_axis_group.call(axis_bottom(x.clone()).tick_count(width / 80).tick_size_inner(6.0).tick_padding(3.0));
    ;
    // rust_d3::axis::render_axis_bottom(&x_axis, &mut x_axis_group);
    
    // Append y-axis
    let mut y_axis_group = svg.append("g");
    y_axis_group.attr("transform", &format!("translate({},0)", margin_left));
    let y_axis = axis_left(y.clone()).tick_count(height / 40).tick_size_inner(6.0).tick_padding(3.0);
    rust_d3::axis::render_axis_left(&y_axis, &mut y_axis_group);
    let area = Area::new()
        .x(|_d: &f32, i: usize| x.scale(i as f64))
        .y0(|_d: &f32, _| y.scale(min_close as f64))
        .y1(|d: &f32, _| y.scale(*d as f64));
    svg.append("path")
      .attr("fill", "steelblue")
      .attr("d", &area.generate(&closes))
      .attr("stroke-width", "2");
    let root_key = svg.iter().next().copied().unwrap();
    Selection::render_node(&arena, root_key)
}

struct ChartApp {
    svg: String,
    texture: Option<TextureHandle>,
}

impl ChartApp {
    fn rasterize_svg(&self, ctx: &eframe::egui::Context) -> Option<TextureHandle> {
        let opt = Options::default();
        let tree = Tree::from_str(&self.svg, &opt).ok()?;
        let size = tree.size();
        let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32)?;
        render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
        let image = ColorImage::from_rgba_unmultiplied([
            size.width() as usize,
            size.height() as usize
        ], pixmap.data());
        Some(ctx.load_texture("chart_svg", image, eframe::egui::TextureOptions::default()))
    }
}

impl eframe::App for ChartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.texture.is_none() {
            self.texture = self.rasterize_svg(ctx);
        }
        egui::CentralPanel::default().frame(
            egui::Frame::default().fill(egui::Color32::WHITE)
        ).show(ctx, |ui| {
            ui.heading("Apple Stock Chart");
            if let Some(tex) = &self.texture {
                ui.image(tex);
            } else {
                ui.label("Failed to render SVG");
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let svg_str = generate_svg_chart();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Apple Stock Chart",
        options,
        Box::new(|cc| Ok(Box::new(ChartApp { svg: svg_str, texture: None }))),
    )
}
