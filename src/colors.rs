#[macro_export]
macro_rules! hex_color {
    ($r:expr, $g:expr, $b:expr) => {{
        Color::rgb($r as f32 / 255., $g as f32 / 255., $b as f32 / 255.)
    }};
}
