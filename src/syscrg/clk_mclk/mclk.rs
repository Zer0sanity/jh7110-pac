#[doc = "Register `mclk` reader"]
pub type R = crate::R<MclkSpec>;
#[doc = "Register `mclk` writer"]
pub type W = crate::W<MclkSpec>;
#[doc = "Clock multiplexing selector: clk_mclk_inner, clk_mclk_ext\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_mclk_inner` as the MCLK clock."]
    ClkMclkInner = 0,
    #[doc = "1: Select `clk_mclk_ext` as the MCLK clock."]
    ClkMclkExt = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_mclk_inner, clk_mclk_ext"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkMclkInner),
            1 => Some(ClkMuxSel::ClkMclkExt),
            _ => None,
        }
    }
    #[doc = "Select `clk_mclk_inner` as the MCLK clock."]
    #[inline(always)]
    pub fn is_clk_mclk_inner(&self) -> bool {
        *self == ClkMuxSel::ClkMclkInner
    }
    #[doc = "Select `clk_mclk_ext` as the MCLK clock."]
    #[inline(always)]
    pub fn is_clk_mclk_ext(&self) -> bool {
        *self == ClkMuxSel::ClkMclkExt
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_mclk_inner, clk_mclk_ext"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_mclk_inner` as the MCLK clock."]
    #[inline(always)]
    pub fn clk_mclk_inner(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkMclkInner)
    }
    #[doc = "Select `clk_mclk_ext` as the MCLK clock."]
    #[inline(always)]
    pub fn clk_mclk_ext(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkMclkExt)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_mclk_inner, clk_mclk_ext"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_mclk_inner, clk_mclk_ext"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<MclkSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "Clock MCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MclkSpec;
impl crate::RegisterSpec for MclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mclk::R`](R) reader structure"]
impl crate::Readable for MclkSpec {}
#[doc = "`write(|w| ..)` method takes [`mclk::W`](W) writer structure"]
impl crate::Writable for MclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mclk to value 0"]
impl crate::Resettable for MclkSpec {
    const RESET_VALUE: u32 = 0;
}
