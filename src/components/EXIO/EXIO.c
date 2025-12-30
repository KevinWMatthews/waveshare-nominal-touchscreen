#include "TCA9554PWR.h"
#include "Buzzer.h"

esp_err_t EXIO_Init(void)
{
    TCA9554PWR_Init(0x00);
    Buzzer_Off();
    return ESP_OK;
}
