import connect_python

logger = connect_python.get_logger(__name__)


@connect_python.main
def check_serial_devices(client: connect_python.Client):
    logger.info("Scanning for serial devices...")

    import serial.tools.list_ports
    ports = serial.tools.list_ports.comports()
    for port in ports:
        logger.info(port.device)
    devices = [port.device for port in ports]
    client.set_dropdown_options('serial_device', devices)


if __name__ == "__main__":
    check_serial_devices()
