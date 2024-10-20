#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    stat: Stat,
    mode: Mode,
    smode: Smode,
    ie: Ie,
    istat: Istat,
    _reserved6: [u8; 0x08],
    rand: [Rand; 8],
    _reserved7: [u8; 0x20],
    auto_rqsts: AutoRqsts,
    auto_age: AutoAge,
}
impl RegisterBlock {
    #[doc = "0x00 - TRNG Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - TRNG STAT Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - TRNG MODE Register"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x0c - TRNG SMODE Register"]
    #[inline(always)]
    pub const fn smode(&self) -> &Smode {
        &self.smode
    }
    #[doc = "0x10 - TRNG Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x14 - TRNG Interrupt Status Register"]
    #[inline(always)]
    pub const fn istat(&self) -> &Istat {
        &self.istat
    }
    #[doc = "0x20..0x40 - TRNG RAND Register"]
    #[inline(always)]
    pub const fn rand(&self, n: usize) -> &Rand {
        &self.rand[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - TRNG RAND Register"]
    #[inline(always)]
    pub fn rand_iter(&self) -> impl Iterator<Item = &Rand> {
        self.rand.iter()
    }
    #[doc = "0x60 - Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter"]
    #[inline(always)]
    pub const fn auto_rqsts(&self) -> &AutoRqsts {
        &self.auto_rqsts
    }
    #[doc = "0x64 - Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer"]
    #[inline(always)]
    pub const fn auto_age(&self) -> &AutoAge {
        &self.auto_age
    }
}
#[doc = "ctrl (rw) register accessor: TRNG Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "ctrl")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "TRNG Control Register"]
pub mod ctrl;
#[doc = "stat (r) register accessor: TRNG STAT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "stat")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "TRNG STAT Register"]
pub mod stat;
#[doc = "mode (rw) register accessor: TRNG MODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "mode")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "TRNG MODE Register"]
pub mod mode;
#[doc = "smode (rw) register accessor: TRNG SMODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smode`]
module"]
#[doc(alias = "smode")]
pub type Smode = crate::Reg<smode::SmodeSpec>;
#[doc = "TRNG SMODE Register"]
pub mod smode;
#[doc = "ie (rw) register accessor: TRNG Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "ie")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "TRNG Interrupt Enable Register"]
pub mod ie;
#[doc = "istat (r) register accessor: TRNG Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istat`]
module"]
#[doc(alias = "istat")]
pub type Istat = crate::Reg<istat::IstatSpec>;
#[doc = "TRNG Interrupt Status Register"]
pub mod istat;
#[doc = "rand (r) register accessor: TRNG RAND Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rand::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rand`]
module"]
#[doc(alias = "rand")]
pub type Rand = crate::Reg<rand::RandSpec>;
#[doc = "TRNG RAND Register"]
pub mod rand;
#[doc = "auto_rqsts (rw) register accessor: Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_rqsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_rqsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_rqsts`]
module"]
#[doc(alias = "auto_rqsts")]
pub type AutoRqsts = crate::Reg<auto_rqsts::AutoRqstsSpec>;
#[doc = "Auto-reseeding after random number requests by host reaches specified counter: 0 - disable counter, other - reload value for internal counter"]
pub mod auto_rqsts;
#[doc = "auto_age (rw) register accessor: Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auto_age::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auto_age::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_age`]
module"]
#[doc(alias = "auto_age")]
pub type AutoAge = crate::Reg<auto_age::AutoAgeSpec>;
#[doc = "Auto-reseeding after specified timer countdowns to 0: 0 - disable timer, other - reload value for internal timer"]
pub mod auto_age;
