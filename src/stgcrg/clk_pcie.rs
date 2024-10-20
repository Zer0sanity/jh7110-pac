#[repr(C)]
#[doc = "Clock PCIe configuration"]
#[doc(alias = "clk_pcie")]
pub struct ClkPcie {
    axi_mst0: AxiMst0,
    apb: Apb,
    tl: Tl,
}
impl ClkPcie {
    #[doc = "0x00 - Clock PCIe AXI MST0"]
    #[inline(always)]
    pub const fn axi_mst0(&self) -> &AxiMst0 {
        &self.axi_mst0
    }
    #[doc = "0x04 - Clock PCIe APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x08 - Clock PCIe TL"]
    #[inline(always)]
    pub const fn tl(&self) -> &Tl {
        &self.tl
    }
}
#[doc = "axi_mst0 (rw) register accessor: Clock PCIe AXI MST0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_mst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_mst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_mst0`]
module"]
#[doc(alias = "axi_mst0")]
pub type AxiMst0 = crate::Reg<axi_mst0::AxiMst0Spec>;
#[doc = "Clock PCIe AXI MST0"]
pub mod axi_mst0;
#[doc = "apb (rw) register accessor: Clock PCIe APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock PCIe APB"]
pub mod apb;
#[doc = "tl (rw) register accessor: Clock PCIe TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tl`]
module"]
#[doc(alias = "tl")]
pub type Tl = crate::Reg<tl::TlSpec>;
#[doc = "Clock PCIe TL"]
pub mod tl;
