#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    otg: Otg,
    _reserved1: [u8; 0xffa0],
    xhci: Xhci,
    _reserved2: [u8; 0xffe0],
    device: Device,
}
impl RegisterBlock {
    #[doc = "0x00..0x60 - USB3 OTG registers"]
    #[inline(always)]
    pub const fn otg(&self) -> &Otg {
        &self.otg
    }
    #[doc = "0x10000..0x10020 - USB3 XHCI registers"]
    #[inline(always)]
    pub const fn xhci(&self) -> &Xhci {
        &self.xhci
    }
    #[doc = "0x20000..0x20314 - USB3 device registers"]
    #[inline(always)]
    pub const fn device(&self) -> &Device {
        &self.device
    }
}
#[doc = "USB3 OTG registers"]
pub use self::otg::Otg;
#[doc = r"Cluster"]
#[doc = "USB3 OTG registers"]
pub mod otg;
#[doc = "USB3 XHCI registers"]
pub use self::xhci::Xhci;
#[doc = r"Cluster"]
#[doc = "USB3 XHCI registers"]
pub mod xhci;
#[doc = "USB3 device registers"]
pub use self::device::Device;
#[doc = r"Cluster"]
#[doc = "USB3 device registers"]
pub mod device;
