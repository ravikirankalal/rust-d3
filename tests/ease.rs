mod ease {
    pub fn linear(t: f64) -> f64 {
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
        1.0 + t1 * t1 * t1
    }

    pub fn cubic_in_out(t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let t1 = 2.0 * t - 2.0;
            0.5 * t1 * t1 * t1 + 1.0
        }
    }
}

#[test]
fn test_ease_linear() {
    assert_eq!(ease::linear(0.5), 0.5);
}

#[test]
fn test_ease_quad_in() {
    assert_eq!(ease::quad_in(0.5), 0.25);
}

#[test]
fn test_ease_quad_out() {
    assert_eq!(ease::quad_out(0.5), 0.75);
}

#[test]
fn test_ease_quad_in_out() {
    assert_eq!(ease::quad_in_out(0.5), 0.5);
}

#[test]
fn test_ease_cubic_in() {
    assert_eq!(ease::cubic_in(0.5), 0.125);
}

#[test]
fn test_ease_cubic_out() {
    assert_eq!(ease::cubic_out(0.5), 0.875);
}

#[test]
fn test_ease_cubic_in_out() {
    assert_eq!(ease::cubic_in_out(0.5), 0.5);
}
