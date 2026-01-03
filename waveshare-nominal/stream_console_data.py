from datetime import datetime, timezone
from typing import Optional

import connect_python
import serial
from connect_python.ts import ConnectTimestamp, Nanoseconds

logger = connect_python.get_logger(__name__)


class TouchscreenReading:
    NOMINAL_LOG_TAG = "NOMINAL"

    timestamp: ConnectTimestamp
    x: int
    y: int

    def __init__(self, time_since_boot: Nanoseconds, x: int, y: int) -> None:
        self.timestamp = time_since_boot
        self.x = x
        self.y = y

    def __str__(self) -> str:
        return f'TouchReading: ({self.timestamp}) X={self.x}, Y={self.y}'

    @staticmethod
    def parse(data: str) -> Optional[TouchscreenReading]:
        # Sample data (with color disabled):
        # b'I (18786) LVGL: X=259 Y=362\r\n'
        parts = data.split(" ")
        log_level = parts[0]
        time_since_boot_ms = parts[1].strip('()')
        time_since_boot_ns = int(time_since_boot_ms) * 1_000_000
        module = parts[2].rstrip(':')
        if module == TouchscreenReading.NOMINAL_LOG_TAG:
            x = int(parts[3].lstrip('X='))
            y = int(parts[4].lstrip('Y='))
            return TouchscreenReading(time_since_boot_ns, x, y)
        else:
            return None


@connect_python.main
def stream_console_data(client: connect_python.Client):
    port = client.get_value('serial_device')
    if not port:
        logger.error('No serial device selected')
        raise Exception('No serial device selected')
    logger.info(f'Streaming from serial device: {port}')

    start_time = datetime.now(timezone.utc)
    with serial.Serial(port=port, baudrate=115200) as serial_device:
        while True:
            line = serial_device.readline()
            line = line.decode('utf-8').strip()
            logger.debug(line)
            reading: Optional[TouchscreenReading] = TouchscreenReading.parse(line)
            if not reading:
                logger.debug('Ignored reading')
                continue
            logger.info(reading)

            client.stream(
                "touch", reading.timestamp, reading.x, name="x_coord", start_time=start_time
            )
            client.stream(
                "touch", reading.timestamp, reading.y, name="y_coord", start_time=start_time
            )


if __name__ == "__main__":
    stream_console_data()
