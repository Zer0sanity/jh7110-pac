#[repr(C)]
#[doc = "USB3 OTG registers"]
#[doc(alias = "otg")]
pub struct Otg {
    did: Did,
    rid: Rid,
    capabilities: Capabilities,
    _reserved3: [u8; 0x04],
    cmd: Cmd,
    sts: Sts,
    state: State,
    _reserved6: [u8; 0x04],
    int: [Int; 2],
    refclk: Refclk,
    tmr: Tmr,
    _reserved9: [u8; 0x10],
    simulate: Simulate,
    over: Over,
    susp_ctrl: SuspCtrl,
    phyrst_cfg: PhyrstCfg,
    anasts: Anasts,
    adp_ramp_time: AdpRampTime,
    ctrl: [Ctrl; 2],
}
impl Otg {
    #[doc = "0x00 - USB3 OTG VID."]
    #[inline(always)]
    pub const fn did(&self) -> &Did {
        &self.did
    }
    #[doc = "0x04 - USB3 OTG RID."]
    #[inline(always)]
    pub const fn rid(&self) -> &Rid {
        &self.rid
    }
    #[doc = "0x08 - USB3 OTG capabilities."]
    #[inline(always)]
    pub const fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }
    #[doc = "0x10 - USB3 OTG command."]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x14 - USB3 OTG status."]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x18 - USB3 OTG state."]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x20..0x28 - USB3 OTG interrupt registers - 0: enable, 1: vector status. Write `1` to interrupt vector fields to clear the status."]
    #[inline(always)]
    pub const fn int(&self, n: usize) -> &Int {
        &self.int[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - USB3 OTG interrupt registers - 0: enable, 1: vector status. Write `1` to interrupt vector fields to clear the status."]
    #[inline(always)]
    pub fn int_iter(&self) -> impl Iterator<Item = &Int> {
        self.int.iter()
    }
    #[doc = "0x20 - USB3 OTG interrupt registers - 0: enable, 1: vector status. Write `1` to interrupt vector fields to clear the status."]
    #[inline(always)]
    pub const fn int_en(&self) -> &Int {
        self.int(0)
    }
    #[doc = "0x24 - USB3 OTG interrupt registers - 0: enable, 1: vector status. Write `1` to interrupt vector fields to clear the status."]
    #[inline(always)]
    pub const fn int_vect(&self) -> &Int {
        self.int(1)
    }
    #[doc = "0x28 - USB3 OTG reference clock."]
    #[inline(always)]
    pub const fn refclk(&self) -> &Refclk {
        &self.refclk
    }
    #[doc = "0x2c - USB3 OTG timer."]
    #[inline(always)]
    pub const fn tmr(&self) -> &Tmr {
        &self.tmr
    }
    #[doc = "0x40 - USB3 OTG simulate."]
    #[inline(always)]
    pub const fn simulate(&self) -> &Simulate {
        &self.simulate
    }
    #[doc = "0x44 - USB3 OTG override."]
    #[inline(always)]
    pub const fn over(&self) -> &Over {
        &self.over
    }
    #[doc = "0x48 - USB3 OTG suspend control."]
    #[inline(always)]
    pub const fn susp_ctrl(&self) -> &SuspCtrl {
        &self.susp_ctrl
    }
    #[doc = "0x4c - USB3 OTG PHY reset configuration."]
    #[inline(always)]
    pub const fn phyrst_cfg(&self) -> &PhyrstCfg {
        &self.phyrst_cfg
    }
    #[doc = "0x50 - USB3 OTG ANA status."]
    #[inline(always)]
    pub const fn anasts(&self) -> &Anasts {
        &self.anasts
    }
    #[doc = "0x54 - USB3 OTG ADP ramp time."]
    #[inline(always)]
    pub const fn adp_ramp_time(&self) -> &AdpRampTime {
        &self.adp_ramp_time
    }
    #[doc = "0x58..0x60 - USB3 OTG control registers."]
    #[inline(always)]
    pub const fn ctrl(&self, n: usize) -> &Ctrl {
        &self.ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x60 - USB3 OTG control registers."]
    #[inline(always)]
    pub fn ctrl_iter(&self) -> impl Iterator<Item = &Ctrl> {
        self.ctrl.iter()
    }
    #[doc = "0x58 - USB3 OTG control registers."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl {
        self.ctrl(0)
    }
    #[doc = "0x5c - USB3 OTG control registers."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl {
        self.ctrl(1)
    }
}
#[doc = "did (rw) register accessor: USB3 OTG VID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`did::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`did::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@did`]
module"]
#[doc(alias = "did")]
pub type Did = crate::Reg<did::DidSpec>;
#[doc = "USB3 OTG VID."]
pub mod did;
#[doc = "rid (rw) register accessor: USB3 OTG RID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rid`]
module"]
#[doc(alias = "rid")]
pub type Rid = crate::Reg<rid::RidSpec>;
#[doc = "USB3 OTG RID."]
pub mod rid;
#[doc = "capabilities (rw) register accessor: USB3 OTG capabilities.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capabilities::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities`]
module"]
#[doc(alias = "capabilities")]
pub type Capabilities = crate::Reg<capabilities::CapabilitiesSpec>;
#[doc = "USB3 OTG capabilities."]
pub mod capabilities;
#[doc = "cmd (rw) register accessor: USB3 OTG command.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "cmd")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "USB3 OTG command."]
pub mod cmd;
#[doc = "sts (rw) register accessor: USB3 OTG status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "sts")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "USB3 OTG status."]
pub mod sts;
#[doc = "state (rw) register accessor: USB3 OTG state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
#[doc(alias = "state")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "USB3 OTG state."]
pub mod state;
#[doc = "int (rw) register accessor: USB3 OTG interrupt registers - 0: enable, 1: vector status. Write `1` to interrupt vector fields to clear the status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int`]
module"]
#[doc(alias = "int")]
pub type Int = crate::Reg<int::IntSpec>;
#[doc = "USB3 OTG interrupt registers - 0: enable, 1: vector status. Write `1` to interrupt vector fields to clear the status."]
pub mod int;
#[doc = "refclk (rw) register accessor: USB3 OTG reference clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refclk`]
module"]
#[doc(alias = "refclk")]
pub type Refclk = crate::Reg<refclk::RefclkSpec>;
#[doc = "USB3 OTG reference clock."]
pub mod refclk;
#[doc = "tmr (rw) register accessor: USB3 OTG timer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr`]
module"]
#[doc(alias = "tmr")]
pub type Tmr = crate::Reg<tmr::TmrSpec>;
#[doc = "USB3 OTG timer."]
pub mod tmr;
#[doc = "simulate (rw) register accessor: USB3 OTG simulate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simulate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simulate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simulate`]
module"]
#[doc(alias = "simulate")]
pub type Simulate = crate::Reg<simulate::SimulateSpec>;
#[doc = "USB3 OTG simulate."]
pub mod simulate;
#[doc = "over (rw) register accessor: USB3 OTG override.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@over`]
module"]
#[doc(alias = "over")]
pub type Over = crate::Reg<over::OverSpec>;
#[doc = "USB3 OTG override."]
pub mod over;
#[doc = "susp_ctrl (rw) register accessor: USB3 OTG suspend control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`susp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`susp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@susp_ctrl`]
module"]
#[doc(alias = "susp_ctrl")]
pub type SuspCtrl = crate::Reg<susp_ctrl::SuspCtrlSpec>;
#[doc = "USB3 OTG suspend control."]
pub mod susp_ctrl;
#[doc = "phyrst_cfg (rw) register accessor: USB3 OTG PHY reset configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyrst_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phyrst_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phyrst_cfg`]
module"]
#[doc(alias = "phyrst_cfg")]
pub type PhyrstCfg = crate::Reg<phyrst_cfg::PhyrstCfgSpec>;
#[doc = "USB3 OTG PHY reset configuration."]
pub mod phyrst_cfg;
#[doc = "anasts (rw) register accessor: USB3 OTG ANA status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anasts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anasts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anasts`]
module"]
#[doc(alias = "anasts")]
pub type Anasts = crate::Reg<anasts::AnastsSpec>;
#[doc = "USB3 OTG ANA status."]
pub mod anasts;
#[doc = "adp_ramp_time (rw) register accessor: USB3 OTG ADP ramp time.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adp_ramp_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adp_ramp_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adp_ramp_time`]
module"]
#[doc(alias = "adp_ramp_time")]
pub type AdpRampTime = crate::Reg<adp_ramp_time::AdpRampTimeSpec>;
#[doc = "USB3 OTG ADP ramp time."]
pub mod adp_ramp_time;
#[doc = "ctrl (rw) register accessor: USB3 OTG control registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "ctrl")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "USB3 OTG control registers."]
pub mod ctrl;
