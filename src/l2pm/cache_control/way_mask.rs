#[repr(C)]
#[doc = "L2 Cache Control Way Mask registers. Configures the masks to enable cache bank ways."]
#[doc(alias = "way_mask")]
pub struct WayMask {
    bank: Bank,
    _way_mask_reserved: _WayMaskReserved,
}
impl WayMask {
    #[doc = "0x00 - L2 Cache Control Way Mask bank registers. Configures the masks to enable cache bank ways."]
    #[inline(always)]
    pub const fn bank(&self) -> &Bank {
        &self.bank
    }
    #[doc = "0x04 - L2 Cache Control Way Mask reserved register."]
    #[inline(always)]
    pub const fn _way_mask_reserved(&self) -> &_WayMaskReserved {
        &self._way_mask_reserved
    }
}
#[doc = "bank (rw) register accessor: L2 Cache Control Way Mask bank registers. Configures the masks to enable cache bank ways.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank`]
module"]
#[doc(alias = "bank")]
pub type Bank = crate::Reg<bank::BankSpec>;
#[doc = "L2 Cache Control Way Mask bank registers. Configures the masks to enable cache bank ways."]
pub mod bank;
#[doc = "_way_mask_reserved (rw) register accessor: L2 Cache Control Way Mask reserved register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_way_mask_reserved::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_way_mask_reserved::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_way_mask_reserved`]
module"]
#[doc(alias = "_way_mask_reserved")]
pub type _WayMaskReserved = crate::Reg<_way_mask_reserved::_WayMaskReservedSpec>;
#[doc = "L2 Cache Control Way Mask reserved register."]
pub mod _way_mask_reserved;
