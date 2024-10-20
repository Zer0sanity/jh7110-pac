#[repr(C)]
#[doc = "Clock ISP DOM registers"]
#[doc(alias = "clk_isp_dom")]
pub struct ClkIspDom {
    ispcore_2x: Ispcore2x,
    axi: Axi,
    noc_bus_axi: NocBusAxi,
}
impl ClkIspDom {
    #[doc = "0x00 - clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x"]
    #[inline(always)]
    pub const fn ispcore_2x(&self) -> &Ispcore2x {
        &self.ispcore_2x
    }
    #[doc = "0x04 - clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
    #[doc = "0x08 - clk_u0_sft7110_noc_bus_clk_isp_axi"]
    #[inline(always)]
    pub const fn noc_bus_axi(&self) -> &NocBusAxi {
        &self.noc_bus_axi
    }
}
#[doc = "ispcore_2x (rw) register accessor: clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ispcore_2x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ispcore_2x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ispcore_2x`]
module"]
#[doc(alias = "ispcore_2x")]
pub type Ispcore2x = crate::Reg<ispcore_2x::Ispcore2xSpec>;
#[doc = "clk_u0_dom_isp_top_clk_dom_isp_top_clk_ispcore_2x"]
pub mod ispcore_2x;
#[doc = "axi (rw) register accessor: clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "clk_u0_dom_isp_top_clk_dom_isp_top_clk_isp_axi"]
pub mod axi;
#[doc = "noc_bus_axi (rw) register accessor: clk_u0_sft7110_noc_bus_clk_isp_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`noc_bus_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`noc_bus_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@noc_bus_axi`]
module"]
#[doc(alias = "noc_bus_axi")]
pub type NocBusAxi = crate::Reg<noc_bus_axi::NocBusAxiSpec>;
#[doc = "clk_u0_sft7110_noc_bus_clk_isp_axi"]
pub mod noc_bus_axi;
