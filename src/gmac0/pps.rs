#[repr(C)]
#[doc = "PPS registers"]
#[doc(alias = "pps")]
pub struct Pps {
    target_time_sec: TargetTimeSec,
    target_time_nsec: TargetTimeNsec,
    interval: Interval,
    width: Width,
}
impl Pps {
    #[doc = "0x00 - PPS Target Time - Seconds"]
    #[inline(always)]
    pub const fn target_time_sec(&self) -> &TargetTimeSec {
        &self.target_time_sec
    }
    #[doc = "0x04 - PPS Target Time - Nanoseconds"]
    #[inline(always)]
    pub const fn target_time_nsec(&self) -> &TargetTimeNsec {
        &self.target_time_nsec
    }
    #[doc = "0x08 - PPS Interval"]
    #[inline(always)]
    pub const fn interval(&self) -> &Interval {
        &self.interval
    }
    #[doc = "0x0c - PPS Width"]
    #[inline(always)]
    pub const fn width(&self) -> &Width {
        &self.width
    }
}
#[doc = "target_time_sec (rw) register accessor: PPS Target Time - Seconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_sec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_sec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_time_sec`]
module"]
#[doc(alias = "target_time_sec")]
pub type TargetTimeSec = crate::Reg<target_time_sec::TargetTimeSecSpec>;
#[doc = "PPS Target Time - Seconds"]
pub mod target_time_sec;
#[doc = "target_time_nsec (rw) register accessor: PPS Target Time - Nanoseconds\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target_time_nsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_time_nsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_time_nsec`]
module"]
#[doc(alias = "target_time_nsec")]
pub type TargetTimeNsec = crate::Reg<target_time_nsec::TargetTimeNsecSpec>;
#[doc = "PPS Target Time - Nanoseconds"]
pub mod target_time_nsec;
#[doc = "interval (rw) register accessor: PPS Interval\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interval`]
module"]
#[doc(alias = "interval")]
pub type Interval = crate::Reg<interval::IntervalSpec>;
#[doc = "PPS Interval"]
pub mod interval;
#[doc = "width (rw) register accessor: PPS Width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`width::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`width::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@width`]
module"]
#[doc(alias = "width")]
pub type Width = crate::Reg<width::WidthSpec>;
#[doc = "PPS Width"]
pub mod width;
