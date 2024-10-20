#[repr(C)]
#[doc = "Clock AXI CFG 0 DEC registers"]
#[doc(alias = "clk_axi_cfg0_dec")]
pub struct ClkAxiCfg0Dec {
    main_div: MainDiv,
    dec_main: DecMain,
    hifi4: Hifi4,
}
impl ClkAxiCfg0Dec {
    #[doc = "0x00 - Clock AXI Config 0 DEC Main Divider"]
    #[inline(always)]
    pub const fn main_div(&self) -> &MainDiv {
        &self.main_div
    }
    #[doc = "0x04 - Clock AXI Config 0 DEC Main"]
    #[inline(always)]
    pub const fn dec_main(&self) -> &DecMain {
        &self.dec_main
    }
    #[doc = "0x08 - Clock AXI Config 0 DEC HIFI4"]
    #[inline(always)]
    pub const fn hifi4(&self) -> &Hifi4 {
        &self.hifi4
    }
}
#[doc = "main_div (rw) register accessor: Clock AXI Config 0 DEC Main Divider\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`main_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_div`]
module"]
#[doc(alias = "main_div")]
pub type MainDiv = crate::Reg<main_div::MainDivSpec>;
#[doc = "Clock AXI Config 0 DEC Main Divider"]
pub mod main_div;
#[doc = "dec_main (rw) register accessor: Clock AXI Config 0 DEC Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dec_main::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dec_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dec_main`]
module"]
#[doc(alias = "dec_main")]
pub type DecMain = crate::Reg<dec_main::DecMainSpec>;
#[doc = "Clock AXI Config 0 DEC Main"]
pub mod dec_main;
#[doc = "hifi4 (rw) register accessor: Clock AXI Config 0 DEC HIFI4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hifi4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hifi4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hifi4`]
module"]
#[doc(alias = "hifi4")]
pub type Hifi4 = crate::Reg<hifi4::Hifi4Spec>;
#[doc = "Clock AXI Config 0 DEC HIFI4"]
pub mod hifi4;
