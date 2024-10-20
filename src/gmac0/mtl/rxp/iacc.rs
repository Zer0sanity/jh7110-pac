#[repr(C)]
#[doc = "MTL RXP IACC registers"]
#[doc(alias = "iacc")]
pub struct Iacc {
    ctrl_status: CtrlStatus,
    data: Data,
}
impl Iacc {
    #[doc = "0x00 - MTL RXP IACC Control and Status"]
    #[inline(always)]
    pub const fn ctrl_status(&self) -> &CtrlStatus {
        &self.ctrl_status
    }
    #[doc = "0x04 - MTL RXP IACC Data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "ctrl_status (rw) register accessor: MTL RXP IACC Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_status`]
module"]
#[doc(alias = "ctrl_status")]
pub type CtrlStatus = crate::Reg<ctrl_status::CtrlStatusSpec>;
#[doc = "MTL RXP IACC Control and Status"]
pub mod ctrl_status;
#[doc = "data (rw) register accessor: MTL RXP IACC Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "MTL RXP IACC Data"]
pub mod data;
