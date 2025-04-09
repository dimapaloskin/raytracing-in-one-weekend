use crate::math::Vector3;

pub type Color = Vector3;

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn vec3_to_color(vec: &Vector3) -> u32 {
    let r = linear_to_gamma(vec.x);
    let g = linear_to_gamma(vec.y);
    let b = linear_to_gamma(vec.z);

    let r = (r * 255.0).clamp(0.0, 255.0) as u8;
    let g = (g * 255.0).clamp(0.0, 255.0) as u8;
    let b = (b * 255.0).clamp(0.0, 255.0) as u8;
    (0xff as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}

pub fn floats_to_color(r: f64, g: f64, b: f64) -> u32 {
    let r = (r * 255.0).clamp(0.0, 255.0) as u8;
    let g = (g * 255.0).clamp(0.0, 255.0) as u8;
    let b = (b * 255.0).clamp(0.0, 255.0) as u8;
    (0xff as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}
