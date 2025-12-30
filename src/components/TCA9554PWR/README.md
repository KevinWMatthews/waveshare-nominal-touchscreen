# `EXIO`

Waveshare's driver library for TI's `TCA9554PWR` GPIO expander.

## Note

In the original Waveshare example application, this module:
- was located in the directory `EXIO`
- had a circular dependency on the `Buzzer` library

This application moves the GPIO expander driver into a separate ESP-IDF component.
