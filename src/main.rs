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

    let buttons = read_buttons();
    draw_circle(
        Point { x: 160, y: 100 },
        TOUCH_RADIUS * 2,
        button_style(buttons.a),
    );
    draw_circle(
        Point { x: 190, y: 90 },
        TOUCH_RADIUS * 2,
        button_style(buttons.b),
    );
    draw_circle(
        Point { x: 160, y: 70 },
        TOUCH_RADIUS * 2,
        button_style(buttons.x),
    );
    draw_circle(
        Point { x: 190, y: 60 },
        TOUCH_RADIUS * 2,
        button_style(buttons.y),
    );
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

fn button_style(btn: bool) -> Style {
    let fill_color = if btn { Color::ACCENT } else { Color::NONE };
    Style {
        fill_color,
        stroke_color: Color::DARK,
        stroke_width: 2,
    }
}
