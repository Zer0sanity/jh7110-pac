#[repr(C)]
#[doc = "PCS (AN/TBI/SGMII/RGMII) registers"]
#[doc(alias = "pcs")]
pub struct Pcs {
    an_ctrl: AnCtrl,
    an_status: AnStatus,
    ane: [Ane; 2],
    ane_exp: AneExp,
    tbi: Tbi,
}
impl Pcs {
    #[doc = "0x00 - Auto-Negotiation Control"]
    #[inline(always)]
    pub const fn an_ctrl(&self) -> &AnCtrl {
        &self.an_ctrl
    }
    #[doc = "0x04 - Auto-Negotiation Status"]
    #[inline(always)]
    pub const fn an_status(&self) -> &AnStatus {
        &self.an_status
    }
    #[doc = "0x08..0x10 - Auto-Negotiation Extend Advertisement and Link Partner Ability"]
    #[inline(always)]
    pub const fn ane(&self, n: usize) -> &Ane {
        &self.ane[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x10 - Auto-Negotiation Extend Advertisement and Link Partner Ability"]
    #[inline(always)]
    pub fn ane_iter(&self) -> impl Iterator<Item = &Ane> {
        self.ane.iter()
    }
    #[doc = "0x08 - Auto-Negotiation Extend Advertisement and Link Partner Ability"]
    #[inline(always)]
    pub const fn ane_adv(&self) -> &Ane {
        self.ane(0)
    }
    #[doc = "0x0c - Auto-Negotiation Extend Advertisement and Link Partner Ability"]
    #[inline(always)]
    pub const fn ane_lpa(&self) -> &Ane {
        self.ane(1)
    }
    #[doc = "0x10 - Auto-Negotiation Extend Expansion"]
    #[inline(always)]
    pub const fn ane_exp(&self) -> &AneExp {
        &self.ane_exp
    }
    #[doc = "0x14 - TBI Extend Status"]
    #[inline(always)]
    pub const fn tbi(&self) -> &Tbi {
        &self.tbi
    }
}
#[doc = "an_ctrl (rw) register accessor: Auto-Negotiation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`an_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@an_ctrl`]
module"]
#[doc(alias = "an_ctrl")]
pub type AnCtrl = crate::Reg<an_ctrl::AnCtrlSpec>;
#[doc = "Auto-Negotiation Control"]
pub mod an_ctrl;
#[doc = "an_status (rw) register accessor: Auto-Negotiation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`an_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@an_status`]
module"]
#[doc(alias = "an_status")]
pub type AnStatus = crate::Reg<an_status::AnStatusSpec>;
#[doc = "Auto-Negotiation Status"]
pub mod an_status;
#[doc = "ane (rw) register accessor: Auto-Negotiation Extend Advertisement and Link Partner Ability\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ane::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ane::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ane`]
module"]
#[doc(alias = "ane")]
pub type Ane = crate::Reg<ane::AneSpec>;
#[doc = "Auto-Negotiation Extend Advertisement and Link Partner Ability"]
pub mod ane;
#[doc = "ane_exp (rw) register accessor: Auto-Negotiation Extend Expansion\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ane_exp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ane_exp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ane_exp`]
module"]
#[doc(alias = "ane_exp")]
pub type AneExp = crate::Reg<ane_exp::AneExpSpec>;
#[doc = "Auto-Negotiation Extend Expansion"]
pub mod ane_exp;
#[doc = "tbi (rw) register accessor: TBI Extend Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbi`]
module"]
#[doc(alias = "tbi")]
pub type Tbi = crate::Reg<tbi::TbiSpec>;
#[doc = "TBI Extend Status"]
pub mod tbi;
