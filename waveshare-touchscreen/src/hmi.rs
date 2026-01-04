use core::ptr;

use esp_idf_sys::LV_ALIGN_CENTER;
use esp_idf_sys::LV_OBJ_FLAG_CLICKABLE;
use esp_idf_sys::LV_PART_KNOB;
use esp_idf_sys::lv_align_t;
use esp_idf_sys::lv_arc_create;
use esp_idf_sys::lv_arc_set_bg_angles;
use esp_idf_sys::lv_arc_set_value;
use esp_idf_sys::lv_coord_t;
use esp_idf_sys::lv_disp_get_default;
use esp_idf_sys::lv_disp_get_scr_act;
use esp_idf_sys::lv_line_create;
use esp_idf_sys::lv_line_set_points;
use esp_idf_sys::lv_obj_align;
use esp_idf_sys::lv_obj_clean;
use esp_idf_sys::lv_obj_clear_flag;
use esp_idf_sys::lv_obj_flag_t;
use esp_idf_sys::lv_obj_remove_style;
use esp_idf_sys::lv_obj_set_size;
use esp_idf_sys::lv_obj_set_style_arc_color;
use esp_idf_sys::lv_obj_set_style_arc_width;
use esp_idf_sys::lv_obj_set_style_line_color;
use esp_idf_sys::lv_obj_set_style_line_rounded;
use esp_idf_sys::lv_obj_set_style_line_width;
use esp_idf_sys::lv_obj_set_style_pad_bottom;
use esp_idf_sys::lv_obj_set_style_pad_left;
use esp_idf_sys::lv_obj_set_style_pad_right;
use esp_idf_sys::lv_obj_set_style_pad_top;
use esp_idf_sys::lv_obj_t;
use esp_idf_sys::lv_palette_main;
use esp_idf_sys::lv_palette_t_LV_PALETTE_BLUE;
use esp_idf_sys::lv_part_t;
use esp_idf_sys::lv_point_t;

const MOUTH_LINE_POINTS: [lv_point_t; 2] = [lv_point_t { x: 0, y: 0 }, lv_point_t { x: 100, y: 0 }];
const EYE_POINTS: [lv_point_t; 2] = [lv_point_t { x: 0, y: 0 }, lv_point_t { x: 50, y: 0 }];

pub fn draw_neutral_face() {
    unsafe { lv_obj_clean(lv_scr_act()) };

    let left_eye_line = create_line(&EYE_POINTS);
    set_line_style(left_eye_line);
    unsafe { lv_obj_align(left_eye_line, LV_ALIGN_CENTER as lv_align_t, 65, 50) };

    let right_eye_line = create_line(&EYE_POINTS);
    set_line_style(right_eye_line);
    unsafe { lv_obj_align(right_eye_line, LV_ALIGN_CENTER as lv_align_t, -65, 50) };

    let mouth_line = create_line(&MOUTH_LINE_POINTS);
    set_line_style(mouth_line);
    unsafe { lv_obj_align(mouth_line, LV_ALIGN_CENTER as lv_align_t, 0, -25) };
}

pub fn draw_smiling_face() {
    unsafe { lv_obj_clean(lv_scr_act()) };

    // TODO Properly account for line width and object padding
    // For now, arc widths are padded by 8 pixels and the y offset is reduced by 4 pixels
    let left_eye_arc = create_arc(0, 360, 58, 50);
    set_arc_style(left_eye_arc);
    set_offset_from_center(left_eye_arc, 65, 46);

    let right_eye_arc = create_arc(0, 360, 58, 50);
    set_arc_style(right_eye_arc);
    set_offset_from_center(right_eye_arc, -65, 46);

    let mouth_line = create_line(&MOUTH_LINE_POINTS);
    set_line_style(mouth_line);
    set_offset_from_center(mouth_line, 0, -25);

    let mouth_arc = create_arc(180, 360, 108, 100);
    set_arc_style(mouth_arc);
    set_offset_from_center(mouth_arc, 0, -25);
}

/// Set the style use for drawing lines
fn set_line_style(obj: *mut lv_obj_t) {
    unsafe {
        let width = 8;
        let color = lv_palette_main(lv_palette_t_LV_PALETTE_BLUE);
        lv_obj_set_style_line_width(obj, width, 0);
        lv_obj_set_style_line_color(obj, color, 0);
        lv_obj_set_style_line_rounded(obj, true, 0);
        lv_obj_set_style_pad_top(obj, 0, 0);
        lv_obj_set_style_pad_bottom(obj, 0, 0);
        lv_obj_set_style_pad_left(obj, 0, 0);
        lv_obj_set_style_pad_right(obj, 0, 0);
    }
}

/// Set the style use for drawing arcs
fn set_arc_style(obj: *mut lv_obj_t) {
    unsafe {
        let width = 8;
        let color = lv_palette_main(lv_palette_t_LV_PALETTE_BLUE);
        lv_obj_set_style_arc_width(obj, width, 0);
        lv_obj_set_style_arc_color(obj, color, 0);
        // lv_obj_set_style_pad_top(obj, 0, 0);
        // lv_obj_set_style_pad_bottom(obj, 0, 0);
        // lv_obj_set_style_pad_left(obj, 0, 0);
        // lv_obj_set_style_pad_right(obj, 0, 0);
    }
}

/// Draw an arc.
///
/// Angles must range from 0-360.
fn create_arc(
    start_angle: u16,
    end_angle: u16,
    width: lv_coord_t,
    height: lv_coord_t,
) -> *mut lv_obj_t {
    unsafe {
        let arc = lv_arc_create(lv_scr_act());
        lv_obj_set_size(arc, width, height);
        lv_arc_set_bg_angles(arc, start_angle, end_angle);

        // LVGL's arcs are a control object. Hide these elements, leaving only the curve.
        lv_arc_set_value(arc, 0);
        lv_obj_remove_style(arc, ptr::null_mut(), LV_PART_KNOB as lv_part_t);
        lv_obj_clear_flag(arc, LV_OBJ_FLAG_CLICKABLE as lv_obj_flag_t);
        arc
    }
}

fn create_line(points: &[lv_point_t]) -> *mut lv_obj_t {
    unsafe {
        let line = lv_line_create(lv_scr_act());
        // Note: it is theoretically possible for points.len() to overflow
        lv_line_set_points(line, points.as_ptr(), points.len() as u16);
        line
    }
}

fn set_offset_from_center(obj: *mut lv_obj_t, x_ofs: lv_coord_t, y_ofs: lv_coord_t) {
    unsafe { lv_obj_align(obj, LV_ALIGN_CENTER as lv_align_t, x_ofs, y_ofs) };
}

/// Get the active screen of the default display
///
/// An implementation of LVGL's lv_scr_act(), which is an inline function and has no Rust bindings.
unsafe fn lv_scr_act() -> *mut lv_obj_t {
    unsafe { lv_disp_get_scr_act(lv_disp_get_default()) }
}
