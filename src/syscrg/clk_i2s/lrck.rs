#[doc = "Register `lrck` reader"]
pub type R = crate::R<LrckSpec>;
#[doc = "Register `lrck` writer"]
pub type W = crate::W<LrckSpec>;
#[doc = "Clock multiplexing selector: clk_i2s_lrck_mst, clk_i2s_lrck_ext\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_i2s_lrck_mst` as the I2S LRCK clock."]
    ClkI2sLrckMst = 0,
    #[doc = "1: Select `clk_i2s_lrck_ext` as the I2S LRCK clock."]
    ClkI2sLrckExt = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_i2s_lrck_mst, clk_i2s_lrck_ext"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkI2sLrckMst),
            1 => Some(ClkMuxSel::ClkI2sLrckExt),
            _ => None,
        }
    }
    #[doc = "Select `clk_i2s_lrck_mst` as the I2S LRCK clock."]
    #[inline(always)]
    pub fn is_clk_i2s_lrck_mst(&self) -> bool {
        *self == ClkMuxSel::ClkI2sLrckMst
    }
    #[doc = "Select `clk_i2s_lrck_ext` as the I2S LRCK clock."]
    #[inline(always)]
    pub fn is_clk_i2s_lrck_ext(&self) -> bool {
        *self == ClkMuxSel::ClkI2sLrckExt
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_i2s_lrck_mst, clk_i2s_lrck_ext"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_i2s_lrck_mst` as the I2S LRCK clock."]
    #[inline(always)]
    pub fn clk_i2s_lrck_mst(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkI2sLrckMst)
    }
    #[doc = "Select `clk_i2s_lrck_ext` as the I2S LRCK clock."]
    #[inline(always)]
    pub fn clk_i2s_lrck_ext(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkI2sLrckExt)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2s_lrck_mst, clk_i2s_lrck_ext"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_i2s_lrck_mst, clk_i2s_lrck_ext"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<LrckSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "Clock I2S LRCK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lrck::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lrck::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LrckSpec;
impl crate::RegisterSpec for LrckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lrck::R`](R) reader structure"]
impl crate::Readable for LrckSpec {}
#[doc = "`write(|w| ..)` method takes [`lrck::W`](W) writer structure"]
impl crate::Writable for LrckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lrck to value 0"]
impl crate::Resettable for LrckSpec {
    const RESET_VALUE: u32 = 0;
}
