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

    if let Some(pad) = read_pad(Peer::COMBINED) {
        let touch_pos = Point {
            x: PAD_RADIUS + pad.x / 20,
            y: PAD_RADIUS - pad.y / 20,
        };
        let style = Style {
            fill_color: Color::DarkBlue,
            stroke_color: Color::None,
            stroke_width: 2,
        };
        draw_circle(touch_pos, TOUCH_RADIUS * 2, style);
    };

    let buttons = read_buttons(Peer::COMBINED);
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
    clear_screen(Color::White);
    let style = Style {
        fill_color: Color::None,
        stroke_color: Color::DarkBlue,
        stroke_width: 2,
    };
    draw_circle(Point { x: 10, y: 10 }, PAD_RADIUS * 2, style);
}

fn button_style(btn: bool) -> Style {
    let fill_color = if btn { Color::DarkBlue } else { Color::None };
    Style {
        fill_color,
        stroke_color: Color::DarkBlue,
        stroke_width: 2,
    }
}
