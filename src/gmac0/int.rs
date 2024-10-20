#[repr(C)]
#[doc = "MAC Interrupt registers"]
#[doc(alias = "int")]
pub struct Int {
    status: Status,
    enable: Enable,
}
impl Int {
    #[doc = "0x00 - MAC Interrupt Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - MAC Interrupt Enable"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
}
#[doc = "status (r) register accessor: MAC Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "MAC Interrupt Status"]
pub mod status;
#[doc = "enable (rw) register accessor: MAC Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "enable")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "MAC Interrupt Enable"]
pub mod enable;
