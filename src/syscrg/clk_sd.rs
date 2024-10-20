#[repr(C)]
#[doc = "Clock SD registers"]
#[doc(alias = "clk_sd")]
pub struct ClkSd {
    ahb: [Ahb; 2],
    clk_sd_card: [ClkSdCard; 2],
}
impl ClkSd {
    #[doc = "0x00..0x08 - Clock SD AHB"]
    #[inline(always)]
    pub const fn ahb(&self, n: usize) -> &Ahb {
        &self.ahb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Clock SD AHB"]
    #[inline(always)]
    pub fn ahb_iter(&self) -> impl Iterator<Item = &Ahb> {
        self.ahb.iter()
    }
    #[doc = "0x00 - Clock SD AHB"]
    #[inline(always)]
    pub const fn ahb_u0(&self) -> &Ahb {
        self.ahb(0)
    }
    #[doc = "0x04 - Clock SD AHB"]
    #[inline(always)]
    pub const fn ahb_u1(&self) -> &Ahb {
        self.ahb(1)
    }
    #[doc = "0x08..0x10 - Clock SD Card"]
    #[inline(always)]
    pub const fn clk_sd_card(&self, n: usize) -> &ClkSdCard {
        &self.clk_sd_card[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x10 - Clock SD Card"]
    #[inline(always)]
    pub fn clk_sd_card_iter(&self) -> impl Iterator<Item = &ClkSdCard> {
        self.clk_sd_card.iter()
    }
    #[doc = "0x08 - Clock SD Card"]
    #[inline(always)]
    pub const fn clk_sd_card_u0(&self) -> &ClkSdCard {
        self.clk_sd_card(0)
    }
    #[doc = "0x0c - Clock SD Card"]
    #[inline(always)]
    pub const fn clk_sd_card_u1(&self) -> &ClkSdCard {
        self.clk_sd_card(1)
    }
}
#[doc = "ahb (rw) register accessor: Clock SD AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb`]
module"]
#[doc(alias = "ahb")]
pub type Ahb = crate::Reg<ahb::AhbSpec>;
#[doc = "Clock SD AHB"]
pub mod ahb;
#[doc = "clk_sd_card (rw) register accessor: Clock SD Card\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sd_card::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sd_card::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sd_card`]
module"]
#[doc(alias = "clk_sd_card")]
pub type ClkSdCard = crate::Reg<clk_sd_card::ClkSdCardSpec>;
#[doc = "Clock SD Card"]
pub mod clk_sd_card;
