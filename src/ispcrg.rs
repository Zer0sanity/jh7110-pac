#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_dom4_apb_func: ClkDom4ApbFunc,
    clk_mipi_rx0_pxl: ClkMipiRx0Pxl,
    clk_dvp_inv: ClkDvpInv,
    clk_u0_m31dphy_cfgclk_in: ClkU0M31dphyCfgclkIn,
    clk_u0_m31dphy_refclk_in: ClkU0M31dphyRefclkIn,
    clk_u0_m31dphy_txclkesc_lan0: ClkU0M31dphyTxclkescLan0,
    clk_u0_vin_pclk: ClkU0VinPclk,
    clk_u0_vin_sys_clk: ClkU0VinSysClk,
    clk_u0_vin_pixel_clk_if: [ClkU0VinPixelClkIf; 4],
    clk_u0_vin_p_axiwr: ClkU0VinPAxiwr,
    clk_ispv2_top_wrapper: ClkIspv2TopWrapper,
    reset: [Reset; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - Clock DOM4 APB Function"]
    #[inline(always)]
    pub const fn clk_dom4_apb_func(&self) -> &ClkDom4ApbFunc {
        &self.clk_dom4_apb_func
    }
    #[doc = "0x04 - Clock MIPI RX 0 Pixel"]
    #[inline(always)]
    pub const fn clk_mipi_rx0_pxl(&self) -> &ClkMipiRx0Pxl {
        &self.clk_mipi_rx0_pxl
    }
    #[doc = "0x08 - Clock DVP Inverter"]
    #[inline(always)]
    pub const fn clk_dvp_inv(&self) -> &ClkDvpInv {
        &self.clk_dvp_inv
    }
    #[doc = "0x0c - Clock U0 M31 DPHY Config In"]
    #[inline(always)]
    pub const fn clk_u0_m31dphy_cfgclk_in(&self) -> &ClkU0M31dphyCfgclkIn {
        &self.clk_u0_m31dphy_cfgclk_in
    }
    #[doc = "0x10 - Clock U0 M31 DPHY Reference In"]
    #[inline(always)]
    pub const fn clk_u0_m31dphy_refclk_in(&self) -> &ClkU0M31dphyRefclkIn {
        &self.clk_u0_m31dphy_refclk_in
    }
    #[doc = "0x14 - Clock U0 M31 DPHY TX Clock Escape LAN0"]
    #[inline(always)]
    pub const fn clk_u0_m31dphy_txclkesc_lan0(&self) -> &ClkU0M31dphyTxclkescLan0 {
        &self.clk_u0_m31dphy_txclkesc_lan0
    }
    #[doc = "0x18 - Clock U0 Video In PCLK"]
    #[inline(always)]
    pub const fn clk_u0_vin_pclk(&self) -> &ClkU0VinPclk {
        &self.clk_u0_vin_pclk
    }
    #[doc = "0x1c - Clock U0 Video In SYSCLK"]
    #[inline(always)]
    pub const fn clk_u0_vin_sys_clk(&self) -> &ClkU0VinSysClk {
        &self.clk_u0_vin_sys_clk
    }
    #[doc = "0x20..0x30 - Clock Video In Pixel Clock Interfaces"]
    #[inline(always)]
    pub const fn clk_u0_vin_pixel_clk_if(&self, n: usize) -> &ClkU0VinPixelClkIf {
        &self.clk_u0_vin_pixel_clk_if[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Clock Video In Pixel Clock Interfaces"]
    #[inline(always)]
    pub fn clk_u0_vin_pixel_clk_if_iter(&self) -> impl Iterator<Item = &ClkU0VinPixelClkIf> {
        self.clk_u0_vin_pixel_clk_if.iter()
    }
    #[doc = "0x30 - Clock U0 Video In P AXIWR"]
    #[inline(always)]
    pub const fn clk_u0_vin_p_axiwr(&self) -> &ClkU0VinPAxiwr {
        &self.clk_u0_vin_p_axiwr
    }
    #[doc = "0x34 - Clock ISPv2 Top Wrapper Clock C: clk_ispv2_top_wrapper_clk_c"]
    #[inline(always)]
    pub const fn clk_ispv2_top_wrapper(&self) -> &ClkIspv2TopWrapper {
        &self.clk_ispv2_top_wrapper
    }
    #[doc = "0x38..0x40 - ISPCRG Reset registers"]
    #[inline(always)]
    pub const fn reset(&self, n: usize) -> &Reset {
        &self.reset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x40 - ISPCRG Reset registers"]
    #[inline(always)]
    pub fn reset_iter(&self) -> impl Iterator<Item = &Reset> {
        self.reset.iter()
    }
    #[doc = "0x38 - ISPCRG Reset registers"]
    #[inline(always)]
    pub const fn reset_software_assert0_addr_assert_sel(&self) -> &Reset {
        self.reset(0)
    }
    #[doc = "0x3c - ISPCRG Reset registers"]
    #[inline(always)]
    pub const fn reset_ispcrg_status(&self) -> &Reset {
        self.reset(1)
    }
}
#[doc = "clk_dom4_apb_func (rw) register accessor: Clock DOM4 APB Function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dom4_apb_func::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dom4_apb_func::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dom4_apb_func`]
module"]
#[doc(alias = "clk_dom4_apb_func")]
pub type ClkDom4ApbFunc = crate::Reg<clk_dom4_apb_func::ClkDom4ApbFuncSpec>;
#[doc = "Clock DOM4 APB Function"]
pub mod clk_dom4_apb_func;
#[doc = "clk_mipi_rx0_pxl (rw) register accessor: Clock MIPI RX 0 Pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_mipi_rx0_pxl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_mipi_rx0_pxl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_mipi_rx0_pxl`]
module"]
#[doc(alias = "clk_mipi_rx0_pxl")]
pub type ClkMipiRx0Pxl = crate::Reg<clk_mipi_rx0_pxl::ClkMipiRx0PxlSpec>;
#[doc = "Clock MIPI RX 0 Pixel"]
pub mod clk_mipi_rx0_pxl;
#[doc = "clk_dvp_inv (rw) register accessor: Clock DVP Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dvp_inv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dvp_inv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dvp_inv`]
module"]
#[doc(alias = "clk_dvp_inv")]
pub type ClkDvpInv = crate::Reg<clk_dvp_inv::ClkDvpInvSpec>;
#[doc = "Clock DVP Inverter"]
pub mod clk_dvp_inv;
#[doc = "clk_u0_m31dphy_cfgclk_in (rw) register accessor: Clock U0 M31 DPHY Config In\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_m31dphy_cfgclk_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_m31dphy_cfgclk_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_m31dphy_cfgclk_in`]
module"]
#[doc(alias = "clk_u0_m31dphy_cfgclk_in")]
pub type ClkU0M31dphyCfgclkIn = crate::Reg<clk_u0_m31dphy_cfgclk_in::ClkU0M31dphyCfgclkInSpec>;
#[doc = "Clock U0 M31 DPHY Config In"]
pub mod clk_u0_m31dphy_cfgclk_in;
#[doc = "clk_u0_m31dphy_refclk_in (rw) register accessor: Clock U0 M31 DPHY Reference In\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_m31dphy_refclk_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_m31dphy_refclk_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_m31dphy_refclk_in`]
module"]
#[doc(alias = "clk_u0_m31dphy_refclk_in")]
pub type ClkU0M31dphyRefclkIn = crate::Reg<clk_u0_m31dphy_refclk_in::ClkU0M31dphyRefclkInSpec>;
#[doc = "Clock U0 M31 DPHY Reference In"]
pub mod clk_u0_m31dphy_refclk_in;
#[doc = "clk_u0_m31dphy_txclkesc_lan0 (rw) register accessor: Clock U0 M31 DPHY TX Clock Escape LAN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_m31dphy_txclkesc_lan0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_m31dphy_txclkesc_lan0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_m31dphy_txclkesc_lan0`]
module"]
#[doc(alias = "clk_u0_m31dphy_txclkesc_lan0")]
pub type ClkU0M31dphyTxclkescLan0 =
    crate::Reg<clk_u0_m31dphy_txclkesc_lan0::ClkU0M31dphyTxclkescLan0Spec>;
#[doc = "Clock U0 M31 DPHY TX Clock Escape LAN0"]
pub mod clk_u0_m31dphy_txclkesc_lan0;
#[doc = "clk_u0_vin_pclk (rw) register accessor: Clock U0 Video In PCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_vin_pclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_vin_pclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_vin_pclk`]
module"]
#[doc(alias = "clk_u0_vin_pclk")]
pub type ClkU0VinPclk = crate::Reg<clk_u0_vin_pclk::ClkU0VinPclkSpec>;
#[doc = "Clock U0 Video In PCLK"]
pub mod clk_u0_vin_pclk;
#[doc = "clk_u0_vin_sys_clk (rw) register accessor: Clock U0 Video In SYSCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_vin_sys_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_vin_sys_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_vin_sys_clk`]
module"]
#[doc(alias = "clk_u0_vin_sys_clk")]
pub type ClkU0VinSysClk = crate::Reg<clk_u0_vin_sys_clk::ClkU0VinSysClkSpec>;
#[doc = "Clock U0 Video In SYSCLK"]
pub mod clk_u0_vin_sys_clk;
#[doc = "clk_u0_vin_pixel_clk_if (rw) register accessor: Clock Video In Pixel Clock Interfaces\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_vin_pixel_clk_if::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_vin_pixel_clk_if::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_vin_pixel_clk_if`]
module"]
#[doc(alias = "clk_u0_vin_pixel_clk_if")]
pub type ClkU0VinPixelClkIf = crate::Reg<clk_u0_vin_pixel_clk_if::ClkU0VinPixelClkIfSpec>;
#[doc = "Clock Video In Pixel Clock Interfaces"]
pub mod clk_u0_vin_pixel_clk_if;
#[doc = "clk_u0_vin_p_axiwr (rw) register accessor: Clock U0 Video In P AXIWR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_vin_p_axiwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_vin_p_axiwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_vin_p_axiwr`]
module"]
#[doc(alias = "clk_u0_vin_p_axiwr")]
pub type ClkU0VinPAxiwr = crate::Reg<clk_u0_vin_p_axiwr::ClkU0VinPAxiwrSpec>;
#[doc = "Clock U0 Video In P AXIWR"]
pub mod clk_u0_vin_p_axiwr;
#[doc = "clk_ispv2_top_wrapper (rw) register accessor: Clock ISPv2 Top Wrapper Clock C: clk_ispv2_top_wrapper_clk_c\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ispv2_top_wrapper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ispv2_top_wrapper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ispv2_top_wrapper`]
module"]
#[doc(alias = "clk_ispv2_top_wrapper")]
pub type ClkIspv2TopWrapper = crate::Reg<clk_ispv2_top_wrapper::ClkIspv2TopWrapperSpec>;
#[doc = "Clock ISPv2 Top Wrapper Clock C: clk_ispv2_top_wrapper_clk_c"]
pub mod clk_ispv2_top_wrapper;
#[doc = "reset (rw) register accessor: ISPCRG Reset registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "reset")]
pub type Reset = crate::Reg<reset::ResetSpec>;
#[doc = "ISPCRG Reset registers"]
pub mod reset;
