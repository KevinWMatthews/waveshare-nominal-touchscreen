use esp_idf_sys as _;
use esp_idf_sys::I2C_Init;
use esp_idf_sys::EXIO_Init;
use esp_idf_sys::LCD_Init;
use esp_idf_sys::QMI8658_Init;
use esp_idf_sys::QMI8658_Loop;
use esp_idf_sys::Touch_Init;
use esp_idf_sys::vTaskDelay;
use std::ptr;

fn main() {
    println!("Hello, world!");
    unsafe { I2C_Init() };
    unsafe { QMI8658_Init() };
    unsafe { EXIO_Init() };
    unsafe { LCD_Init(ptr::null_mut()) };
    unsafe { Touch_Init() };
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
