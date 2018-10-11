use x86::instruction::{outb, inb};
//use core::sync::atomic::{AtomicBool, Ordering};

const COM1: u16 = 0x3F8;
const COM_RX: u16 = 0;
// In:Receive buffer (DLAB=0)
const COM_TX: u16 = 0;
// Out: Transmit buffer (DLAB=0)
const COM_DLL: u16 = 0;
// Out: Divisor Latch Low (DLAB=1)
const COM_DLM: u16 = 1;
// Out: Divisor Latch High (DLAB=1)
const COM_IER: u16 = 1;
// Out: Interrupt Enable Register
const COM_IER_RDI: u8 = 0x01;
//   Enable receiver data interrupt
const COM_IIR: u16 = 2;
// In:	Interrupt ID Register
const COM_FCR: u16 = 2;
// Out: FIFO Control Register
const COM_LCR: u16 = 3;
// Out: Line Control Register
const COM_LCR_DLAB: u8 = 0x80;
//   Divisor latch access bit
const COM_LCR_WLEN8: u8 = 0x03;
//   Wordlength: 8 bits
const COM_MCR: u16 = 4;
// Out: Modem Control Register
const COM_MCR_RTS: u16 = 0x02;
// RTS complement
const COM_MCR_DTR: u16 = 0x01;
// DTR complement
const COM_MCR_OUT2: u16 = 0x08;
// Out2 complement
const COM_LSR: u16 = 5;
// In:	Line Status Register
const COM_LSR_DATA: u8 = 0x01;
//   Data available
const COM_LSR_TXRDY: u8 = 0x20;
//   Transmit buffer avail
const COM_LSR_TSRE: u8 = 0x40;

//static UART_INITIALIZED: AtomicBool = AtomicBool::new(false);

//   Transmitter off
pub struct Uart {
    serial_exists: bool
}


impl Uart {
    pub fn new() -> Result<Uart, ()> {
        unsafe { Self::init() }
    }

    unsafe fn init() -> Result<Uart, ()> {
        // Turn off the FIFO
        outb(COM1 + COM_FCR, 0);

        // Set speed; requires DLAB latch
        outb(COM1 + COM_LCR, COM_LCR_DLAB);
        outb(COM1 + COM_DLL, 12);//12 = (uint8_t)(115200 / 9600)
        outb(COM1 + COM_DLM, 0);

        // 8 data bits, 1 stop bit, parity off; turn off DLAB latch
        outb(COM1 + COM_LCR, COM_LCR_WLEN8 & !COM_LCR_DLAB);

        // No modem controls
        outb(COM1 + COM_MCR, 0);
        // Enable rcv interrupts
        outb(COM1 + COM_IER, COM_IER_RDI);

        // Clear any preexisting overrun indications and interrupts
        // Serial port doesn't exist if COM_LSR returns 0xFF
        let serial_exists = inb(COM1 + COM_LSR) != 0xFF;
        inb(COM1 + COM_IIR);
        inb(COM1 + COM_RX);
        Ok(Uart { serial_exists })
    }

    pub fn read_byte(&self) -> Option<u8> {
        if !self.serial_exists {
            return None
        }
        unsafe {
            //wait com port empty
            for _ in 0..10000 {
                if (inb(COM1 + COM_IER) & COM_LSR_DATA) != 0 {
                    return Some(inb(COM1))
                }
            }
            None
        }
    }

    pub fn write_byte(&self, value: u8) {
        unsafe {
            //wait com port empty
            for _ in 0..10000 {
                if inb(COM1 + COM_LSR) & COM_LSR_TXRDY != 0 {
                    break;
                }
            }
            outb(COM1, value);
        }
    }
}