#[repr(C)]
#[doc = "Clock NOC Bus registers"]
#[doc(alias = "clk_noc_bus")]
pub struct ClkNocBus {
    cpu_axi: CpuAxi,
    axicfg0_axi: Axicfg0Axi,
}
impl ClkNocBus {
    #[doc = "0x00 - clk_u0_sft7110_noc_bus_clk_cpu_axi"]
    #[inline(always)]
    pub const fn cpu_axi(&self) -> &CpuAxi {
        &self.cpu_axi
    }
    #[doc = "0x04 - clk_u0_sft7110_noc_bus_clk_axicfg0_axi"]
    #[inline(always)]
    pub const fn axicfg0_axi(&self) -> &Axicfg0Axi {
        &self.axicfg0_axi
    }
}
#[doc = "cpu_axi (rw) register accessor: clk_u0_sft7110_noc_bus_clk_cpu_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpu_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_axi`]
module"]
#[doc(alias = "cpu_axi")]
pub type CpuAxi = crate::Reg<cpu_axi::CpuAxiSpec>;
#[doc = "clk_u0_sft7110_noc_bus_clk_cpu_axi"]
pub mod cpu_axi;
#[doc = "axicfg0_axi (rw) register accessor: clk_u0_sft7110_noc_bus_clk_axicfg0_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axicfg0_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axicfg0_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axicfg0_axi`]
module"]
#[doc(alias = "axicfg0_axi")]
pub type Axicfg0Axi = crate::Reg<axicfg0_axi::Axicfg0AxiSpec>;
#[doc = "clk_u0_sft7110_noc_bus_clk_axicfg0_axi"]
pub mod axicfg0_axi;
