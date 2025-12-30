use esp_idf_sys as _;
use esp_idf_sys::I2C_Init;
use esp_idf_sys::EXIO_Init;

fn main() {
    println!("Hello, world!");
    unsafe { I2C_Init() };
    unsafe { EXIO_Init() };
}
