use esp_idf_sys as _;
use esp_idf_sys::I2C_Init;
use esp_idf_sys::EXIO_Init;
use esp_idf_sys::LCD_Init;
use esp_idf_sys::Touch_Init;
use std::ptr;

fn main() {
    println!("Hello, world!");
    unsafe { I2C_Init() };
    unsafe { EXIO_Init() };
    unsafe { LCD_Init(ptr::null_mut()) };
    unsafe { Touch_Init() };
}
