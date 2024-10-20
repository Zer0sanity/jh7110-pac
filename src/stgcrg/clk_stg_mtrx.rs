#[repr(C)]
#[doc = "Clock STG Matrix Group configuration"]
#[doc(alias = "clk_stg_mtrx")]
pub struct ClkStgMtrx {
    main: Main,
    bus: Bus,
    stg: Stg,
}
impl ClkStgMtrx {
    #[doc = "0x00 - Clock STG Matrix Group Main"]
    #[inline(always)]
    pub const fn main(&self) -> &Main {
        &self.main
    }
    #[doc = "0x04 - Clock STG Matrix Group Bus"]
    #[inline(always)]
    pub const fn bus(&self) -> &Bus {
        &self.bus
    }
    #[doc = "0x08 - Clock STG Matrix Group STG"]
    #[inline(always)]
    pub const fn stg(&self) -> &Stg {
        &self.stg
    }
}
#[doc = "main (rw) register accessor: Clock STG Matrix Group Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`main::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main`]
module"]
#[doc(alias = "main")]
pub type Main = crate::Reg<main::MainSpec>;
#[doc = "Clock STG Matrix Group Main"]
pub mod main;
#[doc = "bus (rw) register accessor: Clock STG Matrix Group Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus`]
module"]
#[doc(alias = "bus")]
pub type Bus = crate::Reg<bus::BusSpec>;
#[doc = "Clock STG Matrix Group Bus"]
pub mod bus;
#[doc = "stg (rw) register accessor: Clock STG Matrix Group STG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stg`]
module"]
#[doc(alias = "stg")]
pub type Stg = crate::Reg<stg::StgSpec>;
#[doc = "Clock STG Matrix Group STG"]
pub mod stg;
