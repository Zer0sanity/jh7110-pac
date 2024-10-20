#[repr(C)]
#[doc = "Clock IMG GPU registers"]
#[doc(alias = "clk_img_gpu")]
pub struct ClkImgGpu {
    divcfg: Divcfg,
    core: Core,
    sys: Sys,
    apb: Apb,
    rtc_toggle: RtcToggle,
    noc_bus_axi: NocBusAxi,
}
impl ClkImgGpu {
    #[doc = "0x00 - Clock IMG GPU Core DIVCFG"]
    #[inline(always)]
    pub const fn divcfg(&self) -> &Divcfg {
        &self.divcfg
    }
    #[doc = "0x04 - clk_gpu_core"]
    #[inline(always)]
    pub const fn core(&self) -> &Core {
        &self.core
    }
    #[doc = "0x08 - clk_u0_img_gpu_sys_clk"]
    #[inline(always)]
    pub const fn sys(&self) -> &Sys {
        &self.sys
    }
    #[doc = "0x0c - clk_u0_img_gpu_apb_clk"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x10 - clk_u0_img_gpu_rtc_toggle"]
    #[inline(always)]
    pub const fn rtc_toggle(&self) -> &RtcToggle {
        &self.rtc_toggle
    }
    #[doc = "0x14 - clk_u0_sft7110_noc_bus_clk_gpu_axi"]
    #[inline(always)]
    pub const fn noc_bus_axi(&self) -> &NocBusAxi {
        &self.noc_bus_axi
    }
}
#[doc = "divcfg (rw) register accessor: Clock IMG GPU Core DIVCFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcfg`]
module"]
#[doc(alias = "divcfg")]
pub type Divcfg = crate::Reg<divcfg::DivcfgSpec>;
#[doc = "Clock IMG GPU Core DIVCFG"]
pub mod divcfg;
#[doc = "core (rw) register accessor: clk_gpu_core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core`]
module"]
#[doc(alias = "core")]
pub type Core = crate::Reg<core::CoreSpec>;
#[doc = "clk_gpu_core"]
pub mod core;
#[doc = "sys (rw) register accessor: clk_u0_img_gpu_sys_clk\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys`]
module"]
#[doc(alias = "sys")]
pub type Sys = crate::Reg<sys::SysSpec>;
#[doc = "clk_u0_img_gpu_sys_clk"]
pub mod sys;
#[doc = "apb (rw) register accessor: clk_u0_img_gpu_apb_clk\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "clk_u0_img_gpu_apb_clk"]
pub mod apb;
#[doc = "rtc_toggle (rw) register accessor: clk_u0_img_gpu_rtc_toggle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_toggle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_toggle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_toggle`]
module"]
#[doc(alias = "rtc_toggle")]
pub type RtcToggle = crate::Reg<rtc_toggle::RtcToggleSpec>;
#[doc = "clk_u0_img_gpu_rtc_toggle"]
pub mod rtc_toggle;
#[doc = "noc_bus_axi (rw) register accessor: clk_u0_sft7110_noc_bus_clk_gpu_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`noc_bus_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`noc_bus_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noc_bus_axi`]
module"]
#[doc(alias = "noc_bus_axi")]
pub type NocBusAxi = crate::Reg<noc_bus_axi::NocBusAxiSpec>;
#[doc = "clk_u0_sft7110_noc_bus_clk_gpu_axi"]
pub mod noc_bus_axi;
