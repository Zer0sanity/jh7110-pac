#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    hard_event_turn_on_mask: HardEventTurnOnMask,
    _reserved1: [u8; 0x04],
    soft_turn_on_power_mode: SoftTurnOnPowerMode,
    soft_turn_off_power_mode: SoftTurnOffPowerMode,
    timeout_seq_thd: TimeoutSeqThd,
    pdc: [Pdc; 3],
    _reserved5: [u8; 0x20],
    sw_encourage: SwEncourage,
    tim: Tim,
    pch_bypass: PchBypass,
    pch_pstate: PchPstate,
    pch_timeout: PchTimeout,
    lp_timeout: LpTimeout,
    hard_turn_on_power_mode: HardTurnOnPowerMode,
    _reserved12: [u8; 0x20],
    current_power_mode: CurrentPowerMode,
    _reserved13: [u8; 0x04],
    status: [Status; 2],
    hw_event_crd: HwEventCrd,
    encourage_type_crd: EncourageTypeCrd,
    pch_active: PchActive,
}
impl RegisterBlock {
    #[doc = "0x04 - Hardware Event Turn-On Mask"]
    #[inline(always)]
    pub const fn hard_event_turn_on_mask(&self) -> &HardEventTurnOnMask {
        &self.hard_event_turn_on_mask
    }
    #[doc = "0x0c - Software Turn-On Power Mode"]
    #[inline(always)]
    pub const fn soft_turn_on_power_mode(&self) -> &SoftTurnOnPowerMode {
        &self.soft_turn_on_power_mode
    }
    #[doc = "0x10 - Software Turn-Off Power Mode"]
    #[inline(always)]
    pub const fn soft_turn_off_power_mode(&self) -> &SoftTurnOffPowerMode {
        &self.soft_turn_off_power_mode
    }
    #[doc = "0x14 - Timeout Sequence Threshold"]
    #[inline(always)]
    pub const fn timeout_seq_thd(&self) -> &TimeoutSeqThd {
        &self.timeout_seq_thd
    }
    #[doc = "0x18..0x24 - Power Domain Cascade register"]
    #[inline(always)]
    pub const fn pdc(&self, n: usize) -> &Pdc {
        &self.pdc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x24 - Power Domain Cascade register"]
    #[inline(always)]
    pub fn pdc_iter(&self) -> impl Iterator<Item = &Pdc> {
        self.pdc.iter()
    }
    #[doc = "0x44 - Software encouragement register"]
    #[inline(always)]
    pub const fn sw_encourage(&self) -> &SwEncourage {
        &self.sw_encourage
    }
    #[doc = "0x48 - Timer Interrupt Mask register"]
    #[inline(always)]
    pub const fn tim(&self) -> &Tim {
        &self.tim
    }
    #[doc = "0x4c - P-channel Bypass register"]
    #[inline(always)]
    pub const fn pch_bypass(&self) -> &PchBypass {
        &self.pch_bypass
    }
    #[doc = "0x50 - P-channel PSTATE register"]
    #[inline(always)]
    pub const fn pch_pstate(&self) -> &PchPstate {
        &self.pch_pstate
    }
    #[doc = "0x54 - P-channel waiting device acknowledge timeout."]
    #[inline(always)]
    pub const fn pch_timeout(&self) -> &PchTimeout {
        &self.pch_timeout
    }
    #[doc = "0x58 - LP Cell Control Timeout Threshold register"]
    #[inline(always)]
    pub const fn lp_timeout(&self) -> &LpTimeout {
        &self.lp_timeout
    }
    #[doc = "0x5c - Hardware Turn-On Power Mode register"]
    #[inline(always)]
    pub const fn hard_turn_on_power_mode(&self) -> &HardTurnOnPowerMode {
        &self.hard_turn_on_power_mode
    }
    #[doc = "0x80 - Current Power Mode register"]
    #[inline(always)]
    pub const fn current_power_mode(&self) -> &CurrentPowerMode {
        &self.current_power_mode
    }
    #[doc = "0x88..0x90 - Event and Interrupt Status registers"]
    #[inline(always)]
    pub const fn status(&self, n: usize) -> &Status {
        &self.status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x90 - Event and Interrupt Status registers"]
    #[inline(always)]
    pub fn status_iter(&self) -> impl Iterator<Item = &Status> {
        self.status.iter()
    }
    #[doc = "0x88 - Event and Interrupt Status registers"]
    #[inline(always)]
    pub const fn status_event(&self) -> &Status {
        self.status(0)
    }
    #[doc = "0x8c - Event and Interrupt Status registers"]
    #[inline(always)]
    pub const fn status_interrupt(&self) -> &Status {
        self.status(1)
    }
    #[doc = "0x90 - Hardware Event Record register"]
    #[inline(always)]
    pub const fn hw_event_crd(&self) -> &HwEventCrd {
        &self.hw_event_crd
    }
    #[doc = "0x94 - Hardware Event Type Record register"]
    #[inline(always)]
    pub const fn encourage_type_crd(&self) -> &EncourageTypeCrd {
        &self.encourage_type_crd
    }
    #[doc = "0x98 - P-channel PACTIVE Status register"]
    #[inline(always)]
    pub const fn pch_active(&self) -> &PchActive {
        &self.pch_active
    }
}
#[doc = "hard_event_turn_on_mask (rw) register accessor: Hardware Event Turn-On Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hard_event_turn_on_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hard_event_turn_on_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hard_event_turn_on_mask`]
module"]
#[doc(alias = "hard_event_turn_on_mask")]
pub type HardEventTurnOnMask = crate::Reg<hard_event_turn_on_mask::HardEventTurnOnMaskSpec>;
#[doc = "Hardware Event Turn-On Mask"]
pub mod hard_event_turn_on_mask;
#[doc = "soft_turn_on_power_mode (rw) register accessor: Software Turn-On Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_turn_on_power_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_turn_on_power_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_turn_on_power_mode`]
module"]
#[doc(alias = "soft_turn_on_power_mode")]
pub type SoftTurnOnPowerMode = crate::Reg<soft_turn_on_power_mode::SoftTurnOnPowerModeSpec>;
#[doc = "Software Turn-On Power Mode"]
pub mod soft_turn_on_power_mode;
#[doc = "soft_turn_off_power_mode (rw) register accessor: Software Turn-Off Power Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_turn_off_power_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_turn_off_power_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_turn_off_power_mode`]
module"]
#[doc(alias = "soft_turn_off_power_mode")]
pub type SoftTurnOffPowerMode = crate::Reg<soft_turn_off_power_mode::SoftTurnOffPowerModeSpec>;
#[doc = "Software Turn-Off Power Mode"]
pub mod soft_turn_off_power_mode;
#[doc = "timeout_seq_thd (rw) register accessor: Timeout Sequence Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout_seq_thd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_seq_thd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_seq_thd`]
module"]
#[doc(alias = "timeout_seq_thd")]
pub type TimeoutSeqThd = crate::Reg<timeout_seq_thd::TimeoutSeqThdSpec>;
#[doc = "Timeout Sequence Threshold"]
pub mod timeout_seq_thd;
#[doc = "pdc (rw) register accessor: Power Domain Cascade register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdc`]
module"]
#[doc(alias = "pdc")]
pub type Pdc = crate::Reg<pdc::PdcSpec>;
#[doc = "Power Domain Cascade register"]
pub mod pdc;
#[doc = "sw_encourage (rw) register accessor: Software encouragement register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_encourage::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_encourage::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_encourage`]
module"]
#[doc(alias = "sw_encourage")]
pub type SwEncourage = crate::Reg<sw_encourage::SwEncourageSpec>;
#[doc = "Software encouragement register"]
pub mod sw_encourage;
#[doc = "tim (rw) register accessor: Timer Interrupt Mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim`]
module"]
#[doc(alias = "tim")]
pub type Tim = crate::Reg<tim::TimSpec>;
#[doc = "Timer Interrupt Mask register"]
pub mod tim;
#[doc = "pch_bypass (rw) register accessor: P-channel Bypass register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pch_bypass`]
module"]
#[doc(alias = "pch_bypass")]
pub type PchBypass = crate::Reg<pch_bypass::PchBypassSpec>;
#[doc = "P-channel Bypass register"]
pub mod pch_bypass;
#[doc = "pch_pstate (rw) register accessor: P-channel PSTATE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_pstate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_pstate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pch_pstate`]
module"]
#[doc(alias = "pch_pstate")]
pub type PchPstate = crate::Reg<pch_pstate::PchPstateSpec>;
#[doc = "P-channel PSTATE register"]
pub mod pch_pstate;
#[doc = "pch_timeout (rw) register accessor: P-channel waiting device acknowledge timeout.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pch_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pch_timeout`]
module"]
#[doc(alias = "pch_timeout")]
pub type PchTimeout = crate::Reg<pch_timeout::PchTimeoutSpec>;
#[doc = "P-channel waiting device acknowledge timeout."]
pub mod pch_timeout;
#[doc = "lp_timeout (rw) register accessor: LP Cell Control Timeout Threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_timeout`]
module"]
#[doc(alias = "lp_timeout")]
pub type LpTimeout = crate::Reg<lp_timeout::LpTimeoutSpec>;
#[doc = "LP Cell Control Timeout Threshold register"]
pub mod lp_timeout;
#[doc = "hard_turn_on_power_mode (rw) register accessor: Hardware Turn-On Power Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hard_turn_on_power_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hard_turn_on_power_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hard_turn_on_power_mode`]
module"]
#[doc(alias = "hard_turn_on_power_mode")]
pub type HardTurnOnPowerMode = crate::Reg<hard_turn_on_power_mode::HardTurnOnPowerModeSpec>;
#[doc = "Hardware Turn-On Power Mode register"]
pub mod hard_turn_on_power_mode;
#[doc = "current_power_mode (rw) register accessor: Current Power Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current_power_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`current_power_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_power_mode`]
module"]
#[doc(alias = "current_power_mode")]
pub type CurrentPowerMode = crate::Reg<current_power_mode::CurrentPowerModeSpec>;
#[doc = "Current Power Mode register"]
pub mod current_power_mode;
#[doc = "status (r) register accessor: Event and Interrupt Status registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Event and Interrupt Status registers"]
pub mod status;
#[doc = "hw_event_crd (r) register accessor: Hardware Event Record register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_event_crd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_event_crd`]
module"]
#[doc(alias = "hw_event_crd")]
pub type HwEventCrd = crate::Reg<hw_event_crd::HwEventCrdSpec>;
#[doc = "Hardware Event Record register"]
pub mod hw_event_crd;
#[doc = "encourage_type_crd (r) register accessor: Hardware Event Type Record register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`encourage_type_crd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@encourage_type_crd`]
module"]
#[doc(alias = "encourage_type_crd")]
pub type EncourageTypeCrd = crate::Reg<encourage_type_crd::EncourageTypeCrdSpec>;
#[doc = "Hardware Event Type Record register"]
pub mod encourage_type_crd;
#[doc = "pch_active (r) register accessor: P-channel PACTIVE Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pch_active::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pch_active`]
module"]
#[doc(alias = "pch_active")]
pub type PchActive = crate::Reg<pch_active::PchActiveSpec>;
#[doc = "P-channel PACTIVE Status register"]
pub mod pch_active;
