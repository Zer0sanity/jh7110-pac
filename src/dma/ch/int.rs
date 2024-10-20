#[repr(C)]
#[doc = "Channel Interrupt registers."]
#[doc(alias = "int")]
pub struct Int {
    status_enable: StatusEnable,
    status: Status,
    signal_enable: SignalEnable,
    clear: Clear,
}
impl Int {
    #[doc = "0x00..0x08 - Channel Interrupt Status Enable"]
    #[inline(always)]
    pub const fn status_enable(&self) -> &StatusEnable {
        &self.status_enable
    }
    #[doc = "0x08..0x10 - Channel Interrupt Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10..0x18 - Channel Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn signal_enable(&self) -> &SignalEnable {
        &self.signal_enable
    }
    #[doc = "0x18..0x20 - Channel Interrupt Clear"]
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
}
#[doc = "status_enable (rw) register accessor: Channel Interrupt Status Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_enable`]
module"]
#[doc(alias = "status_enable")]
pub type StatusEnable = crate::Reg<status_enable::StatusEnableSpec>;
#[doc = "Channel Interrupt Status Enable"]
pub mod status_enable;
#[doc = "status (r) register accessor: Channel Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Channel Interrupt Status"]
pub mod status;
#[doc = "signal_enable (rw) register accessor: Channel Interrupt Signal Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`signal_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`signal_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@signal_enable`]
module"]
#[doc(alias = "signal_enable")]
pub type SignalEnable = crate::Reg<signal_enable::SignalEnableSpec>;
#[doc = "Channel Interrupt Signal Enable"]
pub mod signal_enable;
#[doc = "clear (w) register accessor: Channel Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`]
module"]
#[doc(alias = "clear")]
pub type Clear = crate::Reg<clear::ClearSpec>;
#[doc = "Channel Interrupt Clear"]
pub mod clear;
