import re
from datetime import datetime, timezone
from typing import Optional, List

import connect_python
import serial
from connect_python.ts import ConnectTimestamp, Nanoseconds

logger = connect_python.get_logger(__name__)


class ReadingParser:
    @staticmethod
    def parse(data: str) -> Optional[TouchscreenReading | AccelerometerReading | GyroscopeReading]:
        match = TouchscreenReading.parse(data)
        if match:
            return match

        match = AccelerometerReading.parse(data)
        if match:
            return match

        match = GyroscopeReading.parse(data)
        if match:
            return match

        return None


class TouchscreenReading:
    STREAM_ID: str = 'touch'
    channel_names: List[str] = ['x', 'y', 'z']
    timestamp: ConnectTimestamp
    x: int
    y: int

    # Sample data (with color disabled):
    # b'I (18786) NOMINAL TOUCH: X=259 Y=362\r\n'
    # Regex key:
    # `^`: start anchor
    # `$`: end anchor
    # `?`: match 0 or more
    # `+`: match 1 or more
    # `[]`: a character set
    # `()`: A capture group
    # `\(` and `\)`: escaped parenthesis (not a capture group)
    # `?P<name>`: A named capture group
    # `\d`: one or more digits
    # `\s`: a space
    REGEX = re.compile(
        r"^(?P<level>I)\s\((?P<timestamp>\d+)\)\s(?P<module>NOMINAL TOUCH):\sX=(?P<x>\d+)\sY=(?P<y>\d+)$")

    def __init__(self, time_since_boot: Nanoseconds, x: int, y: int) -> None:
        self.timestamp = time_since_boot
        self.x = x
        self.y = y

    def __str__(self) -> str:
        return f'TouchReading: ({self.timestamp}) X={self.x}, Y={self.y}'

    def parse(data: str) -> Optional[TouchscreenReading]:
        match = TouchscreenReading.REGEX.search(data)
        if match is None:
            return None

        # Log data is expected to be at the Info level
        log_level = match.group('level')
        if log_level != 'I':
            return None

        # Espressif log timestamps are in 'ms since boot'
        time_since_boot_ms = match.group('timestamp')
        time_since_boot_ns = int(time_since_boot_ms) * 1_000_000

        # The regex should ensure that these match groups contain integer data
        x = int(match.group('x'))
        y = int(match.group('y'))
        return TouchscreenReading(time_since_boot_ns, x, y)

    def channel_names(self) -> List[str]:
        return ['x', 'y']

    def channel_values(self) -> List[float]:
        return [self.x, self.y]


class AccelerometerReading:
    STREAM_ID: str = "accel"
    timestamp: ConnectTimestamp
    x: float
    y: float
    z: float
    # Sample data (with color disabled):
    # b'I (191252) NOMINAL ACCEL: X = 0.032836914, Y = 0.13195801, Z = 0.13195801\r\n'
    # Regex key:
    # `^`: start anchor
    # `$`: end anchor
    # `?`: match 0 or more
    # `+`: match 1 or more
    # `[]`: a character set
    # `()`: A capture group
    # `\(` and `\)`: escaped parenthesis (not a capture group)
    # `?P<name>`: A named capture group
    # `\d`: one or more digits
    # `\s`: a space
    REGEX = re.compile(
        r"^(?P<level>I)\s\((?P<timestamp>\d+)\)\s(?P<module>NOMINAL ACCEL):\sX=(?P<x>[-]?\d.\d+)\sY=(?P<y>[-]?\d.\d+)\sZ=(?P<z>[-]?\d.\d+)$")

    def __init__(self, time_since_boot: Nanoseconds, x: float, y: float, z: float) -> None:
        self.timestamp = time_since_boot
        self.x = x
        self.y = y
        self.z = z

    def __str__(self) -> str:
        return f'AccelReading: ({self.timestamp}) X={self.x}, Y={self.y}, Z={self.z}'

    def parse(data: str) -> Optional[AccelerometerReading]:
        match = AccelerometerReading.REGEX.search(data)
        if match is None:
            return None

        # Log data is expected to be at the Info level
        log_level = match.group('level')
        if log_level != 'I':
            return None

        # Espressif log timestamps are in 'ms since boot'
        time_since_boot_ms = match.group('timestamp')
        time_since_boot_ns = int(time_since_boot_ms) * 1_000_000

        # The regex should ensure that these match groups contain float data
        x = float(match.group('x'))
        y = float(match.group('y'))
        z = float(match.group('z'))
        return AccelerometerReading(time_since_boot_ns, x, y, z)

    def channel_names(self) -> List[str]:
        return ['x', 'y', 'z']

    def channel_values(self) -> List[float]:
        return [self.x, self.y, self.z]


class GyroscopeReading:
    STREAM_ID: str = "gyro"
    timestamp: ConnectTimestamp
    x: float
    y: float
    z: float
    # Sample data (with color disabled):
    # b'I (191252) NOMINAL GYRO: X = 2.8339844, Y = 1.5917969, Z = 1.5917969\r\n'
    # Regex key:
    # `^`: start anchor
    # `$`: end anchor
    # `?`: match 0 or more
    # `+`: match 1 or more
    # `[]`: a character set
    # `()`: A capture group
    # `\(` and `\)`: escaped parenthesis (not a capture group)
    # `?P<name>`: A named capture group
    # `\d`: one or more digits
    # `\s`: a space
    REGEX = re.compile(
        r"^(?P<level>I)\s\((?P<timestamp>\d+)\)\s(?P<module>NOMINAL GYRO):\sX=(?P<x>[-]?\d.\d+)\sY=(?P<y>[-]?\d.\d+)\sZ=(?P<z>[-]?\d.\d+)$")

    def __init__(self, time_since_boot: Nanoseconds, x: float, y: float, z: float) -> None:
        self.timestamp = time_since_boot
        self.x = x
        self.y = y
        self.z = z

    def __str__(self) -> str:
        return f'GyroReading: ({self.timestamp}) X={self.x}, Y={self.y}, Z={self.z}'

    def parse(data: str) -> Optional[GyroscopeReading]:
        match = GyroscopeReading.REGEX.search(data)
        if match is None:
            return None

        # Log data is expected to be at the Info level
        log_level = match.group('level')
        if log_level != 'I':
            return None

        # Espressif log timestamps are in 'ms since boot'
        time_since_boot_ms = match.group('timestamp')
        time_since_boot_ns = int(time_since_boot_ms) * 1_000_000

        # The regex should ensure that these match groups contain float data
        x = float(match.group('x'))
        y = float(match.group('y'))
        z = float(match.group('z'))
        return GyroscopeReading(time_since_boot_ns, x, y, z)

    def channel_names(self) -> List[str]:
        return ['x', 'y', 'z']

    def channel_values(self) -> List[float]:
        return [self.x, self.y, self.z]


@connect_python.main
def stream_console_data(client: connect_python.Client):
    port = client.get_value('serial_device')
    if not port:
        logger.error('Error: no serial device selected')
        return
    logger.info(f'Streaming data from serial device: {port}')

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

                if client.get_value('log_console'):
                    logger.info(line)
                reading = ReadingParser.parse(line)
                if not reading:
                    logger.debug('Ignored line of console data')
                    continue

                client.stream(reading.STREAM_ID, reading.timestamp, start_time=start_time,
                              names=reading.channel_names(), values=reading.channel_values())

                if type(reading) == TouchscreenReading:
                    if client.get_value('log_touch'):
                        logger.info(reading)
                elif type(reading) == AccelerometerReading:
                    if client.get_value('log_accel'):
                        logger.info(reading)
                elif type(reading) == GyroscopeReading:
                    if client.get_value('log_gyro'):
                        logger.info(reading)
    except serial.serialutil.SerialException as e:
        # Inform the user if the serial connection can't be formed or fails
        logger.error(f'Serial device error: {e}')


if __name__ == "__main__":
    stream_console_data()
