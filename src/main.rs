#![no_std]
#![no_main]
use firefly_rust::*;

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 160;
const PAD_RADIUS: i32 = 60;
const TOUCH_RADIUS: i32 = 10;

#[no_mangle]
extern fn render() {
    draw_bg();

    if let Some(pad) = read_pad() {
        let touch_pos = Point {
            x: PAD_RADIUS + pad.x / 20,
            y: PAD_RADIUS - pad.y / 20,
        };
        let style = Style {
            fill_color:   Color::ACCENT,
            stroke_color: Color::NONE,
            stroke_width: 2,
        };
        draw_circle(touch_pos, TOUCH_RADIUS * 2, style);
    };
}

fn draw_bg() {
    clear_screen(Color::LIGHT);
    let style = Style {
        fill_color:   Color::NONE,
        stroke_color: Color::DARK,
        stroke_width: 2,
    };
    draw_circle(Point { x: 10, y: 10 }, PAD_RADIUS * 2, style);
}
