idf_component_register(
    SRCS
        "lvgl_driver.c"

    INCLUDE_DIRS
         "."

     REQUIRES Touch_Driver LCD_Driver esp_timer lvgl__lvgl
 )
