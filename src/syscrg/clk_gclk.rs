#[repr(C)]
#[doc = "Clock GCLK registers"]
#[doc(alias = "clk_gclk")]
pub struct ClkGclk {
    clk_gclk0: ClkGclk0,
    clk_gclk1: ClkGclk1,
    clk_gclk2: ClkGclk2,
}
impl ClkGclk {
    #[doc = "0x00 - Clock GCLK 0"]
    #[inline(always)]
    pub const fn clk_gclk0(&self) -> &ClkGclk0 {
        &self.clk_gclk0
    }
    #[doc = "0x04 - Clock GCLK 1"]
    #[inline(always)]
    pub const fn clk_gclk1(&self) -> &ClkGclk1 {
        &self.clk_gclk1
    }
    #[doc = "0x08 - Clock GCLK 2"]
    #[inline(always)]
    pub const fn clk_gclk2(&self) -> &ClkGclk2 {
        &self.clk_gclk2
    }
}
#[doc = "clk_gclk0 (rw) register accessor: Clock GCLK 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gclk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gclk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gclk0`]
module"]
#[doc(alias = "clk_gclk0")]
pub type ClkGclk0 = crate::Reg<clk_gclk0::ClkGclk0Spec>;
#[doc = "Clock GCLK 0"]
pub mod clk_gclk0;
#[doc = "clk_gclk1 (rw) register accessor: Clock GCLK 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gclk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gclk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gclk1`]
module"]
#[doc(alias = "clk_gclk1")]
pub type ClkGclk1 = crate::Reg<clk_gclk1::ClkGclk1Spec>;
#[doc = "Clock GCLK 1"]
pub mod clk_gclk1;
#[doc = "clk_gclk2 (rw) register accessor: Clock GCLK 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gclk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gclk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gclk2`]
module"]
#[doc(alias = "clk_gclk2")]
pub type ClkGclk2 = crate::Reg<clk_gclk2::ClkGclk2Spec>;
#[doc = "Clock GCLK 2"]
pub mod clk_gclk2;
