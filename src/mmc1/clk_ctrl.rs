#[repr(C)]
#[doc = "MMC Clock Control registers"]
#[doc(alias = "clk_ctrl")]
pub struct ClkCtrl {
    clk: [Clk; 2],
    clken: Clken,
}
impl ClkCtrl {
    #[doc = "0x00..0x08 - MMC Clock Configuration - 0: clkdiv, 1: clksrc"]
    #[inline(always)]
    pub const fn clk(&self, n: usize) -> &Clk {
        &self.clk[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - MMC Clock Configuration - 0: clkdiv, 1: clksrc"]
    #[inline(always)]
    pub fn clk_iter(&self) -> impl Iterator<Item = &Clk> {
        self.clk.iter()
    }
    #[doc = "0x00 - MMC Clock Configuration - 0: clkdiv, 1: clksrc"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clk {
        self.clk(0)
    }
    #[doc = "0x04 - MMC Clock Configuration - 0: clkdiv, 1: clksrc"]
    #[inline(always)]
    pub const fn clksrc(&self) -> &Clk {
        self.clk(1)
    }
    #[doc = "0x08 - MMC Clock Enable"]
    #[inline(always)]
    pub const fn clken(&self) -> &Clken {
        &self.clken
    }
}
#[doc = "clk (rw) register accessor: MMC Clock Configuration - 0: clkdiv, 1: clksrc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`]
module"]
#[doc(alias = "clk")]
pub type Clk = crate::Reg<clk::ClkSpec>;
#[doc = "MMC Clock Configuration - 0: clkdiv, 1: clksrc"]
pub mod clk;
#[doc = "clken (rw) register accessor: MMC Clock Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clken`]
module"]
#[doc(alias = "clken")]
pub type Clken = crate::Reg<clken::ClkenSpec>;
#[doc = "MMC Clock Enable"]
pub mod clken;
