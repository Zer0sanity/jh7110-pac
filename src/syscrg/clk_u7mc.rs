#[repr(C)]
#[doc = "Clock U7MC registers"]
#[doc(alias = "clk_u7mc")]
pub struct ClkU7mc {
    core: [Core; 5],
    debug: Debug,
    rtc_toggle: RtcToggle,
    trace: [Trace; 5],
    trace_com: TraceCom,
}
impl ClkU7mc {
    #[doc = "0x00..0x14 - Clock U7MC Core"]
    #[inline(always)]
    pub const fn core(&self, n: usize) -> &Core {
        &self.core[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x14 - Clock U7MC Core"]
    #[inline(always)]
    pub fn core_iter(&self) -> impl Iterator<Item = &Core> {
        self.core.iter()
    }
    #[doc = "0x14 - Clock U7MC Debug"]
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
    #[doc = "0x18 - Clock U7MC RTC Toggle"]
    #[inline(always)]
    pub const fn rtc_toggle(&self) -> &RtcToggle {
        &self.rtc_toggle
    }
    #[doc = "0x1c..0x30 - Clock U7MC Trace"]
    #[inline(always)]
    pub const fn trace(&self, n: usize) -> &Trace {
        &self.trace[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x30 - Clock U7MC Trace"]
    #[inline(always)]
    pub fn trace_iter(&self) -> impl Iterator<Item = &Trace> {
        self.trace.iter()
    }
    #[doc = "0x30 - Clock U7MC Trace"]
    #[inline(always)]
    pub const fn trace_com(&self) -> &TraceCom {
        &self.trace_com
    }
}
#[doc = "core (rw) register accessor: Clock U7MC Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core`]
module"]
#[doc(alias = "core")]
pub type Core = crate::Reg<core::CoreSpec>;
#[doc = "Clock U7MC Core"]
pub mod core;
#[doc = "debug (rw) register accessor: Clock U7MC Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`]
module"]
#[doc(alias = "debug")]
pub type Debug = crate::Reg<debug::DebugSpec>;
#[doc = "Clock U7MC Debug"]
pub mod debug;
#[doc = "rtc_toggle (rw) register accessor: Clock U7MC RTC Toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_toggle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_toggle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_toggle`]
module"]
#[doc(alias = "rtc_toggle")]
pub type RtcToggle = crate::Reg<rtc_toggle::RtcToggleSpec>;
#[doc = "Clock U7MC RTC Toggle"]
pub mod rtc_toggle;
#[doc = "trace (rw) register accessor: Clock U7MC Trace\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trace::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trace::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace`]
module"]
#[doc(alias = "trace")]
pub type Trace = crate::Reg<trace::TraceSpec>;
#[doc = "Clock U7MC Trace"]
pub mod trace;
#[doc = "trace_com (rw) register accessor: Clock U7MC Trace\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trace_com::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trace_com::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trace_com`]
module"]
#[doc(alias = "trace_com")]
pub type TraceCom = crate::Reg<trace_com::TraceComSpec>;
#[doc = "Clock U7MC Trace"]
pub mod trace_com;
