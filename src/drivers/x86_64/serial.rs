use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;

// VARIABLES
static mut COM1: SerialPort = unsafe { SerialPort::new(SERIAL_IO_PORT) };

// FUNCTIONS
pub fn message_output_init() {
	unsafe { COM1.init() };
}

pub fn output_message_byte(byte: u8) {
	unsafe { COM1.send(byte) };
}