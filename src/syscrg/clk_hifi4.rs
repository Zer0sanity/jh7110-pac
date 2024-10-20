#[repr(C)]
#[doc = "Clock HIFI4 registers"]
#[doc(alias = "clk_hifi4")]
pub struct ClkHifi4 {
    core: Core,
    axi: Axi,
}
impl ClkHifi4 {
    #[doc = "0x00 - clk_hifi4_core"]
    #[inline(always)]
    pub const fn core(&self) -> &Core {
        &self.core
    }
    #[doc = "0x04 - clk_hifi4_axi"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
}
#[doc = "core (rw) register accessor: clk_hifi4_core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core`]
module"]
#[doc(alias = "core")]
pub type Core = crate::Reg<core::CoreSpec>;
#[doc = "clk_hifi4_core"]
pub mod core;
#[doc = "axi (rw) register accessor: clk_hifi4_axi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "clk_hifi4_axi"]
pub mod axi;
