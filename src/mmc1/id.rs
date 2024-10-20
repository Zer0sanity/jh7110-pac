#[repr(C)]
#[doc = "MMC ID registers"]
#[doc(alias = "id")]
pub struct Id {
    usrid: Usrid,
    verid: Verid,
}
impl Id {
    #[doc = "0x00 - MMC user ID"]
    #[inline(always)]
    pub const fn usrid(&self) -> &Usrid {
        &self.usrid
    }
    #[doc = "0x04 - MMC version ID"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
}
#[doc = "usrid (rw) register accessor: MMC user ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usrid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usrid`]
module"]
#[doc(alias = "usrid")]
pub type Usrid = crate::Reg<usrid::UsridSpec>;
#[doc = "MMC user ID"]
pub mod usrid;
#[doc = "verid (rw) register accessor: MMC version ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`verid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`]
module"]
#[doc(alias = "verid")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "MMC version ID"]
pub mod verid;
