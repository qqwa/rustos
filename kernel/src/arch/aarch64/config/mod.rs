#[cfg(device = "imx8")]
mod imx8;
#[cfg(device = "imx8")]
pub use self::imx8::*;

#[cfg(device = "qemu-virt")]
mod qemu_virt;
#[cfg(device = "qemu-virt")]
pub use self::qemu_virt::*;
