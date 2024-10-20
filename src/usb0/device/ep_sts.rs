#[repr(C)]
#[doc = "USB3 Endpoint status registers."]
#[doc(alias = "ep_sts")]
pub struct EpSts {
    status: Status,
    sid: Sid,
    en: En,
}
impl EpSts {
    #[doc = "0x00 - USB3 Endpoint status."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - Endpoint status stream ID - used only in SS mode."]
    #[inline(always)]
    pub const fn sid(&self) -> &Sid {
        &self.sid
    }
    #[doc = "0x08 - Endpoint status enable."]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
}
#[doc = "status (rw) register accessor: USB3 Endpoint status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "USB3 Endpoint status."]
pub mod status;
#[doc = "sid (rw) register accessor: Endpoint status stream ID - used only in SS mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sid`]
module"]
#[doc(alias = "sid")]
pub type Sid = crate::Reg<sid::SidSpec>;
#[doc = "Endpoint status stream ID - used only in SS mode."]
pub mod sid;
#[doc = "en (rw) register accessor: Endpoint status enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`]
module"]
#[doc(alias = "en")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "Endpoint status enable."]
pub mod en;
