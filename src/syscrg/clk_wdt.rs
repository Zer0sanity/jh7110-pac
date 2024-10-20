#[repr(C)]
#[doc = "Clock WDT registers"]
#[doc(alias = "clk_wdt")]
pub struct ClkWdt {
    apb: Apb,
    wdt: Wdt,
}
impl ClkWdt {
    #[doc = "0x00 - Clock WDT APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x04 - Clock WDT"]
    #[inline(always)]
    pub const fn wdt(&self) -> &Wdt {
        &self.wdt
    }
}
#[doc = "apb (rw) register accessor: Clock WDT APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock WDT APB"]
pub mod apb;
#[doc = "wdt (rw) register accessor: Clock WDT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt`]
module"]
#[doc(alias = "wdt")]
pub type Wdt = crate::Reg<wdt::WdtSpec>;
#[doc = "Clock WDT"]
pub mod wdt;
