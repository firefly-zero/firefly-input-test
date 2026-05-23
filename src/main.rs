#![no_std]
#![no_main]
#![expect(static_mut_refs)]
#![deny(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]

use core::mem::MaybeUninit;
use firefly_rust::*;

const PAD_RADIUS: i32 = 60;
const TOUCH_RADIUS: i32 = 10;

const S: Point = Point { x: 180, y: 95 };
const E: Point = Point { x: 200, y: 75 };
const W: Point = Point { x: 160, y: 75 };
const N: Point = Point { x: 180, y: 55 };

static mut FONT_BUF: [u8; 871] = [0; 871];
static mut FONT: MaybeUninit<FontRef<'static>> = MaybeUninit::uninit();
static mut THEME: MaybeUninit<Theme> = MaybeUninit::uninit();

fn get_theme() -> Theme {
    unsafe { THEME.assume_init() }
}

#[unsafe(no_mangle)]
extern "C" fn boot() {
    unsafe {
        let file = load_file("font", &mut FONT_BUF);
        FONT = MaybeUninit::new(file.into());
        let theme = get_settings(get_me()).theme;
        THEME = MaybeUninit::new(theme);
    }
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    clear_screen(get_theme().bg);
    draw_pad_bg();
    draw_pad();
    draw_buttons();
}

fn draw_pad() {
    let me = get_me();
    let peers = get_peers();
    for peer in peers {
        draw_peer_pad(peer, peer == me);
    }
    draw_combined_pad();
}

fn draw_combined_pad() {
    let Some(pad) = read_pad(Peer::COMBINED) else {
        return;
    };
    let style = Style {
        fill_color: Color::None,
        stroke_color: get_theme().primary,
        stroke_width: 2,
    };
    draw_touch(pad, style);
}

fn draw_peer_pad(peer: Peer, is_me: bool) {
    let Some(pad) = read_pad(peer) else {
        return;
    };
    let theme = get_theme();
    let color = if is_me { theme.accent } else { theme.primary };
    let style = Style {
        fill_color: color,
        stroke_color: theme.secondary,
        stroke_width: 2,
    };
    draw_touch(pad, style);
}

fn draw_touch(pad: Pad, style: Style) {
    let touch_pos = Point {
        x: PAD_RADIUS + pad.x / 20,
        y: PAD_RADIUS - pad.y / 20,
    };
    draw_circle(touch_pos, TOUCH_RADIUS * 2, style);

    let style = Style {
        fill_color: get_theme().bg,
        stroke_color: Color::None,
        stroke_width: 0,
    };
    let dpad = pad.as_dpad8();
    if dpad.left {
        let point = touch_pos + Point::new(-1, TOUCH_RADIUS - 1);
        draw_rect(point, Size::new(2, 2), style);
    }
    if dpad.right {
        let point = touch_pos + Point::new(TOUCH_RADIUS * 2 - 1, TOUCH_RADIUS - 1);
        draw_rect(point, Size::new(2, 2), style);
    }
    if dpad.up {
        let point = touch_pos + Point::new(TOUCH_RADIUS - 1, -1);
        draw_rect(point, Size::new(2, 2), style);
    }
    if dpad.down {
        let point = touch_pos + Point::new(TOUCH_RADIUS - 1, TOUCH_RADIUS * 2 - 1);
        draw_rect(point, Size::new(2, 2), style);
    }
}

fn draw_buttons() {
    draw_combined_buttons();
    let me = get_me();
    let peers = get_peers();
    for peer in peers {
        draw_peer_buttons(peer, peer == me);
    }
}

fn draw_combined_buttons() {
    let buttons = read_buttons(Peer::COMBINED);
    draw_button(S, "S", buttons.s);
    draw_button(E, "E", buttons.e);
    draw_button(W, "W", buttons.w);
    draw_button(N, "N", buttons.n);
}

fn draw_button(p: Point, name: &str, pressed: bool) {
    let theme = get_theme();
    let style = Style {
        fill_color: theme.bg,
        stroke_color: theme.primary,
        stroke_width: 2,
    };
    let shadow = Style {
        fill_color: theme.primary,
        stroke_color: theme.primary,
        stroke_width: 2,
    };
    let shift = Point::new(1, 1);

    if pressed {
        draw_circle(p, TOUCH_RADIUS * 2, style);
    } else {
        draw_circle(p, TOUCH_RADIUS * 2, shadow);
        draw_circle(p - shift, TOUCH_RADIUS * 2, style);
    }
    draw_button_name(p - shift, name);
}

fn draw_button_name(p: Point, name: &str) {
    let font = unsafe { FONT.assume_init_mut() };
    let text_shift = Point::new(8, 12);
    draw_text(name, font, p + text_shift, get_theme().primary);
}

fn draw_peer_buttons(peer: Peer, is_me: bool) {
    let buttons = read_buttons(peer);
    let theme = get_theme();
    let fill_color = if is_me { theme.accent } else { theme.primary };
    let style = Style {
        fill_color,
        stroke_color: theme.primary,
        stroke_width: 2,
    };
    if buttons.s {
        draw_circle(S, TOUCH_RADIUS * 2, style);
        draw_button_name(S, "S");
    }
    if buttons.e {
        draw_circle(E, TOUCH_RADIUS * 2, style);
        draw_button_name(E, "E");
    }
    if buttons.w {
        draw_circle(W, TOUCH_RADIUS * 2, style);
        draw_button_name(W, "W");
    }
    if buttons.n {
        draw_circle(N, TOUCH_RADIUS * 2, style);
        draw_button_name(N, "N");
    }
}

fn draw_pad_bg() {
    let theme = get_theme();
    let style = Style {
        fill_color: theme.bg,
        stroke_color: theme.primary,
        stroke_width: 2,
    };
    draw_circle(Point::new(11, 11), PAD_RADIUS * 2, style);
    draw_circle(Point::new(10, 10), PAD_RADIUS * 2, style);
}
