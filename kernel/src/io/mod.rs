use core::result;
//pub use self::uart::Uart;

pub mod uart;

/*
type Result<T> = result::Result<T, ()>;
trait Read<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}
*/
