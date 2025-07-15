use chrono::{TimeZone, Utc};
use eframe::egui;
use eframe::egui::{ColorImage, TextureHandle};
use resvg::render;
use rust_d3::axis::AxisRenderable;
use rust_d3::axis::{axis_bottom, axis_left};
use rust_d3::scale::ScaleLinear;
use rust_d3::scale::ScaleTime;
use rust_d3::selection::{Arena, Selection};
use rust_d3::shape::Area;
use rust_d3::time::format::time_parse;
use slotmap::SlotMap;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use tiny_skia::Pixmap;
use usvg::fontdb;
use usvg::{Options, Tree};

fn generate_svg_chart() -> String {
    let file = File::open("examples/aapl.csv").expect("Cannot open aapl.csv");
    let reader = BufReader::new(file);
    let mut dates = Vec::new();
    let mut closes = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        if i == 0 {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue;
        }
        let parsed = time_parse("%Y-%m-%d", parts[0].to_string().as_str());
        dates.push(Utc.from_utc_datetime(&parsed.unwrap()));
        closes.push(parts[1].parse::<f32>().unwrap_or(0.0));
    }
    let width: usize = 928;
    let height: usize = 500;
    let _margin_top: i32 = 20;
    let margin_right: i32 = 30;
    let margin_bottom: i32 = 30;
    let margin_left: i32 = 40;
    
    // Add padding to prevent text clipping
    let _n = closes.len();
    let min_close = 0;
    let max_close = closes.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let arena = Rc::new(RefCell::new(Arena {
        nodes: SlotMap::with_key(),
    }));
    let mut svg = Selection::root(Rc::clone(&arena), "svg");
    svg.attr("width", &width.to_string())
        .attr("height", &height.to_string())
        .attr("viewBox", &format!("{} {} {} {}", 0, 0, width, height))
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("style", "max-width: 100%; height: auto;");
    let x = ScaleTime::new(
        [
            dates.first().unwrap().naive_utc(),
            dates.last().unwrap().naive_utc(),
        ],
        [(margin_left) as f64, (width as i32 - margin_right) as f64],
    );
    let y = ScaleLinear::new(
        [min_close as f64, max_close as f64],
        [(height as i32 - margin_bottom) as f64, 20.0],
    );

    // Area generator
    // let area = Area::new()
    //     .x(|_d: &f32, i: usize| x.scale(dates[i].naive_utc()))
    //     .y0(|_d: &f32, _| y.scale(min_close as f64))
    //     .y1(|d: &f32, _i| y.scale(*d as f64));
    // svg.append("path")
    //     .attr("fill", "steelblue")
    //     .attr("d", &area.generate(&closes));


    let root_key = svg.iter().next().copied().unwrap();

    // // Append x-axis
    let mut x_axis_group = svg.append("g");
    let x_transform = format!("translate(0,{})", height as i32 - margin_bottom);
    println!("[DEBUG] Setting x-axis transform: {}", x_transform);
    x_axis_group.attr(
        "transform",
        &x_transform,
    );
    x_axis_group.call(|sel| {
        axis_bottom(x.clone())
            .with_ticks((width / 80) as usize)
            .tick_size(5.0)
            .render(sel);
    });

    // Append y-axis
    let y_transform = format!("translate({},{})", margin_left, 0);
    println!("[DEBUG] Setting y-axis transform: {}", y_transform);
    svg.append("g")
    .attr("transform", &y_transform)
    .call(|sel| {
        axis_left(y.clone())
            .with_ticks(height /40 )
            .tick_size(5.0)
            .render(sel);
    })
    //remove the domain line
    // .call(|sel| {
    //     sel.select_by(".domain").remove();
    // })
    // Adjust tick lines for x-axis
    .call(|g| {
        g.select_by(".tick").clone()
            .attr(
                "x2",
                (width as i32 - margin_left - margin_right)
                    .to_string()
                    .as_str(),
            )
            .attr("stroke-opacity", "0.1");
        // .attr("stroke", "#888");
    });

    Selection::render_node(&arena, root_key)
}

struct ChartApp {
    svg: String,
    texture: Option<TextureHandle>,
}

impl ChartApp {
    fn rasterize_svg(&self, ctx: &eframe::egui::Context) -> Option<TextureHandle> {
        let mut opt = Options::default();
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts(); // Load system fonts for SVG text rendering
        opt.fontdb = std::sync::Arc::new(fontdb); // Fix: wrap fontdb in Arc
        let tree = Tree::from_str(&self.svg, &opt).ok()?;
        let size = tree.size();
        let mut pixmap = Pixmap::new(size.width() as u32, size.height() as u32)?;
        render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
        let image = ColorImage::from_rgba_unmultiplied(
            [size.width() as usize, size.height() as usize],
            pixmap.data(),
        );
        Some(ctx.load_texture("chart_svg", image, eframe::egui::TextureOptions::default()))
    }
}

impl eframe::App for ChartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Always reload the texture for dynamic updates
        self.texture = self.rasterize_svg(ctx);
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::WHITE))
            .show(ctx, |ui| {
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
    println!("\n--- SVG OUTPUT ---\n{}\n--- END SVG ---\n", svg_str);
    let options = eframe::NativeOptions::default(); // No window size field available in this version
    // To set window size, use ctx.request_repaint() or egui window API after startup if needed
    eframe::run_native(
        "Apple Stock Chart",
        options,
        Box::new(|_cc| {
            Ok(Box::new(ChartApp {
                svg: svg_str,
                texture: None,
            }))
        }),
    )
    
}
