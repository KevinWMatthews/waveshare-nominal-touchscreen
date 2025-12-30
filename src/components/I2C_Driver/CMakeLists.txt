idf_component_register(
    SRCS
        "I2C_Driver.c"

    INCLUDE_DIRS
         "."

     REQUIRES
         driver esp_driver_gpio
)
