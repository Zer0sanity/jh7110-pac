#[repr(C)]
#[doc = "MTL RXP registers"]
#[doc(alias = "rxp")]
pub struct Rxp {
    rxp_ctrl_status: RxpCtrlStatus,
    _reserved1: [u8; 0x0c],
    iacc: Iacc,
}
impl Rxp {
    #[doc = "0x00 - MTL RXP Control and Status"]
    #[inline(always)]
    pub const fn rxp_ctrl_status(&self) -> &RxpCtrlStatus {
        &self.rxp_ctrl_status
    }
    #[doc = "0x10..0x18 - MTL RXP IACC registers"]
    #[inline(always)]
    pub const fn iacc(&self) -> &Iacc {
        &self.iacc
    }
}
#[doc = "rxp_ctrl_status (rw) register accessor: MTL RXP Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxp_ctrl_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxp_ctrl_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxp_ctrl_status`]
module"]
#[doc(alias = "rxp_ctrl_status")]
pub type RxpCtrlStatus = crate::Reg<rxp_ctrl_status::RxpCtrlStatusSpec>;
#[doc = "MTL RXP Control and Status"]
pub mod rxp_ctrl_status;
#[doc = "MTL RXP IACC registers"]
pub use self::iacc::Iacc;
#[doc = r"Cluster"]
#[doc = "MTL RXP IACC registers"]
pub mod iacc;
