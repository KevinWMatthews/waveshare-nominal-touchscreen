use std::ffi::CString;
use std::ptr;

use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _;
use esp_idf_sys::EXIO_Init;
use esp_idf_sys::I2C_Init;
use esp_idf_sys::LCD_Init;
use esp_idf_sys::LVGL_Init;
use esp_idf_sys::QMI8658_Init;
use esp_idf_sys::QMI8658_Loop;
use esp_idf_sys::Touch_Init;
use esp_idf_sys::lv_indev_state_t;
use esp_idf_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED;
use esp_idf_sys::lv_indev_state_t_LV_INDEV_STATE_RELEASED;
use esp_idf_sys::lv_point_t;
use esp_idf_sys::lv_timer_handler;
use esp_idf_sys::vTaskDelay;
use esp_idf_sys::xTaskCreatePinnedToCore;
use log::debug;
use log::info;
use log::warn;

const NOMINAL_LOG_TAG: &'static str = "NOMINAL";

fn main() {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    info!("Starting main application");
    unsafe { I2C_Init() };
    unsafe { QMI8658_Init() };
    unsafe { EXIO_Init() };
    unsafe { LCD_Init(ptr::null_mut()) };
    unsafe { Touch_Init() };
    unsafe { LVGL_Init(Some(touch_event_callback)) };

    let task_name = CString::new("Driver task");
    unsafe {
        xTaskCreatePinnedToCore(
            Some(driver_task),
            task_name.unwrap().as_ptr(),
            4096,
            ptr::null_mut(),
            3,
            ptr::null_mut(),
            0,
        )
    };

    info!("Starting LVGL loop");
    loop {
        // raise the task priority of LVGL and/or reduce the handler period can improve the performance
        unsafe { vTaskDelay(ms_to_ticks(10)) };
        // The task running lv_timer_handler should have lower priority than that running `lv_tick_inc`
        unsafe { lv_timer_handler() };
    }
}

extern "C" fn driver_task(_arg: *mut std::ffi::c_void) {
    loop {
        unsafe { QMI8658_Loop() };
        unsafe { vTaskDelay(ms_to_ticks(100)) };
    }
}

/// A Rust definition of the FreeRTOS macro pdMS_TO_TICKS.
///
/// The original definition is:
/// ```c
/// #define pdMS_TO_TICKS( xTimeInMs )    ( ( TickType_t ) ( ( ( uint64_t ) ( xTimeInMs ) * ( uint64_t ) configTICK_RATE_HZ ) / ( uint64_t ) 1000U ) )
/// ```
fn ms_to_ticks(time_in_ms: u32) -> u32 {
    // TODO Be wary of overflow
    (time_in_ms * esp_idf_sys::configTICK_RATE_HZ) / 1000
}

extern "C" fn touch_event_callback(state: lv_indev_state_t, point: lv_point_t) {
    #[allow(non_upper_case_globals)]
    match state {
        lv_indev_state_t_LV_INDEV_STATE_RELEASED => {
            debug!("Release");
        }
        lv_indev_state_t_LV_INDEV_STATE_PRESSED => {
            debug!("Touch");
            info!(target: NOMINAL_LOG_TAG, "X={} Y={}", point.x, point.y);
        }
        val => {
            warn!("Unexpected touch event state: {val}");
        }
    }
}
