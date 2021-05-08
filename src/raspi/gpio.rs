/**
 * Raspberry pi GPIO Library
 * @file gpio.rs
 * @author hinata
 * @date 2021/05/08 created
 */
use anyhow::{Result};
use thiserror::Error;

pub struct Gpio {
  pub mmap_addr: *mut libc::c_void
}

#[allow(dead_code)]
pub enum GpioResult
{
  CanNotOpenMemDevice = 0x1000
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum GpioError {
  #[error("Error Info (err_code: {err_code:?}, {message:?})")]
  ErrInfo {
      err_code: u32,
      message: String,
  },
  #[error("ErrCode: {0}")]
  ErrCode(u32),
}

#[allow(dead_code)]
pub enum PioMode
{
  Input = 0b000,
  Output = 0b001
}

#[allow(dead_code)]
pub enum GpioPin
{
  Gpio4 = 4,
  Gpio5,
  Gpio6,
  Gpio12 = 12,
  Gpio13,
  Gpio16 = 16,
  Gpio17,
  Gpio18,
  Gpio19,
  Gpio20,
  Gpio21,
  Gpio22,
  Gpio23,
  Gpio24,
  Gpio25,
  Gpio26
}

#[allow(dead_code)]
enum GpfSet
{
  GpfSet0 = 0x001C,
  GpfSet1 = 0x0020
}

#[allow(dead_code)]
enum GpfClr
{
  GpfClr0 = 0x0028,
  GpfClr1 = 0x002C
}

impl Gpio {
    const MMAP_BLOCK_SIZE: usize = 4096;
    const OFFSET_SIZE: u32 = 0x20_0000;

    pub fn new() -> Self
    {
         Gpio {mmap_addr: libc::MAP_FAILED}
    }
    #[allow(dead_code)]
    fn drop(&mut self) {
        if self.mmap_addr != libc::MAP_FAILED {
    
            // destroy memory mapping
            unsafe {libc::munmap(self.mmap_addr, Gpio::MMAP_BLOCK_SIZE); };
        }
    }

    #[allow(dead_code)]
    pub fn initialize(&mut self) -> Result<GpioError> {
        let base_addr;

        self.mmap_addr = libc::MAP_FAILED;

        unsafe {
          base_addr = mmal_sys::bcm_host_get_peripheral_address() + Gpio::OFFSET_SIZE;
        };
  
        let mmap_fd = Gpio::open_devmem();
        if mmap_fd == -1 {
          anyhow::bail!(GpioError::ErrInfo{err_code:GpioResult::CanNotOpenMemDevice as u32, message:"cannot open device memdevice".to_string()})
        }
        else {
            self.mmap_addr = unsafe { 
              libc::mmap(
                0 as *mut libc::c_void,
                Gpio::MMAP_BLOCK_SIZE,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                mmap_fd,
                base_addr as i32
              )
            };
            unsafe { libc::close(mmap_fd); }            
            Ok(GpioError::ErrCode(0))
        }
    }
    pub fn set_pio_mode(&self, pin_no: GpioPin, mode: PioMode)
    {
      let pin_no_ref = pin_no as i32;
      let sel_no = pin_no_ref / 10;
      let offset = (sel_no << 2) as isize;
      let gpfsel: *mut libc::c_uint = unsafe {self.mmap_addr.offset(offset) as *mut libc::c_uint};
      let bit_shift = (pin_no_ref % 10) * 3;

      let write_data = (mode as u32) << bit_shift;
      unsafe{ *gpfsel = write_data; }
    }
    pub fn write_data(&self, pin_no: GpioPin, on_flag: bool)
    {
      let pin_no_ref = pin_no as i32;
      let write_data = 1 << pin_no_ref;
      let offset: isize;

      if let true = on_flag {
        offset = GpfSet::GpfSet0 as isize;
      }
      else {
        offset = GpfClr::GpfClr0 as isize;
      }
      let gpset: *mut libc::c_uint = unsafe {self.mmap_addr.offset(offset) as *mut libc::c_uint};
      unsafe{ *gpset = write_data; }
  }

  #[allow(dead_code)]
  fn open_devmem() -> libc::c_int
  {
    let mut mem_fd: libc::c_int = -1;
  
    if let Ok(filename) = std::ffi::CString::new("/dev/mem") {
        mem_fd = unsafe { 
          libc::open(
            filename.as_ptr(),
            libc::O_RDWR | libc::O_SYNC
          )
        };
        if mem_fd < 0 {
          if let Ok(filename) = std::ffi::CString::new("/dev/gpiomem") {
            mem_fd = unsafe { 
              libc::open(
                filename.as_ptr(),
                libc::O_RDWR | libc::O_SYNC
              )
            };
          }
        }
      }
      mem_fd
  }    
}
