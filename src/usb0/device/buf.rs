#[repr(C)]
#[doc = "USB3 On-chip buffer registers."]
#[doc(alias = "buf")]
pub struct Buf {
    addr: Addr,
    data: Data,
    ctrl: Ctrl,
}
impl Buf {
    #[doc = "0x00 - USB3 On-chip buffer address."]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x04 - USB3 On-chip buffer data."]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x08 - USB3 On-chip buffer control."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "addr (rw) register accessor: USB3 On-chip buffer address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "addr")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "USB3 On-chip buffer address."]
pub mod addr;
#[doc = "data (rw) register accessor: USB3 On-chip buffer data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "USB3 On-chip buffer data."]
pub mod data;
#[doc = "ctrl (rw) register accessor: USB3 On-chip buffer control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "ctrl")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "USB3 On-chip buffer control."]
pub mod ctrl;
