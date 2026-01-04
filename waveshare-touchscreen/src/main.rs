mod hmi;
use std::ffi::CString;
use std::ffi::c_void;
use std::ptr;
use std::sync::mpsc::Sender;

use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _;
use esp_idf_sys::EXIO_Init;
use esp_idf_sys::I2C_Init;
use esp_idf_sys::IMUdata;
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

use crate::hmi::draw_neutral_face;
use crate::hmi::draw_smiling_face;

const ACCEL_LOG_TAG: &'static str = "NOMINAL ACCEL";
const GYRO_LOG_TAG: &'static str = "NOMINAL GYRO";
const TOUCH_LOG_TAG: &'static str = "NOMINAL TOUCH";

#[derive(Clone, Debug)]
enum NominalDataPoint {
    Touch(lv_point_t),
    Accel(IMUdata),
    Gyro(IMUdata),
}

impl NominalDataPoint {
    fn touch_point_from(data: lv_point_t) -> Self {
        NominalDataPoint::Touch(data)
    }

    fn accel_point_from(data: IMUdata) -> Self {
        NominalDataPoint::Accel(data)
    }

    fn gyro_point_from(data: IMUdata) -> Self {
        NominalDataPoint::Gyro(data)
    }
}

fn main() {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    info!("Starting main application");
    unsafe { I2C_Init() };
    unsafe { QMI8658_Init() };
    unsafe { EXIO_Init() };
    unsafe { LCD_Init(ptr::null_mut()) };
    unsafe { Touch_Init() };

    let (log_tx, log_rx) = std::sync::mpsc::channel::<NominalDataPoint>();
    // NOTE: This Box is leaked so that it can be used by the LVGL touch event callback.
    // LVGL isn't currently torn down, so this resource is never deallocated.
    let lvgl_log_tx = Box::leak(Box::new(log_tx.clone()));
    let ptr = lvgl_log_tx as *mut Sender<NominalDataPoint> as *mut c_void;
    unsafe { LVGL_Init(Some(touch_event_callback), ptr) };

    std::thread::Builder::new()
        .stack_size(4096 * 3) // TODO tune stack size
        .spawn(move || {
            loop {
                // NOTE: Blocking receive call
                match log_rx.recv() {
                    Ok(msg) => {
                        debug!("Log channel RX: {msg:?}");
                        match msg {
                            NominalDataPoint::Touch(point) => {
                                info!(target: TOUCH_LOG_TAG, "X={} Y={}", point.x, point.y);
                            }
                            NominalDataPoint::Accel(accel) => {
                                info!(target: ACCEL_LOG_TAG, "X={} Y={} Z={}", accel.x, accel.y, accel.z);
                            }
                            NominalDataPoint::Gyro(gyro) => {
                                info!(target: GYRO_LOG_TAG, "X={} Y={} Z={}", gyro.x, gyro.y, gyro.z);
                            }
                        }
                    }
                    Err(_err) => {
                        warn!("Log channel closed");
                        return;
                    }
                }
            }
        })
        .expect("Spawning log thread");

    let driver_log_tx = Box::into_raw(Box::new(log_tx.clone()));
    let task_name = CString::new("Driver task");
    unsafe {
        xTaskCreatePinnedToCore(
            Some(driver_task),
            task_name.unwrap().as_ptr(),
            4096 * 2, // TODO Tune this stack size
            driver_log_tx as *mut c_void,
            3,
            ptr::null_mut(),
            0,
        )
    };

    info!("Drawing screen and starting LVGL loop");
    draw_neutral_face();
    loop {
        // raise the task priority of LVGL and/or reduce the handler period can improve the performance
        unsafe { vTaskDelay(ms_to_ticks(10)) };
        // The task running lv_timer_handler should have lower priority than that running `lv_tick_inc`
        unsafe { lv_timer_handler() };
    }
}

extern "C" fn driver_task(arg: *mut std::ffi::c_void) {
    let ptr = arg as *mut Sender<NominalDataPoint>;
    let log_tx = unsafe { Box::from_raw(ptr) };
    loop {
        let mut accel = IMUdata::default();
        let mut gyro = IMUdata::default();
        unsafe { QMI8658_Loop(&mut accel, &mut gyro) };
        let _ = log_tx.send(NominalDataPoint::accel_point_from(accel));
        let _ = log_tx.send(NominalDataPoint::gyro_point_from(gyro));
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

pub extern "C" fn touch_event_callback(
    state: lv_indev_state_t,
    prev_state: lv_indev_state_t,
    point: lv_point_t,
    user_data: *mut c_void,
) {
    let log_tx = &mut unsafe { (*(user_data as *mut Sender<NominalDataPoint>)).clone() };
    #[allow(non_upper_case_globals)]
    match (prev_state, state) {
        (lv_indev_state_t_LV_INDEV_STATE_RELEASED, lv_indev_state_t_LV_INDEV_STATE_RELEASED) => {}
        (lv_indev_state_t_LV_INDEV_STATE_RELEASED, lv_indev_state_t_LV_INDEV_STATE_PRESSED) => {
            debug!("Touch");
            let _ = log_tx.send(NominalDataPoint::touch_point_from(point));
            draw_smiling_face();
        }
        (lv_indev_state_t_LV_INDEV_STATE_PRESSED, lv_indev_state_t_LV_INDEV_STATE_PRESSED) => {
            // Log coordinate changes while user is touching screen
            let _ = log_tx.send(NominalDataPoint::touch_point_from(point));
        }
        (lv_indev_state_t_LV_INDEV_STATE_PRESSED, lv_indev_state_t_LV_INDEV_STATE_RELEASED) => {
            debug!("Release");
            draw_neutral_face();
        }
        val => {
            warn!("Unexpected touch event state: {val:?}");
        }
    }
}
