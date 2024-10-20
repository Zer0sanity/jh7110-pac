#[repr(C)]
#[doc = "Clock I2S"]
#[doc(alias = "clk_i2s")]
pub struct ClkI2s {
    apb: Apb,
    bclk_mst: BclkMst,
    bclk_mst_inv: BclkMstInv,
    lrck_mst: LrckMst,
    bclk: Bclk,
    bclk_neg: BclkNeg,
    lrck: Lrck,
}
impl ClkI2s {
    #[doc = "0x00 - Clock I2S APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x04 - Clock I2S BCLK MST"]
    #[inline(always)]
    pub const fn bclk_mst(&self) -> &BclkMst {
        &self.bclk_mst
    }
    #[doc = "0x08 - U0 Clock I2S BCLK MST Inverter"]
    #[inline(always)]
    pub const fn bclk_mst_inv(&self) -> &BclkMstInv {
        &self.bclk_mst_inv
    }
    #[doc = "0x0c - Clock I2S LRCK MST"]
    #[inline(always)]
    pub const fn lrck_mst(&self) -> &LrckMst {
        &self.lrck_mst
    }
    #[doc = "0x10 - Clock I2S BCLK"]
    #[inline(always)]
    pub const fn bclk(&self) -> &Bclk {
        &self.bclk
    }
    #[doc = "0x14 - Clock I2S BCLK Negative"]
    #[inline(always)]
    pub const fn bclk_neg(&self) -> &BclkNeg {
        &self.bclk_neg
    }
    #[doc = "0x18 - Clock I2S LRCK"]
    #[inline(always)]
    pub const fn lrck(&self) -> &Lrck {
        &self.lrck
    }
}
#[doc = "apb (rw) register accessor: Clock I2S APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock I2S APB"]
pub mod apb;
#[doc = "bclk_mst (rw) register accessor: Clock I2S BCLK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bclk_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bclk_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bclk_mst`]
module"]
#[doc(alias = "bclk_mst")]
pub type BclkMst = crate::Reg<bclk_mst::BclkMstSpec>;
#[doc = "Clock I2S BCLK MST"]
pub mod bclk_mst;
#[doc = "bclk_mst_inv (rw) register accessor: U0 Clock I2S BCLK MST Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bclk_mst_inv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bclk_mst_inv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bclk_mst_inv`]
module"]
#[doc(alias = "bclk_mst_inv")]
pub type BclkMstInv = crate::Reg<bclk_mst_inv::BclkMstInvSpec>;
#[doc = "U0 Clock I2S BCLK MST Inverter"]
pub mod bclk_mst_inv;
#[doc = "lrck_mst (rw) register accessor: Clock I2S LRCK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrck_mst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrck_mst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrck_mst`]
module"]
#[doc(alias = "lrck_mst")]
pub type LrckMst = crate::Reg<lrck_mst::LrckMstSpec>;
#[doc = "Clock I2S LRCK MST"]
pub mod lrck_mst;
#[doc = "bclk (rw) register accessor: Clock I2S BCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bclk`]
module"]
#[doc(alias = "bclk")]
pub type Bclk = crate::Reg<bclk::BclkSpec>;
#[doc = "Clock I2S BCLK"]
pub mod bclk;
#[doc = "bclk_neg (rw) register accessor: Clock I2S BCLK Negative\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bclk_neg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bclk_neg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bclk_neg`]
module"]
#[doc(alias = "bclk_neg")]
pub type BclkNeg = crate::Reg<bclk_neg::BclkNegSpec>;
#[doc = "Clock I2S BCLK Negative"]
pub mod bclk_neg;
#[doc = "lrck (rw) register accessor: Clock I2S LRCK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lrck`]
module"]
#[doc(alias = "lrck")]
pub type Lrck = crate::Reg<lrck::LrckSpec>;
#[doc = "Clock I2S LRCK"]
pub mod lrck;
