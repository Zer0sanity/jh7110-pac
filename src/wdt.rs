#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    load: Load,
    value: Value,
    control: Control,
    int_clear: IntClear,
    ris: Ris,
    ims: Ims,
    _reserved6: [u8; 0x0be8],
    lock: Lock,
    _reserved7: [u8; 0x02fc],
    itcr: Itcr,
    itop: Itop,
}
impl RegisterBlock {
    #[doc = "0x00 - StarFive JH7110 Watchdog Load register."]
    #[inline(always)]
    pub const fn load(&self) -> &Load {
        &self.load
    }
    #[doc = "0x04 - StarFive JH7110 Watchdog Value register."]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
    #[doc = "0x08 - StarFive JH7110 Watchdog Control register."]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x0c - StarFive JH7110 Watchdog Interrupt Clear register."]
    #[inline(always)]
    pub const fn int_clear(&self) -> &IntClear {
        &self.int_clear
    }
    #[doc = "0x10 - StarFive JH7110 Watchdog Raw Interrupt Status register."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x14 - StarFive JH7110 Watchdog Interrupt Masked Status register."]
    #[inline(always)]
    pub const fn ims(&self) -> &Ims {
        &self.ims
    }
    #[doc = "0xc00 - StarFive JH7110 Watchdog Lock register."]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0xf00 - StarFive JH7110 Watchdog Integration Test Control register."]
    #[inline(always)]
    pub const fn itcr(&self) -> &Itcr {
        &self.itcr
    }
    #[doc = "0xf04 - StarFive JH7110 Watchdog Integration Test Operation register."]
    #[inline(always)]
    pub const fn itop(&self) -> &Itop {
        &self.itop
    }
}
#[doc = "load (rw) register accessor: StarFive JH7110 Watchdog Load register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load`]
module"]
#[doc(alias = "load")]
pub type Load = crate::Reg<load::LoadSpec>;
#[doc = "StarFive JH7110 Watchdog Load register."]
pub mod load;
#[doc = "value (r) register accessor: StarFive JH7110 Watchdog Value register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
#[doc(alias = "value")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "StarFive JH7110 Watchdog Value register."]
pub mod value;
#[doc = "control (rw) register accessor: StarFive JH7110 Watchdog Control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "StarFive JH7110 Watchdog Control register."]
pub mod control;
#[doc = "int_clear (w) register accessor: StarFive JH7110 Watchdog Interrupt Clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`]
module"]
#[doc(alias = "int_clear")]
pub type IntClear = crate::Reg<int_clear::IntClearSpec>;
#[doc = "StarFive JH7110 Watchdog Interrupt Clear register."]
pub mod int_clear;
#[doc = "ris (r) register accessor: StarFive JH7110 Watchdog Raw Interrupt Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "ris")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "StarFive JH7110 Watchdog Raw Interrupt Status register."]
pub mod ris;
#[doc = "ims (r) register accessor: StarFive JH7110 Watchdog Interrupt Masked Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ims::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ims`]
module"]
#[doc(alias = "ims")]
pub type Ims = crate::Reg<ims::ImsSpec>;
#[doc = "StarFive JH7110 Watchdog Interrupt Masked Status register."]
pub mod ims;
#[doc = "lock (rw) register accessor: StarFive JH7110 Watchdog Lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
#[doc(alias = "lock")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "StarFive JH7110 Watchdog Lock register."]
pub mod lock;
#[doc = "itcr (rw) register accessor: StarFive JH7110 Watchdog Integration Test Control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itcr`]
module"]
#[doc(alias = "itcr")]
pub type Itcr = crate::Reg<itcr::ItcrSpec>;
#[doc = "StarFive JH7110 Watchdog Integration Test Control register."]
pub mod itcr;
#[doc = "itop (w) register accessor: StarFive JH7110 Watchdog Integration Test Operation register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itop`]
module"]
#[doc(alias = "itop")]
pub type Itop = crate::Reg<itop::ItopSpec>;
#[doc = "StarFive JH7110 Watchdog Integration Test Operation register."]
pub mod itop;
