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
    let me = get_me();
    let peers = get_peers();
    for peer in peers {
        let buttons = read_buttons(peer);
        let is_me = peer == me;
        draw_circle(
            Point { x: 160, y: 100 },
            TOUCH_RADIUS * 2,
            button_style(buttons.a, is_me),
        );
        draw_circle(
            Point { x: 190, y: 90 },
            TOUCH_RADIUS * 2,
            button_style(buttons.b, is_me),
        );
        draw_circle(
            Point { x: 160, y: 70 },
            TOUCH_RADIUS * 2,
            button_style(buttons.x, is_me),
        );
        draw_circle(
            Point { x: 190, y: 60 },
            TOUCH_RADIUS * 2,
            button_style(buttons.y, is_me),
        );
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
    draw_circle(Point { x: 160, y: 100 }, TOUCH_RADIUS * 2, style);
    draw_circle(Point { x: 190, y: 90 }, TOUCH_RADIUS * 2, style);
    draw_circle(Point { x: 160, y: 70 }, TOUCH_RADIUS * 2, style);
    draw_circle(Point { x: 190, y: 60 }, TOUCH_RADIUS * 2, style);
}

fn button_style(btn: bool, is_me: bool) -> Style {
    let fill_color = if btn { COMBINED_COLOR } else { Color::None };
    let stroke_color = if btn {
        if is_me {
            ME_COLOR
        } else {
            PEER_COLOR
        }
    } else {
        Color::None
    };
    Style {
        fill_color,
        stroke_color,
        stroke_width: 2,
    }
}
