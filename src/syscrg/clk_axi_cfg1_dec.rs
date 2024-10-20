#[repr(C)]
#[doc = "Clock AXI CFG 1 DEC registers"]
#[doc(alias = "clk_axi_cfg1_dec")]
pub struct ClkAxiCfg1Dec {
    dec_main: DecMain,
    ahb: Ahb,
}
impl ClkAxiCfg1Dec {
    #[doc = "0x00 - clk_u0_axi_cfg1_dec_clk_main"]
    #[inline(always)]
    pub const fn dec_main(&self) -> &DecMain {
        &self.dec_main
    }
    #[doc = "0x04 - clk_u0_axi_cfg1_dec_clk_ahb"]
    #[inline(always)]
    pub const fn ahb(&self) -> &Ahb {
        &self.ahb
    }
}
#[doc = "dec_main (rw) register accessor: clk_u0_axi_cfg1_dec_clk_main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dec_main::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dec_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dec_main`]
module"]
#[doc(alias = "dec_main")]
pub type DecMain = crate::Reg<dec_main::DecMainSpec>;
#[doc = "clk_u0_axi_cfg1_dec_clk_main"]
pub mod dec_main;
#[doc = "ahb (rw) register accessor: clk_u0_axi_cfg1_dec_clk_ahb\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb`]
module"]
#[doc(alias = "ahb")]
pub type Ahb = crate::Reg<ahb::AhbSpec>;
#[doc = "clk_u0_axi_cfg1_dec_clk_ahb"]
pub mod ahb;
