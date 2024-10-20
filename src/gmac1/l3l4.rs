#[repr(C)]
#[doc = "MAC L3/L4 Filter registers"]
#[doc(alias = "l3l4")]
pub struct L3l4 {
    l3l4_ctrl: L3l4Ctrl,
    l4_addr: L4Addr,
    _reserved2: [u8; 0x08],
    l3_addr: [L3Addr; 2],
    _reserved3: [u8; 0x14],
    _reserved_l3l4: _ReservedL3l4,
}
impl L3l4 {
    #[doc = "0x00 - L3/L4 Filter Control"]
    #[inline(always)]
    pub const fn l3l4_ctrl(&self) -> &L3l4Ctrl {
        &self.l3l4_ctrl
    }
    #[doc = "0x04 - L4 Filter Address"]
    #[inline(always)]
    pub const fn l4_addr(&self) -> &L4Addr {
        &self.l4_addr
    }
    #[doc = "0x10..0x18 - L3 Filter Address"]
    #[inline(always)]
    pub const fn l3_addr(&self, n: usize) -> &L3Addr {
        &self.l3_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - L3 Filter Address"]
    #[inline(always)]
    pub fn l3_addr_iter(&self) -> impl Iterator<Item = &L3Addr> {
        self.l3_addr.iter()
    }
    #[doc = "0x2c - Reserved"]
    #[inline(always)]
    pub const fn _reserved_l3l4(&self) -> &_ReservedL3l4 {
        &self._reserved_l3l4
    }
}
#[doc = "l3l4_ctrl (rw) register accessor: L3/L4 Filter Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l3l4_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l3l4_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l3l4_ctrl`]
module"]
#[doc(alias = "l3l4_ctrl")]
pub type L3l4Ctrl = crate::Reg<l3l4_ctrl::L3l4CtrlSpec>;
#[doc = "L3/L4 Filter Control"]
pub mod l3l4_ctrl;
#[doc = "l4_addr (rw) register accessor: L4 Filter Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l4_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l4_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l4_addr`]
module"]
#[doc(alias = "l4_addr")]
pub type L4Addr = crate::Reg<l4_addr::L4AddrSpec>;
#[doc = "L4 Filter Address"]
pub mod l4_addr;
#[doc = "l3_addr (rw) register accessor: L3 Filter Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@l3_addr`]
module"]
#[doc(alias = "l3_addr")]
pub type L3Addr = crate::Reg<l3_addr::L3AddrSpec>;
#[doc = "L3 Filter Address"]
pub mod l3_addr;
#[doc = "_reserved_l3l4 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_reserved_l3l4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_reserved_l3l4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_reserved_l3l4`]
module"]
#[doc(alias = "_reserved_l3l4")]
pub type _ReservedL3l4 = crate::Reg<_reserved_l3l4::_ReservedL3l4Spec>;
#[doc = "Reserved"]
pub mod _reserved_l3l4;
