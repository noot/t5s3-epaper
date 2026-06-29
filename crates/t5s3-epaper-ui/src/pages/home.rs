use core::fmt::Write as _;

use embedded_graphics::{
    image::Image,
    mono_font::{
        ascii::{FONT_6X10, FONT_9X18_BOLD},
        MonoTextStyle,
    },
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
    text::{Alignment, Text},
};
use embedded_graphics_core::pixelcolor::{Gray4, GrayColor};
use t5s3_epaper_core::Display;
use tinybmp::Bmp;

use crate::{
    datetime::{DAY_NAMES, MONTH_NAMES},
    fmt::FmtBuf,
    layout::{SCREEN_W, STATUS_H},
    screen::Screen,
};

const COLS: usize = 3;
const ICON_W: u16 = 150;
const ICON_H: u16 = 220;
const GAP_X: u16 = 20;
const GAP_Y: u16 = 40;
const GRID_TOP_Y: i32 = STATUS_H + 40;

pub(crate) struct Icon {
    label: &'static str,
    pub(crate) screen: Screen,
}

pub(crate) const ICONS: [Icon; 7] = [
    Icon {
        label: "GPS",
        screen: Screen::Gps,
    },
    Icon {
        label: "LoRa",
        screen: Screen::Lora,
    },
    Icon {
        label: "Light",
        screen: Screen::Frontlight,
    },
    Icon {
        label: "Sleep",
        screen: Screen::Sleep,
    },
    Icon {
        label: "Files",
        screen: Screen::Files,
    },
    Icon {
        label: "Info",
        screen: Screen::Info,
    },
    Icon {
        label: "Settings",
        screen: Screen::Settings,
    },
];

fn grid_origin_x() -> i32 {
    (SCREEN_W - (COLS as i32 * ICON_W as i32 + (COLS as i32 - 1) * GAP_X as i32)) / 2
}

fn icon_rect(idx: usize) -> (i32, i32) {
    let col = idx % COLS;
    let row = idx / COLS;
    (
        grid_origin_x() + (col as i32) * (ICON_W as i32 + GAP_X as i32),
        GRID_TOP_Y + (row as i32) * (ICON_H as i32 + GAP_Y as i32),
    )
}

pub(crate) fn hit_test(sx: i32, sy: i32) -> Option<usize> {
    for i in 0..ICONS.len() {
        let (x, y) = icon_rect(i);
        if sx >= x && sx < x + ICON_W as i32 && sy >= y && sy < y + ICON_H as i32 {
            return Some(i);
        }
    }
    None
}

const ICON_PX: i32 = 100;

fn icon_bytes(screen: Screen) -> Option<&'static [u8]> {
    Some(match screen {
        Screen::Gps => include_bytes!("../../assets/icons/gps.bmp"),
        Screen::Lora => include_bytes!("../../assets/icons/lora.bmp"),
        Screen::Frontlight => include_bytes!("../../assets/icons/light.bmp"),
        Screen::Sleep => include_bytes!("../../assets/icons/sleep.bmp"),
        Screen::Files => include_bytes!("../../assets/icons/files.bmp"),
        Screen::Info => include_bytes!("../../assets/icons/info.bmp"),
        Screen::Settings => include_bytes!("../../assets/icons/settings.bmp"),
        Screen::Home | Screen::Image | Screen::Reader => return None,
    })
}

fn draw_glyph(display: &mut Display, screen: Screen, cx: i32, cy: i32) {
    let Some(bytes) = icon_bytes(screen) else {
        return;
    };
    let Ok(bmp) = Bmp::<Gray4>::from_slice(bytes) else {
        return;
    };
    Image::new(&bmp, Point::new(cx - ICON_PX / 2, cy - ICON_PX / 2))
        .draw(display)
        .ok();
}

pub(crate) fn draw_home(display: &mut Display, date: Option<(usize, i64, u32, u32)>) {
    let bold = MonoTextStyle::new(&FONT_9X18_BOLD, Gray4::BLACK);
    let small = MonoTextStyle::new(&FONT_6X10, Gray4::new(4));

    // date header above the icon grid, once the clock has been synced
    if let Some((dow, year, month, day)) = date {
        let mut buf = FmtBuf::<32>::new();
        write!(
            buf,
            "{}, {} {}, {}",
            DAY_NAMES[dow],
            MONTH_NAMES[(month - 1) as usize],
            day,
            year
        )
        .ok();
        Text::with_alignment(
            buf.as_str(),
            Point::new(SCREEN_W / 2, 82),
            bold,
            Alignment::Center,
        )
        .draw(display)
        .ok();
    }

    let border = PrimitiveStyleBuilder::new()
        .stroke_color(Gray4::BLACK)
        .stroke_width(2)
        .fill_color(Gray4::WHITE)
        .build();

    for (i, icon) in ICONS.iter().enumerate() {
        let (x, y) = icon_rect(i);

        RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::new(x, y), Size::new(ICON_W as u32, ICON_H as u32)),
            Size::new(16, 16),
        )
        .into_styled(border)
        .draw(display)
        .ok();

        draw_glyph(display, icon.screen, x + ICON_W as i32 / 2, y + 80);

        Text::with_alignment(
            icon.label,
            Point::new(x + ICON_W as i32 / 2, y + ICON_H as i32 - 40),
            small,
            Alignment::Center,
        )
        .draw(display)
        .ok();
    }
}
