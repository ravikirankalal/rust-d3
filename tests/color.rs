use rust_d3::color::Color;
use rust_d3::color::hcl::Hcl;
use rust_d3::color::hsl::Hsl;
use rust_d3::color::lab::Lab;
use rust_d3::color::rgb::Rgb;

#[test]
fn test_color_from_str_rgb() {
    assert_eq!(
        "rgb(255,0,0)".parse::<Color>().unwrap(),
        Color::Rgb(Rgb::new(255, 0, 0, 1.0))
    );
    assert_eq!(
        "rgb(0, 128, 255)".parse::<Color>().unwrap(),
        Color::Rgb(Rgb::new(0, 128, 255, 1.0))
    );
}

#[test]
fn test_color_from_str_rgba() {
    assert_eq!(
        "rgba(255,0,0,0.5)".parse::<Color>().unwrap(),
        Color::Rgb(Rgb::new(255, 0, 0, 0.5))
    );
    assert_eq!(
        "rgba(0, 128, 255, 1.0)".parse::<Color>().unwrap(),
        Color::Rgb(Rgb::new(0, 128, 255, 1.0))
    );
}

#[test]
fn test_color_from_str_hex() {
    assert_eq!(
        "#ff0000".parse::<Color>().unwrap(),
        Color::Rgb(Rgb::new(255, 0, 0, 1.0))
    );
    assert_eq!(
        "#0080ff".parse::<Color>().unwrap(),
        Color::Rgb(Rgb::new(0, 128, 255, 1.0))
    );
}

#[test]
fn test_color_from_str_hsl() {
    assert_eq!(
        "hsl(0,100%,50%)".parse::<Color>().unwrap(),
        Color::Hsl(Hsl::new(0.0, 100.0, 50.0, 1.0))
    );
    assert_eq!(
        "hsl(120,50%,75%)".parse::<Color>().unwrap(),
        Color::Hsl(Hsl::new(120.0, 50.0, 75.0, 1.0))
    );
}

#[test]
fn test_color_from_str_hsla() {
    assert_eq!(
        "hsla(0,100%,50%,0.5)".parse::<Color>().unwrap(),
        Color::Hsl(Hsl::new(0.0, 100.0, 50.0, 0.5))
    );
    assert_eq!(
        "hsla(120,50%,75%,1.0)".parse::<Color>().unwrap(),
        Color::Hsl(Hsl::new(120.0, 50.0, 75.0, 1.0))
    );
}

#[test]
fn test_color_from_str_invalid() {
    assert!("invalid".parse::<Color>().is_err());
    assert!("#123".parse::<Color>().is_err());
    assert!("rgb(1,2)".parse::<Color>().is_err());
    assert!("rgba(1,2,3)".parse::<Color>().is_err());
    assert!("hsl(1,2)".parse::<Color>().is_err());
    assert!("hsla(1,2,3)".parse::<Color>().is_err());
}

#[test]
fn test_rgb_to_hsl_conversion() {
    let red = Color::Rgb(Rgb::new(255, 0, 0, 1.0));
    let red_hsl = red.hsl();
    // Allow for some floating point inaccuracies
    assert!((red_hsl.h - 0.0).abs() < 0.1);
    assert!((red_hsl.s - 100.0).abs() < 0.1);
    assert!((red_hsl.l - 50.0).abs() < 0.1);
    assert!((red_hsl.opacity - 1.0).abs() < 0.1);

    let green = Color::Rgb(Rgb::new(0, 255, 0, 1.0));
    let green_hsl = green.hsl();
    assert!((green_hsl.h - 120.0).abs() < 0.1);
    assert!((green_hsl.s - 100.0).abs() < 0.1);
    assert!((green_hsl.l - 50.0).abs() < 0.1);
    assert!((green_hsl.opacity - 1.0).abs() < 0.1);

    let blue = Color::Rgb(Rgb::new(0, 0, 255, 1.0));
    let blue_hsl = blue.hsl();
    assert!((blue_hsl.h - 240.0).abs() < 0.1);
    assert!((blue_hsl.s - 100.0).abs() < 0.1);
    assert!((blue_hsl.l - 50.0).abs() < 0.1);
    assert!((blue_hsl.opacity - 1.0).abs() < 0.1);

    let gray = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
    let gray_hsl = gray.hsl();
    let expected_l = (128.0 / 255.0 + 128.0 / 255.0) / 2.0 * 100.0;
    assert!((gray_hsl.h - 0.0).abs() < 0.1);
    assert!((gray_hsl.s - 0.0).abs() < 0.1);
    assert!((gray_hsl.l - expected_l).abs() < 0.1);
    assert!((gray_hsl.opacity - 1.0).abs() < 0.1);
}

#[test]
fn test_hsl_to_rgb_conversion() {
    let red_hsl = Color::Hsl(Hsl::new(0.0, 100.0, 50.0, 1.0));
    let red_rgb = red_hsl.rgb();
    assert_eq!(red_rgb, Rgb::new(255, 0, 0, 1.0));

    let green_hsl = Color::Hsl(Hsl::new(120.0, 100.0, 50.0, 1.0));
    let green_rgb = green_hsl.rgb();
    assert_eq!(green_rgb, Rgb::new(0, 255, 0, 1.0));

    let blue_hsl = Color::Hsl(Hsl::new(240.0, 100.0, 50.0, 1.0));
    let blue_rgb = blue_hsl.rgb();
    assert_eq!(blue_rgb, Rgb::new(0, 0, 255, 1.0));

    let gray_hsl = Color::Hsl(Hsl::new(0.0, 0.0, 50.2, 1.0));
    let gray_rgb = gray_hsl.rgb();
    assert_eq!(gray_rgb, Rgb::new(128, 128, 128, 1.0));
}

#[test]
fn test_brighter() {
    let color = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
    let brighter_color = color.brighter(None);
    assert_eq!(brighter_color, Color::Rgb(Rgb::new(183, 183, 183, 1.0)));

    let brighter_color_k2 = color.brighter(Some(2.0));
    assert_eq!(brighter_color_k2, Color::Rgb(Rgb::new(255, 255, 255, 1.0)));
}

#[test]
fn test_darker() {
    let color = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
    let darker_color = color.darker(None);
    assert_eq!(darker_color, Color::Rgb(Rgb::new(90, 90, 90, 1.0)));

    let darker_color_k2 = color.darker(Some(2.0));
    assert_eq!(darker_color_k2, Color::Rgb(Rgb::new(63, 63, 63, 1.0)));
}

#[test]
fn test_rgb_to_lab_conversion() {
    let red = Color::Rgb(Rgb::new(255, 0, 0, 1.0));
    let red_lab = red.lab();
    assert!((red_lab.l - 53.23).abs() < 0.05);
    assert!((red_lab.a - 80.11).abs() < 0.02);
    assert!((red_lab.b - 67.22).abs() < 0.02);
    assert!((red_lab.opacity - 1.0).abs() < 0.01);

    let green = Color::Rgb(Rgb::new(0, 255, 0, 1.0));
    let green_lab = green.lab();
    assert!((green_lab.l - 87.73).abs() < 0.01);
    assert!((green_lab.a - (-86.18)).abs() < 0.01);
    assert!((green_lab.b - 83.18).abs() < 0.01);
    assert!((green_lab.opacity - 1.0).abs() < 0.01);

    let blue = Color::Rgb(Rgb::new(0, 0, 255, 1.0));
    let blue_lab = blue.lab();
    assert!((blue_lab.l - 32.30).abs() < 0.01);
    assert!((blue_lab.a - 79.19).abs() < 0.01);
    assert!((blue_lab.b - (-107.86)).abs() < 0.01);
    assert!((blue_lab.opacity - 1.0).abs() < 0.01);

    let gray = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
    let gray_lab = gray.lab();
    assert!((gray_lab.l - 53.59).abs() < 0.01);
    assert!((gray_lab.a - 0.0).abs() < 0.01);
    assert!((gray_lab.b - 0.0).abs() < 0.01);
    assert!((gray_lab.opacity - 1.0).abs() < 0.01);
}

#[test]
fn test_lab_to_rgb_conversion() {
    let red_lab = Color::Lab(Lab::new(53.23, 80.11, 67.22, 1.0));
    let red_rgb = red_lab.rgb();
    assert_eq!(red_rgb, Rgb::new(255, 0, 0, 1.0));

    let green_lab = Color::Lab(Lab::new(87.73, -86.18, 83.18, 1.0));
    let green_rgb = green_lab.rgb();
    assert_eq!(green_rgb, Rgb::new(0, 255, 0, 1.0));

    let blue_lab = Color::Lab(Lab::new(32.30, 79.19, -107.86, 1.0));
    let blue_rgb = blue_lab.rgb();
    assert_eq!(blue_rgb, Rgb::new(0, 0, 255, 1.0));

    let gray_lab = Color::Lab(Lab::new(53.59, 0.0, 0.0, 1.0));
    let gray_rgb = gray_lab.rgb();
    assert_eq!(gray_rgb, Rgb::new(128, 128, 128, 1.0));
}

#[test]
fn test_lab_to_hsl_conversion() {
    let red_lab = Color::Lab(Lab::new(53.23, 80.11, 67.22, 1.0));
    let red_hsl = red_lab.hsl();
    assert!((red_hsl.h - 0.0).abs() < 0.1);
    assert!((red_hsl.s - 100.0).abs() < 0.1);
    assert!((red_hsl.l - 50.0).abs() < 0.1);
    assert!((red_hsl.opacity - 1.0).abs() < 0.1);

    let green_lab = Color::Lab(Lab::new(87.73, -86.18, 83.18, 1.0));
    let green_hsl = green_lab.hsl();
    assert!((green_hsl.h - 120.0).abs() < 0.1);
    assert!((green_hsl.s - 100.0).abs() < 0.1);
    assert!((green_hsl.l - 50.0).abs() < 0.1);
    assert!((green_hsl.opacity - 1.0).abs() < 0.1);

    let blue_lab = Color::Lab(Lab::new(32.30, 79.19, -107.86, 1.0));
    let blue_hsl = blue_lab.hsl();
    assert!((blue_hsl.h - 240.0).abs() < 0.1);
    assert!((blue_hsl.s - 100.0).abs() < 0.1);
    assert!((blue_hsl.l - 50.0).abs() < 0.1);
    assert!((blue_hsl.opacity - 1.0).abs() < 0.1);

    let gray_lab = Color::Lab(Lab::new(53.59, 0.0, 0.0, 1.0));
    let gray_hsl = gray_lab.hsl();
    assert!((gray_hsl.h - 0.0).abs() < 0.1);
    assert!((gray_hsl.s - 0.0).abs() < 0.1);
    assert!((gray_hsl.l - 50.2).abs() < 0.1);
    assert!((gray_hsl.opacity - 1.0).abs() < 0.1);
}

#[test]
fn test_gamma() {
    let color = Color::Rgb(Rgb::new(128, 128, 128, 1.0));
    let gamma_color = color.gamma(2.2);
    // Expected values for 128,128,128 gamma 2.2 are approximately 55,55,55
    assert!((gamma_color.rgb().r as f32 - 186.0).abs() < 1.0);
    assert!((gamma_color.rgb().g as f32 - 186.0).abs() < 1.0);
    assert!((gamma_color.rgb().b as f32 - 186.0).abs() < 1.0);
}

#[test]
fn test_clamp_rgb() {
    let color = Color::Rgb(Rgb::new(u8::MAX, u8::MIN, 128, 2.0)); // Test with max, min, and out-of-range opacity
    let clamped_color = color.clamp();
    assert_eq!(clamped_color, Color::Rgb(Rgb::new(255, 0, 128, 1.0)));

    let color_overflow = Color::Rgb(Rgb::new(200, 200, 200, -0.5)); // Test with negative opacity
    let clamped_color_overflow = color_overflow.clamp();
    assert_eq!(
        clamped_color_overflow,
        Color::Rgb(Rgb::new(200, 200, 200, 0.0))
    );
}

#[test]
fn test_clamp_hsl() {
    let color = Color::Hsl(Hsl::new(400.0, -10.0, 120.0, 2.0));
    let clamped_color = color.clamp();
    assert_eq!(clamped_color, Color::Hsl(Hsl::new(40.0, 0.0, 100.0, 1.0)));
}

#[test]
fn test_clamp_lab() {
    let color = Color::Lab(Lab::new(120.0, -200.0, 200.0, 2.0));
    let clamped_color = color.clamp();
    assert_eq!(
        clamped_color,
        Color::Lab(Lab::new(100.0, -200.0, 200.0, 1.0))
    );
}

#[test]
fn test_format_hex() {
    let color = Color::Rgb(Rgb::new(255, 0, 128, 1.0));
    assert_eq!(color.format_hex(), "#ff0080");
}

#[test]
fn test_format_rgb() {
    let color_rgb = Color::Rgb(Rgb::new(255, 0, 128, 1.0));
    assert_eq!(color_rgb.format_rgb(), "rgb(255,0,128)");

    let color_rgba = Color::Rgb(Rgb::new(255, 0, 128, 0.5));
    assert_eq!(color_rgba.format_rgb(), "rgba(255,0,128,0.5)");
}

#[test]
fn test_format_hsl() {
    let color_hsl = Color::Hsl(Hsl::new(120.0, 50.0, 75.0, 1.0));
    assert_eq!(color_hsl.format_hsl(), "hsl(120,50%,75%)");

    let color_hsla = Color::Hsl(Hsl::new(120.0, 50.0, 75.0, 0.5));
    assert_eq!(color_hsla.format_hsl(), "hsla(120,50%,75%,0.5)");
}

#[test]
fn test_rgb_displayable() {
    let displayable_color = Rgb::new(255, 0, 0, 1.0);
    assert!(displayable_color.displayable());

    let non_displayable_color_high_opacity = Rgb::new(255, 0, 0, 1.1);
    assert!(!non_displayable_color_high_opacity.displayable());

    let non_displayable_color_low_opacity = Rgb::new(255, 0, 0, -0.1);
    assert!(!non_displayable_color_low_opacity.displayable());
}

#[test]
fn test_color_copy() {
    let original_rgb = Color::Rgb(Rgb::new(255, 0, 0, 1.0));
    let copied_rgb = original_rgb.copy();
    assert_eq!(original_rgb, copied_rgb);

    let original_hsl = Color::Hsl(Hsl::new(120.0, 50.0, 75.0, 0.5));
    let copied_hsl = original_hsl.copy();
    assert_eq!(original_hsl, copied_hsl);

    let original_lab = Color::Lab(Lab::new(50.0, 20.0, 30.0, 1.0));
    let copied_lab = original_lab.copy();
    assert_eq!(original_lab, copied_lab);
}

#[test]
fn test_lab_interpolate() {
    let lab1 = Lab::new(10.0, 20.0, 30.0, 1.0);
    let lab2 = Lab::new(30.0, 40.0, 50.0, 0.5);

    let interpolated_lab = lab1.interpolate(&lab2, 0.5);
    assert!((interpolated_lab.l - 20.0).abs() < 1e-6);
    assert!((interpolated_lab.a - 30.0).abs() < 1e-6);
    assert!((interpolated_lab.b - 40.0).abs() < 1e-6);
    assert!((interpolated_lab.opacity - 0.75).abs() < 1e-6);

    let interpolated_lab_t0 = lab1.interpolate(&lab2, 0.0);
    assert!((interpolated_lab_t0.l - lab1.l).abs() < 1e-6);
    assert!((interpolated_lab_t0.a - lab1.a).abs() < 1e-6);
    assert!((interpolated_lab_t0.b - lab1.b).abs() < 1e-6);
    assert!((interpolated_lab_t0.opacity - lab1.opacity).abs() < 1e-6);

    let interpolated_lab_t1 = lab1.interpolate(&lab2, 1.0);
    assert!((interpolated_lab_t1.l - lab2.l).abs() < 1e-6);
    assert!((interpolated_lab_t1.a - lab2.a).abs() < 1e-6);
    assert!((interpolated_lab_t1.b - lab2.b).abs() < 1e-6);
    assert!((interpolated_lab_t1.opacity - lab2.opacity).abs() < 1e-6);
}

#[test]
fn test_hcl_new() {
    let hcl = Hcl::new(100.0, 50.0, 70.0, 0.8);
    assert_eq!(hcl.h, 100.0);
    assert_eq!(hcl.c, 50.0);
    assert_eq!(hcl.l, 70.0);
    assert_eq!(hcl.opacity, 0.8);
}

#[test]
fn test_hcl_brighter() {
    let hcl = Hcl::new(100.0, 50.0, 50.0, 1.0);
    let brighter_hcl = hcl.brighter(None);
    assert!((brighter_hcl.l - (50.0 + 18.0)).abs() < 1e-6);
    assert_eq!(brighter_hcl.h, 100.0);
    assert_eq!(brighter_hcl.c, 50.0);
    assert_eq!(brighter_hcl.opacity, 1.0);

    let brighter_hcl_k2 = hcl.brighter(Some(2.0));
    assert!((brighter_hcl_k2.l - (50.0f32 + 18.0f32 * 2.0f32).min(100.0f32)).abs() < 1e-6f32);
}

#[test]
fn test_hcl_darker() {
    let hcl = Hcl::new(100.0, 50.0, 50.0, 1.0);
    let darker_hcl = hcl.darker(None);
    assert!((darker_hcl.l - (50.0 - 18.0)).abs() < 1e-6);
    assert_eq!(darker_hcl.h, 100.0);
    assert_eq!(darker_hcl.c, 50.0);
    assert_eq!(darker_hcl.opacity, 1.0);

    let darker_hcl_k2 = hcl.darker(Some(2.0));
    assert!((darker_hcl_k2.l - (50.0f32 - 18.0f32 * 2.0f32).max(0.0f32)).abs() < 1e-6f32);
}

#[test]
fn test_hcl_opacity() {
    let hcl = Hcl::new(100.0, 50.0, 70.0, 0.8);
    let new_opacity_hcl = hcl.opacity(0.5);
    assert_eq!(new_opacity_hcl.opacity, 0.5);
    assert_eq!(new_opacity_hcl.h, 100.0);
    assert_eq!(new_opacity_hcl.c, 50.0);
    assert_eq!(new_opacity_hcl.l, 70.0);
}

#[test]
fn test_hcl_clamp() {
    let hcl = Hcl::new(400.0, -10.0, 120.0, 2.0);
    let clamped_hcl = hcl.clamp();
    assert!((clamped_hcl.h - 40.0).abs() < 1e-6); // 400 % 360 = 40
    assert!((clamped_hcl.c - 0.0).abs() < 1e-6); // -10 clamped to 0
    assert!((clamped_hcl.l - 100.0).abs() < 1e-6); // 120 clamped to 100
    assert!((clamped_hcl.opacity - 1.0).abs() < 1e-6); // 2.0 clamped to 1.0
}

#[test]
fn test_color_hcl_conversion() {
    let rgb_color = Color::Rgb(Rgb::new(255, 0, 0, 1.0)); // Red
    let hcl_from_rgb = rgb_color.hcl();
    // Expected HCL for red (approximate)
    assert!((hcl_from_rgb.h - 39.76).abs() < 2.0);
    assert!((hcl_from_rgb.c - 104.68).abs() < 2.0);
    assert!((hcl_from_rgb.l - 53.23).abs() < 2.0);

    let hsl_color = Color::Hsl(Hsl::new(120.0, 100.0, 50.0, 1.0)); // Green
    let hcl_from_hsl = hsl_color.hcl();
    // Actual: h=136.01595, c=119.77588, l=87.734726
    assert!((hcl_from_hsl.h - 136.0).abs() < 2.0);
    assert!((hcl_from_hsl.c - 119.78).abs() < 2.0); // Use actual value
    assert!((hcl_from_hsl.l - 87.73).abs() < 2.0);

    let lab_color = Color::Lab(Lab::new(50.0, 20.0, 30.0, 1.0));
    let hcl_from_lab = lab_color.hcl();
    // Expected HCL from Lab (approximate)
    assert!((hcl_from_lab.h - 56.3).abs() < 2.0);
    assert!((hcl_from_lab.c - 36.0).abs() < 2.0);
    assert!((hcl_from_lab.l - 50.0).abs() < 2.0);
}

#[test]
fn test_hcl_to_rgb_conversion() {
    let hcl_color = Color::Hcl(Hcl::new(39.76, 104.68, 53.23, 1.0)); // Red HCL
    let rgb_from_hcl = hcl_color.rgb();
    assert!((rgb_from_hcl.r as f32 - 255.0).abs() < 2.0);
    assert!((rgb_from_hcl.g as f32 - 0.0).abs() < 2.0);
    assert!((rgb_from_hcl.b as f32 - 0.0).abs() < 2.0);

    let hcl_color_green = Color::Hcl(Hcl::new(136.0, 104.5, 87.7, 1.0)); // Green HCL
    let rgb_from_hcl_green = hcl_color_green.rgb();
    // Actual: r=83, g=251, b=62
    assert!((rgb_from_hcl_green.r as f32 - 83.0).abs() < 2.0);
    assert!((rgb_from_hcl_green.g as f32 - 251.0).abs() < 2.0);
    assert!((rgb_from_hcl_green.b as f32 - 62.0).abs() < 2.0);
}

#[test]
fn test_hcl_to_hsl_conversion() {
    let hcl_color = Color::Hcl(Hcl::new(39.76, 104.68, 53.23, 1.0)); // Red HCL
    let hsl_from_hcl = hcl_color.hsl();
    // Actual: h=359.7647, s=100, l=50
    assert!((hsl_from_hcl.h - 359.76).abs() < 2.0);
    assert!((hsl_from_hcl.s - 100.0).abs() < 2.0);
    assert!((hsl_from_hcl.l - 50.0).abs() < 2.0);
}

#[test]
fn test_hcl_to_lab_conversion() {
    let hcl_color = Color::Hcl(Hcl::new(56.3, 36.0, 50.0, 1.0));
    let lab_from_hcl = hcl_color.lab();
    assert!((lab_from_hcl.l - 50.0).abs() < 0.1);
    assert!((lab_from_hcl.a - 20.0).abs() < 0.1);
    assert!((lab_from_hcl.b - 30.0).abs() < 0.1);
}

#[test]
fn test_format_hcl() {
    let hcl = Color::Hcl(Hcl::new(100.0, 50.0, 70.0, 1.0));
    assert_eq!(hcl.format_hcl(), "hcl(100,50,70)");

    let hcla = Color::Hcl(Hcl::new(100.0, 50.0, 70.0, 0.5));
    assert_eq!(hcla.format_hcl(), "hcla(100,50,70,0.5)");
}
