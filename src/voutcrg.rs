#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_apb: ClkApb,
    clk_dc8200_pix0: ClkDc8200Pix0,
    clk_dsi_sys: ClkDsiSys,
    clk_tx_esc: ClkTxEsc,
    clk_u0_dc8200: ClkU0Dc8200,
    clk_dom_vout_top_lcd: ClkDomVoutTopLcd,
    clk_u0_cdns_dsitx: ClkU0CdnsDsitx,
    clk_u0_mipitx_dphy_txesc: ClkU0MipitxDphyTxesc,
    clk_u0_hdmi_tx: [ClkU0HdmiTx; 3],
    rst: [Rst; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - Clock APB"]
    #[inline(always)]
    pub const fn clk_apb(&self) -> &ClkApb {
        &self.clk_apb
    }
    #[doc = "0x04 - Clock DC8200 Pixel 0"]
    #[inline(always)]
    pub const fn clk_dc8200_pix0(&self) -> &ClkDc8200Pix0 {
        &self.clk_dc8200_pix0
    }
    #[doc = "0x08 - Clock DSI System"]
    #[inline(always)]
    pub const fn clk_dsi_sys(&self) -> &ClkDsiSys {
        &self.clk_dsi_sys
    }
    #[doc = "0x0c - Clock Transmit Escape"]
    #[inline(always)]
    pub const fn clk_tx_esc(&self) -> &ClkTxEsc {
        &self.clk_tx_esc
    }
    #[doc = "0x10..0x24 - Clock U0 DC8200 registers"]
    #[inline(always)]
    pub const fn clk_u0_dc8200(&self) -> &ClkU0Dc8200 {
        &self.clk_u0_dc8200
    }
    #[doc = "0x24 - Clock DOM VOUT Top LCD"]
    #[inline(always)]
    pub const fn clk_dom_vout_top_lcd(&self) -> &ClkDomVoutTopLcd {
        &self.clk_dom_vout_top_lcd
    }
    #[doc = "0x28..0x38 - Clock U0 Cadence DSI Transmit registers"]
    #[inline(always)]
    pub const fn clk_u0_cdns_dsitx(&self) -> &ClkU0CdnsDsitx {
        &self.clk_u0_cdns_dsitx
    }
    #[doc = "0x38 - Clock U0 MIPI Transmit DPHY Transmit Escape"]
    #[inline(always)]
    pub const fn clk_u0_mipitx_dphy_txesc(&self) -> &ClkU0MipitxDphyTxesc {
        &self.clk_u0_mipitx_dphy_txesc
    }
    #[doc = "0x3c..0x48 - Clock U0 HDMI Transmit registers"]
    #[inline(always)]
    pub const fn clk_u0_hdmi_tx(&self, n: usize) -> &ClkU0HdmiTx {
        &self.clk_u0_hdmi_tx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x48 - Clock U0 HDMI Transmit registers"]
    #[inline(always)]
    pub fn clk_u0_hdmi_tx_iter(&self) -> impl Iterator<Item = &ClkU0HdmiTx> {
        self.clk_u0_hdmi_tx.iter()
    }
    #[doc = "0x3c - Clock U0 HDMI Transmit registers"]
    #[inline(always)]
    pub const fn clk_u0_hdmi_tx_mclk(&self) -> &ClkU0HdmiTx {
        self.clk_u0_hdmi_tx(0)
    }
    #[doc = "0x40 - Clock U0 HDMI Transmit registers"]
    #[inline(always)]
    pub const fn clk_u0_hdmi_tx_bclk(&self) -> &ClkU0HdmiTx {
        self.clk_u0_hdmi_tx(1)
    }
    #[doc = "0x44 - Clock U0 HDMI Transmit registers"]
    #[inline(always)]
    pub const fn clk_u0_hdmi_tx_sys(&self) -> &ClkU0HdmiTx {
        self.clk_u0_hdmi_tx(2)
    }
    #[doc = "0x48..0x50 - VOUT CRG RESET register"]
    #[inline(always)]
    pub const fn rst(&self, n: usize) -> &Rst {
        &self.rst[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x48..0x50 - VOUT CRG RESET register"]
    #[inline(always)]
    pub fn rst_iter(&self) -> impl Iterator<Item = &Rst> {
        self.rst.iter()
    }
    #[doc = "0x48 - VOUT CRG RESET register"]
    #[inline(always)]
    pub const fn rst_software_assert0_addr_assert_sel(&self) -> &Rst {
        self.rst(0)
    }
    #[doc = "0x4c - VOUT CRG RESET register"]
    #[inline(always)]
    pub const fn rst_voutcrg_status(&self) -> &Rst {
        self.rst(1)
    }
}
#[doc = "clk_apb (rw) register accessor: Clock APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb`]
module"]
#[doc(alias = "clk_apb")]
pub type ClkApb = crate::Reg<clk_apb::ClkApbSpec>;
#[doc = "Clock APB"]
pub mod clk_apb;
#[doc = "clk_dc8200_pix0 (rw) register accessor: Clock DC8200 Pixel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dc8200_pix0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dc8200_pix0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dc8200_pix0`]
module"]
#[doc(alias = "clk_dc8200_pix0")]
pub type ClkDc8200Pix0 = crate::Reg<clk_dc8200_pix0::ClkDc8200Pix0Spec>;
#[doc = "Clock DC8200 Pixel 0"]
pub mod clk_dc8200_pix0;
#[doc = "clk_dsi_sys (rw) register accessor: Clock DSI System\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dsi_sys::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dsi_sys::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dsi_sys`]
module"]
#[doc(alias = "clk_dsi_sys")]
pub type ClkDsiSys = crate::Reg<clk_dsi_sys::ClkDsiSysSpec>;
#[doc = "Clock DSI System"]
pub mod clk_dsi_sys;
#[doc = "clk_tx_esc (rw) register accessor: Clock Transmit Escape\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tx_esc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tx_esc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_tx_esc`]
module"]
#[doc(alias = "clk_tx_esc")]
pub type ClkTxEsc = crate::Reg<clk_tx_esc::ClkTxEscSpec>;
#[doc = "Clock Transmit Escape"]
pub mod clk_tx_esc;
#[doc = "Clock U0 DC8200 registers"]
pub use self::clk_u0_dc8200::ClkU0Dc8200;
#[doc = r"Cluster"]
#[doc = "Clock U0 DC8200 registers"]
pub mod clk_u0_dc8200;
#[doc = "clk_dom_vout_top_lcd (rw) register accessor: Clock DOM VOUT Top LCD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dom_vout_top_lcd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dom_vout_top_lcd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dom_vout_top_lcd`]
module"]
#[doc(alias = "clk_dom_vout_top_lcd")]
pub type ClkDomVoutTopLcd = crate::Reg<clk_dom_vout_top_lcd::ClkDomVoutTopLcdSpec>;
#[doc = "Clock DOM VOUT Top LCD"]
pub mod clk_dom_vout_top_lcd;
#[doc = "Clock U0 Cadence DSI Transmit registers"]
pub use self::clk_u0_cdns_dsitx::ClkU0CdnsDsitx;
#[doc = r"Cluster"]
#[doc = "Clock U0 Cadence DSI Transmit registers"]
pub mod clk_u0_cdns_dsitx;
#[doc = "clk_u0_mipitx_dphy_txesc (rw) register accessor: Clock U0 MIPI Transmit DPHY Transmit Escape\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_mipitx_dphy_txesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_mipitx_dphy_txesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_mipitx_dphy_txesc`]
module"]
#[doc(alias = "clk_u0_mipitx_dphy_txesc")]
pub type ClkU0MipitxDphyTxesc = crate::Reg<clk_u0_mipitx_dphy_txesc::ClkU0MipitxDphyTxescSpec>;
#[doc = "Clock U0 MIPI Transmit DPHY Transmit Escape"]
pub mod clk_u0_mipitx_dphy_txesc;
#[doc = "clk_u0_hdmi_tx (rw) register accessor: Clock U0 HDMI Transmit registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_hdmi_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_hdmi_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_u0_hdmi_tx`]
module"]
#[doc(alias = "clk_u0_hdmi_tx")]
pub type ClkU0HdmiTx = crate::Reg<clk_u0_hdmi_tx::ClkU0HdmiTxSpec>;
#[doc = "Clock U0 HDMI Transmit registers"]
pub mod clk_u0_hdmi_tx;
#[doc = "rst (rw) register accessor: VOUT CRG RESET register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst`]
module"]
#[doc(alias = "rst")]
pub type Rst = crate::Reg<rst::RstSpec>;
#[doc = "VOUT CRG RESET register"]
pub mod rst;
