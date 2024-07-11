#![no_std]
#![no_main]
use firefly_rust::*;

pub const WIDTH: usize = 240;
pub const HEIGHT: usize = 160;
const PAD_RADIUS: i32 = 60;
const TOUCH_RADIUS: i32 = 10;

const ME_COLOR: Color = Color::DarkBlue;
const PEER_COLOR: Color = Color::Blue;
const COMBINED_COLOR: Color = Color::LightBlue;

const A: Point = Point { x: 160, y: 100 };
const B: Point = Point { x: 190, y: 90 };
const X: Point = Point { x: 160, y: 70 };
const Y: Point = Point { x: 190, y: 60 };

#[no_mangle]
extern fn render() {
    draw_bg();
    draw_pad();
    draw_buttons();
}

fn draw_pad() {
    if let Some(pad) = read_pad(Peer::COMBINED) {
        let touch_pos = Point {
            x: PAD_RADIUS + pad.x / 20,
            y: PAD_RADIUS - pad.y / 20,
        };
        let style = Style {
            fill_color: COMBINED_COLOR,
            stroke_color: Color::None,
            stroke_width: 2,
        };
        draw_circle(touch_pos, TOUCH_RADIUS * 2, style);
    };
    let me = get_me();
    let peers = get_peers();
    for peer in peers {
        let Some(pad) = read_pad(peer) else {
            continue;
        };
        let touch_pos = Point {
            x: PAD_RADIUS + pad.x / 20,
            y: PAD_RADIUS - pad.y / 20,
        };
        let color = if peer == me { ME_COLOR } else { PEER_COLOR };
        let style = Style {
            fill_color: Color::None,
            stroke_color: color,
            stroke_width: 2,
        };
        draw_circle(touch_pos, TOUCH_RADIUS * 2, style);
    }
}

fn draw_buttons() {
    {
        let buttons = read_buttons(Peer::COMBINED);
        let style = Style {
            fill_color: COMBINED_COLOR,
            stroke_color: Color::None,
            stroke_width: 2,
        };
        if buttons.a {
            draw_circle(A, TOUCH_RADIUS * 2, style)
        }
        if buttons.b {
            draw_circle(B, TOUCH_RADIUS * 2, style)
        }
        if buttons.x {
            draw_circle(X, TOUCH_RADIUS * 2, style)
        }
        if buttons.y {
            draw_circle(Y, TOUCH_RADIUS * 2, style)
        }
    }

    let me = get_me();
    let peers = get_peers();
    for peer in peers {
        let buttons = read_buttons(peer);
        let stroke_color = if peer == me { ME_COLOR } else { PEER_COLOR };
        let style = Style {
            fill_color: Color::None,
            stroke_color,
            stroke_width: 2,
        };
        if buttons.a {
            draw_circle(A, TOUCH_RADIUS * 2, style)
        }
        if buttons.b {
            draw_circle(B, TOUCH_RADIUS * 2, style)
        }
        if buttons.x {
            draw_circle(X, TOUCH_RADIUS * 2, style)
        }
        if buttons.y {
            draw_circle(Y, TOUCH_RADIUS * 2, style)
        }
    }
}

fn draw_bg() {
    clear_screen(Color::White);
    let style = Style {
        fill_color: Color::None,
        stroke_color: Color::LightGray,
        stroke_width: 2,
    };
    draw_circle(Point { x: 10, y: 10 }, PAD_RADIUS * 2, style);

    let style = Style {
        fill_color: Color::None,
        stroke_color: Color::Gray,
        stroke_width: 2,
    };
    draw_circle(A, TOUCH_RADIUS * 2, style);
    draw_circle(B, TOUCH_RADIUS * 2, style);
    draw_circle(X, TOUCH_RADIUS * 2, style);
    draw_circle(Y, TOUCH_RADIUS * 2, style);
}
