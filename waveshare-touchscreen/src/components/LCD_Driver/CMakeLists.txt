idf_component_register(
    SRCS
        "ST7701S.c"

    INCLUDE_DIRS
         "."

     REQUIRES TCA9554PWR esp_lcd
)
