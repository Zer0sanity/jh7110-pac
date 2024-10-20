#[repr(C)]
#[doc = "MTL ECC Interrupt registers"]
#[doc(alias = "ecc_int")]
pub struct EccInt {
    enable: Enable,
    status: Status,
}
impl EccInt {
    #[doc = "0x00 - MTL ECC Interrupt Enable"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x04 - MTL ECC Interrupt Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "enable (rw) register accessor: MTL ECC Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "enable")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "MTL ECC Interrupt Enable"]
pub mod enable;
#[doc = "status (rw) register accessor: MTL ECC Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "MTL ECC Interrupt Status"]
pub mod status;
