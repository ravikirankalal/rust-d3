// src/chord/path.rs

use super::{Chord, Group, Subgroup};
use std::f64::consts::PI;

pub struct ArcGenerator {
    pub inner_radius: Option<Box<dyn Fn(&Group) -> f64>>,
    pub outer_radius: Option<Box<dyn Fn(&Group) -> f64>>,
}

impl ArcGenerator {
    pub fn new() -> Self {
        ArcGenerator {
            inner_radius: None,
            outer_radius: None,
        }
    }

    pub fn inner_radius(mut self, r: f64) -> Self {
        self.inner_radius = Some(Box::new(move |_| r));
        self
    }

    pub fn inner_radius_fn(mut self, f: Box<dyn Fn(&Group) -> f64>) -> Self {
        self.inner_radius = Some(f);
        self
    }

    pub fn outer_radius(mut self, r: f64) -> Self {
        self.outer_radius = Some(Box::new(move |_| r));
        self
    }

    pub fn outer_radius_fn(mut self, f: Box<dyn Fn(&Group) -> f64>) -> Self {
        self.outer_radius = Some(f);
        self
    }

    pub fn path(&self, group: &Group) -> String {
        let inner_radius = self.inner_radius.as_ref().map_or(0.0, |f| f(group));
        let outer_radius = self.outer_radius.as_ref().map_or(0.0, |f| f(group));

        let start_angle = group.start_angle - PI / 2.0;
        let end_angle = group.end_angle - PI / 2.0;

        let x1 = outer_radius * start_angle.cos();
        let y1 = outer_radius * start_angle.sin();

        let x2 = outer_radius * end_angle.cos();
        let y2 = outer_radius * end_angle.sin();

        let x3 = inner_radius * end_angle.cos();
        let y3 = inner_radius * end_angle.sin();

        let x4 = inner_radius * start_angle.cos();
        let y4 = inner_radius * start_angle.sin();

        let large_arc_flag = if (end_angle - start_angle).abs() > PI {
            1
        } else {
            0
        };

        format!(
            "M{:.6},{:.6}
            A{:.6},{:.6},0,{:?},1,{:.6},{:.6}
            L{:.6},{:.6}
            A{:.6},{:.6},0,{:?},0,{:.6},{:.6}Z",
            x1,
            y1,
            outer_radius,
            outer_radius,
            large_arc_flag,
            x2,
            y2,
            x3,
            y3,
            inner_radius,
            inner_radius,
            large_arc_flag,
            x4,
            y4
        )
    }
}

pub struct RibbonGenerator {
    pub radius: Option<Box<dyn Fn(&Chord) -> f64>>,
    pub start_angle: Option<Box<dyn Fn(&Subgroup) -> f64>>,
    pub end_angle: Option<Box<dyn Fn(&Subgroup) -> f64>>,
    pub source_radius: Option<Box<dyn Fn(&Subgroup) -> f64>>,
    pub target_radius: Option<Box<dyn Fn(&Subgroup) -> f64>>,
}

impl RibbonGenerator {
    pub fn new() -> Self {
        RibbonGenerator {
            radius: None,
            start_angle: None,
            end_angle: None,
            source_radius: None,
            target_radius: None,
        }
    }

    pub fn radius(mut self, r: f64) -> Self {
        self.radius = Some(Box::new(move |_| r));
        self
    }

    pub fn radius_fn(mut self, f: Box<dyn Fn(&Chord) -> f64>) -> Self {
        self.radius = Some(f);
        self
    }

    pub fn start_angle(mut self, f: Box<dyn Fn(&Subgroup) -> f64>) -> Self {
        self.start_angle = Some(f);
        self
    }

    pub fn end_angle(mut self, f: Box<dyn Fn(&Subgroup) -> f64>) -> Self {
        self.end_angle = Some(f);
        self
    }

    pub fn source_radius(mut self, f: Box<dyn Fn(&Subgroup) -> f64>) -> Self {
        self.source_radius = Some(f);
        self
    }

    pub fn target_radius(mut self, f: Box<dyn Fn(&Subgroup) -> f64>) -> Self {
        self.target_radius = Some(f);
        self
    }

    pub fn path(&self, chord: &Chord) -> String {
        let radius = self.radius.as_ref().map_or(0.0, |f| f(chord));

        let source_start_angle = self
            .start_angle
            .as_ref()
            .map_or(chord.source.start_angle, |f| f(&chord.source))
            - PI / 2.0;
        let source_end_angle = self
            .end_angle
            .as_ref()
            .map_or(chord.source.end_angle, |f| f(&chord.source))
            - PI / 2.0;
        let target_start_angle = self
            .start_angle
            .as_ref()
            .map_or(chord.target.start_angle, |f| f(&chord.target))
            - PI / 2.0;
        let target_end_angle = self
            .end_angle
            .as_ref()
            .map_or(chord.target.end_angle, |f| f(&chord.target))
            - PI / 2.0;

        let source_radius = self
            .source_radius
            .as_ref()
            .map_or(radius, |f| f(&chord.source));
        let target_radius = self
            .target_radius
            .as_ref()
            .map_or(radius, |f| f(&chord.target));

        let sx1 = source_radius * source_start_angle.cos();
        let sy1 = source_radius * source_start_angle.sin();

        let sx2 = source_radius * source_end_angle.cos();
        let sy2 = source_radius * source_end_angle.sin();

        let tx1 = target_radius * target_start_angle.cos();
        let ty1 = target_radius * target_start_angle.sin();

        let tx2 = target_radius * target_end_angle.cos();
        let ty2 = target_radius * target_end_angle.sin();

        let source_large_arc_flag = if (source_end_angle - source_start_angle).abs() > PI {
            1
        } else {
            0
        };
        let target_large_arc_flag = if (target_end_angle - target_start_angle).abs() > PI {
            1
        } else {
            0
        };

        format!(
            "M{:.6},{:.6}
            A{:.6},{:.6},0,{:?},1,{:.6},{:.6}
            Q0,0,{:.6},{:.6}
            A{:.6},{:.6},0,{:?},1,{:.6},{:.6}
            Q0,0,{:.6},{:.6}Z",
            sx1,
            sy1,
            source_radius,
            source_radius,
            source_large_arc_flag,
            sx2,
            sy2,
            tx1,
            ty1,
            target_radius,
            target_radius,
            target_large_arc_flag,
            tx2,
            ty2,
            sx1,
            sy1
        )
    }
}
