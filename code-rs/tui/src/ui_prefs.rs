use std::sync::atomic::{AtomicBool, Ordering};

use ratatui::widgets::BorderType;

static ROUNDED_CORNERS: AtomicBool = AtomicBool::new(false);

pub(crate) fn set_rounded_corners(enabled: bool) {
    ROUNDED_CORNERS.store(enabled, Ordering::Relaxed);
}

pub(crate) fn rounded_corners_enabled() -> bool {
    ROUNDED_CORNERS.load(Ordering::Relaxed)
}

pub(crate) fn box_border_type() -> BorderType {
    if rounded_corners_enabled() {
        BorderType::Rounded
    } else {
        BorderType::Plain
    }
}

