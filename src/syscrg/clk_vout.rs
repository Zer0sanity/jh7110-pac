#[repr(C)]
#[doc = "Clock Video Output registers"]
#[doc(alias = "clk_vout")]
pub struct ClkVout {
    src: Src,
    axi_divcfg: AxiDivcfg,
    noc_diplay_axi: NocDiplayAxi,
    ahb: Ahb,
    axi: Axi,
    hdmi_tx0_mclk: HdmiTx0Mclk,
    mipi_phy: MipiPhy,
}
impl ClkVout {
    #[doc = "0x00 - clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src"]
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    #[doc = "0x04 - Clock Video Output AXI DIVCFG"]
    #[inline(always)]
    pub const fn axi_divcfg(&self) -> &AxiDivcfg {
        &self.axi_divcfg
    }
    #[doc = "0x08 - Clock NOC Display AXI"]
    #[inline(always)]
    pub const fn noc_diplay_axi(&self) -> &NocDiplayAxi {
        &self.noc_diplay_axi
    }
    #[doc = "0x0c - Clock Video Output AHB"]
    #[inline(always)]
    pub const fn ahb(&self) -> &Ahb {
        &self.ahb
    }
    #[doc = "0x10 - Clock Video Output AXI"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
    #[doc = "0x14 - Clock Video Output HDMI TX0 MCLK"]
    #[inline(always)]
    pub const fn hdmi_tx0_mclk(&self) -> &HdmiTx0Mclk {
        &self.hdmi_tx0_mclk
    }
    #[doc = "0x18 - Clock Video Output MIPI PHY Reference"]
    #[inline(always)]
    pub const fn mipi_phy(&self) -> &MipiPhy {
        &self.mipi_phy
    }
}
#[doc = "src (rw) register accessor: clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`]
module"]
#[doc(alias = "src")]
pub type Src = crate::Reg<src::SrcSpec>;
#[doc = "clk_u0_dom_vout_top_clk_dom_vout_top_clk_vout_src"]
pub mod src;
#[doc = "axi_divcfg (rw) register accessor: Clock Video Output AXI DIVCFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_divcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_divcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_divcfg`]
module"]
#[doc(alias = "axi_divcfg")]
pub type AxiDivcfg = crate::Reg<axi_divcfg::AxiDivcfgSpec>;
#[doc = "Clock Video Output AXI DIVCFG"]
pub mod axi_divcfg;
#[doc = "noc_diplay_axi (rw) register accessor: Clock NOC Display AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`noc_diplay_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`noc_diplay_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noc_diplay_axi`]
module"]
#[doc(alias = "noc_diplay_axi")]
pub type NocDiplayAxi = crate::Reg<noc_diplay_axi::NocDiplayAxiSpec>;
#[doc = "Clock NOC Display AXI"]
pub mod noc_diplay_axi;
#[doc = "ahb (rw) register accessor: Clock Video Output AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb`]
module"]
#[doc(alias = "ahb")]
pub type Ahb = crate::Reg<ahb::AhbSpec>;
#[doc = "Clock Video Output AHB"]
pub mod ahb;
#[doc = "axi (rw) register accessor: Clock Video Output AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "Clock Video Output AXI"]
pub mod axi;
#[doc = "hdmi_tx0_mclk (rw) register accessor: Clock Video Output HDMI TX0 MCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdmi_tx0_mclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdmi_tx0_mclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdmi_tx0_mclk`]
module"]
#[doc(alias = "hdmi_tx0_mclk")]
pub type HdmiTx0Mclk = crate::Reg<hdmi_tx0_mclk::HdmiTx0MclkSpec>;
#[doc = "Clock Video Output HDMI TX0 MCLK"]
pub mod hdmi_tx0_mclk;
#[doc = "mipi_phy (rw) register accessor: Clock Video Output MIPI PHY Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mipi_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mipi_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mipi_phy`]
module"]
#[doc(alias = "mipi_phy")]
pub type MipiPhy = crate::Reg<mipi_phy::MipiPhySpec>;
#[doc = "Clock Video Output MIPI PHY Reference"]
pub mod mipi_phy;
