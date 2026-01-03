import re
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
        # b'I (18786) NOMINAL: X=259 Y=362\r\n'
        # Regex key::
        # `^`: start anchor
        # `$`: end anchor
        # `()`: A capture group
        # `\(` and `\)`: escaped parenthesis (not a capture group)
        # `?P<name>`: A named capture group
        # `\d+`: one or more digits
        # `\s`: a space
        pattern = r"^(?P<level>I)\s\((?P<timestamp>\d+)\)\s(?P<module>NOMINAL):\sX=(?P<x>\d+)\sY=(?P<y>\d+)$"
        match = re.search(pattern, data)
        if match is None:
            return None

        module = match.group('module')
        if module != TouchscreenReading.NOMINAL_LOG_TAG:
            return None

        log_level = match.group('level')
        if log_level != 'I':
            return None

        time_since_boot_ms = match.group('timestamp')
        time_since_boot_ns = int(time_since_boot_ms) * 1_000_000  # TODO Will this always match?

        x = int(match.group('x'))
        y = int(match.group('y'))
        return TouchscreenReading(time_since_boot_ns, x, y)


@connect_python.main
def stream_console_data(client: connect_python.Client):
    port = client.get_value('serial_device')
    if not port:
        logger.error('No serial device selected')
        raise Exception('No serial device selected')
    logger.info(f'Streaming from serial device: {port}')

    start_time = datetime.now(timezone.utc)
    try:
        with serial.Serial(port=port, baudrate=115200) as serial_device:
            while True:
                line: bytes = serial_device.readline()
                try:
                    line: str = line.decode('utf-8').strip()
                except UnicodeDecodeError:
                    # There can be garbage on the console when reflashing.
                    # Silently ignore it, as the console connection is still active
                    logger.debug('Ignoring non-unicode data')
                    continue

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
    except serial.serialutil.SerialException as e:
        # Inform the user if the serial connection can't be formed or fails
        logger.error(f'Serial device error: {e}')


if __name__ == "__main__":
    stream_console_data()
