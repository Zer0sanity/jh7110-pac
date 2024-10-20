#[doc = "Register `lrck_mst` reader"]
pub type R = crate::R<LrckMstSpec>;
#[doc = "Register `lrck_mst` writer"]
pub type W = crate::W<LrckMstSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=64, Default=64, Min=64, Typical=64"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=64, Default=64, Min=64, Typical=64"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Clock multiplexing selector: clk_i2stx_bclk_mst_inv, clk_i2stx_bclk_mst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_i2stx_bclk_mst_inv` as the I2S LRCK MST clock."]
    ClkI2stxBclkMstInv = 0,
    #[doc = "1: Select `clk_i2stx_bclk_mst` as the I2S LRCK MST clock."]
    ClkI2stxBclkMst = 1,
}
impl From<ClkMuxSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkMuxSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkMuxSel {
    type Ux = u8;
}
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_i2stx_bclk_mst_inv, clk_i2stx_bclk_mst"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkI2stxBclkMstInv),
            1 => Some(ClkMuxSel::ClkI2stxBclkMst),
            _ => None,
        }
    }
    #[doc = "Select `clk_i2stx_bclk_mst_inv` as the I2S LRCK MST clock."]
    #[inline(always)]
    pub fn is_clk_i2stx_bclk_mst_inv(&self) -> bool {
        *self == ClkMuxSel::ClkI2stxBclkMstInv
    }
    #[doc = "Select `clk_i2stx_bclk_mst` as the I2S LRCK MST clock."]
    #[inline(always)]
    pub fn is_clk_i2stx_bclk_mst(&self) -> bool {
        *self == ClkMuxSel::ClkI2stxBclkMst
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_i2stx_bclk_mst_inv, clk_i2stx_bclk_mst"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_i2stx_bclk_mst_inv` as the I2S LRCK MST clock."]
    #[inline(always)]
    pub fn clk_i2stx_bclk_mst_inv(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkI2stxBclkMstInv)
    }
    #[doc = "Select `clk_i2stx_bclk_mst` as the I2S LRCK MST clock."]
    #[inline(always)]
    pub fn clk_i2stx_bclk_mst(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkI2stxBclkMst)
    }
}
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=64, Default=64, Min=64, Typical=64"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2stx_bclk_mst_inv, clk_i2stx_bclk_mst"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=64, Default=64, Min=64, Typical=64"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<LrckMstSpec> {
        ClkDivcfgW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2stx_bclk_mst_inv, clk_i2stx_bclk_mst"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<LrckMstSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "Clock I2S LRCK MST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrck_mst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrck_mst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LrckMstSpec;
impl crate::RegisterSpec for LrckMstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lrck_mst::R`](R) reader structure"]
impl crate::Readable for LrckMstSpec {}
#[doc = "`write(|w| ..)` method takes [`lrck_mst::W`](W) writer structure"]
impl crate::Writable for LrckMstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lrck_mst to value 0x40"]
impl crate::Resettable for LrckMstSpec {
    const RESET_VALUE: u32 = 0x40;
}
