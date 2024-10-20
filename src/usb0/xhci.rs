#[repr(C)]
#[doc = "USB3 XHCI registers"]
#[doc(alias = "xhci")]
pub struct Xhci {
    cap: Cap,
}
impl Xhci {
    #[doc = "0x00..0x20 - USB3 XHCI Capability registers."]
    #[inline(always)]
    pub const fn cap(&self) -> &Cap {
        &self.cap
    }
}
#[doc = "USB3 XHCI Capability registers."]
pub use self::cap::Cap;
#[doc = r"Cluster"]
#[doc = "USB3 XHCI Capability registers."]
pub mod cap;
