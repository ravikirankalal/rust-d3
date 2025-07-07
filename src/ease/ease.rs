'''pub fn linear(t: f64) -> f64 {
    t
}

pub fn quad_in(t: f64) -> f64 {
    t * t
}

pub fn quad_out(t: f64) -> f64 {
    t * (2.0 - t)
}

pub fn quad_in_out(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        -1.0 + (4.0 - 2.0 * t) * t
    }
}

pub fn cubic_in(t: f64) -> f64 {
    t * t * t
}

pub fn cubic_out(t: f64) -> f64 {
    let t1 = t - 1.0;
    t1 * t1 * t1 + 1.0
}

pub fn cubic_in_out(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let t1 = 2.0 * t - 2.0;
        0.5 * t1 * t1 * t1 + 1.0
    }
}

pub fn poly_in(t: f64, e: f64) -> f64 {
    t.powf(e)
}

pub fn poly_out(t: f64, e: f64) -> f64 {
    1.0 - (1.0 - t).powf(e)
}

pub fn poly_in_out(t: f64, e: f64) -> f64 {
    if t < 0.5 {
        0.5 * (2.0 * t).powf(e)
    } else {
        1.0 - 0.5 * (2.0 - 2.0 * t).powf(e)
    }
}

pub fn sin_in(t: f64) -> f64 {
    1.0 - (t * std::f64::consts::PI / 2.0).cos()
}

pub fn sin_out(t: f64) -> f64 {
    (t * std::f64::consts::PI / 2.0).sin()
}

pub fn sin_in_out(t: f64) -> f64 {
    (1.0 - (t * std::f64::consts::PI).cos()) / 2.0
}

pub fn exp_in(t: f64) -> f64 {
    if t == 0.0 { 0.0 } else { (10.0 * t - 10.0).exp2() }
}

pub fn exp_out(t: f64) -> f64 {
    if t == 1.0 { 1.0 } else { 1.0 - (-10.0 * t).exp2() }
}

pub fn exp_in_out(t: f64) -> f64 {
    if t == 0.0 { 0.0 }
    else if t == 1.0 { 1.0 }
    else if t < 0.5 { 0.5 * (20.0 * t - 10.0).exp2() }
    else { (2.0 - (20.0 * t - 10.0).exp2()) / 2.0 }
}

pub fn circle_in(t: f64) -> f64 {
    1.0 - (1.0 - t * t).sqrt()
}

pub fn circle_out(t: f64) -> f64 {
    let t1 = t - 1.0;
    (1.0 - t1 * t1).sqrt()
}

pub fn circle_in_out(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
    } else {
        let t1 = 2.0 * t - 2.0;
        ((1.0 - t1 * t1).sqrt() + 1.0) / 2.0
    }
}

pub fn bounce_out(t: f64) -> f64 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t2 = t - 1.5 / d1;
        n1 * t2 * t2 + 0.75
    } else if t < 2.5 / d1 {
        let t2 = t - 2.25 / d1;
        n1 * t2 * t2 + 0.9375
    } else {
        let t2 = t - 2.625 / d1;
        n1 * t2 * t2 + 0.984375
    }
}

pub fn bounce_in(t: f64) -> f64 {
    1.0 - bounce_out(1.0 - t)
}

pub fn bounce_in_out(t: f64) -> f64 {
    if t < 0.5 {
        (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
    }
}

pub fn back_in(t: f64, s: f64) -> f64 {
    s * t * t * t - (s - 1.0) * t * t
}

pub fn back_out(t: f64, s: f64) -> f64 {
    let t1 = t - 1.0;
    1.0 + s * t1 * t1 * t1 + (s - 1.0) * t1 * t1
}

pub fn back_in_out(t: f64, s: f64) -> f64 {
    let s1 = s * 1.525;
    if t < 0.5 {
        0.5 * (2.0 * t) * (2.0 * t) * ((s1 + 1.0) * 2.0 * t - s1)
    } else {
        let t1 = 2.0 * t - 2.0;
        0.5 * (t1 * t1 * ((s1 + 1.0) * t1 + s1) + 2.0)
    }
}

pub fn elastic_in(t: f64, a: f64, p: f64) -> f64 {
    let s = (2.0 * std::f64::consts::PI / p) * t.asin();
    -a * (10.0 * t - 10.0).exp2() * ((t - s) * (2.0 * std::f64::consts::PI) / p).sin()
}

pub fn elastic_out(t: f64, a: f64, p: f64) -> f64 {
    let s = (2.0 * std::f64::consts::PI / p) * 1.0_f64.asin();
    a * (-10.0 * t).exp2() * ((t - s) * (2.0 * std::f64::consts::PI) / p).sin() + 1.0
}

pub fn elastic_in_out(t: f64, a: f64, p: f64) -> f64 {
    let s = (2.0 * std::f64::consts::PI / p) * 1.0_f64.asin();
    if t < 0.5 {
        -0.5 * (a * (20.0 * t - 10.0).exp2() * ((2.0 * t - 0.5 - s) * (2.0 * std::f64::consts::PI) / p).sin())
    } else {
        a * (-20.0 * t + 10.0).exp2() * ((2.0 * t - 0.5 - s) * (2.0 * std::f64::consts::PI) / p).sin() * 0.5 + 1.0
    }
}
''