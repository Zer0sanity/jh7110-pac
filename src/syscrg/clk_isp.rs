#[repr(C)]
#[doc = "Clock ISP registers"]
#[doc(alias = "clk_isp")]
pub struct ClkIsp {
    isp_2x: Isp2x,
    axi: Axi,
}
impl ClkIsp {
    #[doc = "0x00 - Clock ISP 2x"]
    #[inline(always)]
    pub const fn isp_2x(&self) -> &Isp2x {
        &self.isp_2x
    }
    #[doc = "0x04 - Clock ISP AXI"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
}
#[doc = "isp_2x (rw) register accessor: Clock ISP 2x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_2x::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_2x::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_2x`]
module"]
#[doc(alias = "isp_2x")]
pub type Isp2x = crate::Reg<isp_2x::Isp2xSpec>;
#[doc = "Clock ISP 2x"]
pub mod isp_2x;
#[doc = "axi (rw) register accessor: Clock ISP AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "Clock ISP AXI"]
pub mod axi;
