#[repr(C)]
#[doc = "Clock DDR registers"]
#[doc(alias = "clk_ddr")]
pub struct ClkDdr {
    bus: Bus,
    axi: Axi,
}
impl ClkDdr {
    #[doc = "0x00 - clk_ddr_bus"]
    #[inline(always)]
    pub const fn bus(&self) -> &Bus {
        &self.bus
    }
    #[doc = "0x04 - clk_ddr_axi"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
}
#[doc = "bus (rw) register accessor: clk_ddr_bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus`]
module"]
#[doc(alias = "bus")]
pub type Bus = crate::Reg<bus::BusSpec>;
#[doc = "clk_ddr_bus"]
pub mod bus;
#[doc = "axi (rw) register accessor: clk_ddr_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "clk_ddr_axi"]
pub mod axi;
