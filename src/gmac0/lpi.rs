#[repr(C)]
#[doc = "MAC LPI Energy Efficient Ethernet (EEE) registers"]
#[doc(alias = "lpi")]
pub struct Lpi {
    ctrl_status: CtrlStatus,
    timer_ctrl: TimerCtrl,
    entry_timer: EntryTimer,
}
impl Lpi {
    #[doc = "0x00 - MAC LPI Control and Status"]
    #[inline(always)]
    pub const fn ctrl_status(&self) -> &CtrlStatus {
        &self.ctrl_status
    }
    #[doc = "0x04 - MAC LPI Timer Control"]
    #[inline(always)]
    pub const fn timer_ctrl(&self) -> &TimerCtrl {
        &self.timer_ctrl
    }
    #[doc = "0x08 - MAC LPI Entry Timer"]
    #[inline(always)]
    pub const fn entry_timer(&self) -> &EntryTimer {
        &self.entry_timer
    }
}
#[doc = "ctrl_status (rw) register accessor: MAC LPI Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_status`]
module"]
#[doc(alias = "ctrl_status")]
pub type CtrlStatus = crate::Reg<ctrl_status::CtrlStatusSpec>;
#[doc = "MAC LPI Control and Status"]
pub mod ctrl_status;
#[doc = "timer_ctrl (rw) register accessor: MAC LPI Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ctrl`]
module"]
#[doc(alias = "timer_ctrl")]
pub type TimerCtrl = crate::Reg<timer_ctrl::TimerCtrlSpec>;
#[doc = "MAC LPI Timer Control"]
pub mod timer_ctrl;
#[doc = "entry_timer (rw) register accessor: MAC LPI Entry Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry_timer`]
module"]
#[doc(alias = "entry_timer")]
pub type EntryTimer = crate::Reg<entry_timer::EntryTimerSpec>;
#[doc = "MAC LPI Entry Timer"]
pub mod entry_timer;
