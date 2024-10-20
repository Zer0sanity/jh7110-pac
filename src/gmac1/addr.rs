#[repr(C)]
#[doc = "Hardware Address registers"]
#[doc(alias = "addr")]
pub struct Addr {
    high: High,
    low: Low,
}
impl Addr {
    #[doc = "0x00 - Hardware Address High"]
    #[inline(always)]
    pub const fn high(&self) -> &High {
        &self.high
    }
    #[doc = "0x04 - Hardware Address Low"]
    #[inline(always)]
    pub const fn low(&self) -> &Low {
        &self.low
    }
}
#[doc = "high (rw) register accessor: Hardware Address High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high`]
module"]
#[doc(alias = "high")]
pub type High = crate::Reg<high::HighSpec>;
#[doc = "Hardware Address High"]
pub mod high;
#[doc = "low (rw) register accessor: Hardware Address Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low`]
module"]
#[doc(alias = "low")]
pub type Low = crate::Reg<low::LowSpec>;
#[doc = "Hardware Address Low"]
pub mod low;
