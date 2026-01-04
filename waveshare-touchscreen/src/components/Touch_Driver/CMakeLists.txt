idf_component_register(
    SRCS
        "GT911.c"
        "esp_lcd_touch/esp_lcd_touch.c"

    INCLUDE_DIRS
         "."
         "esp_lcd_touch"

     REQUIRES driver esp_lcd I2C_Driver TCA9554PWR LCD_Driver
)
