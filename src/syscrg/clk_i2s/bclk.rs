#[doc = "Register `bclk` reader"]
pub type R = crate::R<BclkSpec>;
#[doc = "Register `bclk` writer"]
pub type W = crate::W<BclkSpec>;
#[doc = "Clock multiplexing selector: clk_i2s_bclk_mst, clk_i2s_bclk_ext\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_i2s_bclk_mst` as the I2S BCLK clock."]
    ClkI2sBclkMst = 0,
    #[doc = "1: Select `clk_i2s_bclk_ext` as the I2S BCLK clock."]
    ClkI2sBclkExt = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_i2s_bclk_mst, clk_i2s_bclk_ext"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkI2sBclkMst),
            1 => Some(ClkMuxSel::ClkI2sBclkExt),
            _ => None,
        }
    }
    #[doc = "Select `clk_i2s_bclk_mst` as the I2S BCLK clock."]
    #[inline(always)]
    pub fn is_clk_i2s_bclk_mst(&self) -> bool {
        *self == ClkMuxSel::ClkI2sBclkMst
    }
    #[doc = "Select `clk_i2s_bclk_ext` as the I2S BCLK clock."]
    #[inline(always)]
    pub fn is_clk_i2s_bclk_ext(&self) -> bool {
        *self == ClkMuxSel::ClkI2sBclkExt
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_i2s_bclk_mst, clk_i2s_bclk_ext"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_i2s_bclk_mst` as the I2S BCLK clock."]
    #[inline(always)]
    pub fn clk_i2s_bclk_mst(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkI2sBclkMst)
    }
    #[doc = "Select `clk_i2s_bclk_ext` as the I2S BCLK clock."]
    #[inline(always)]
    pub fn clk_i2s_bclk_ext(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkI2sBclkExt)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2s_bclk_mst, clk_i2s_bclk_ext"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2s_bclk_mst, clk_i2s_bclk_ext"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<BclkSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "Clock I2S BCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BclkSpec;
impl crate::RegisterSpec for BclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bclk::R`](R) reader structure"]
impl crate::Readable for BclkSpec {}
#[doc = "`write(|w| ..)` method takes [`bclk::W`](W) writer structure"]
impl crate::Writable for BclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bclk to value 0"]
impl crate::Resettable for BclkSpec {
    const RESET_VALUE: u32 = 0;
}
