#[doc = "Register `tdm` reader"]
pub type R = crate::R<TdmSpec>;
#[doc = "Register `tdm` writer"]
pub type W = crate::W<TdmSpec>;
#[doc = "Clock multiplexing selector: clk_tdm_internal, clk_tdm_ext\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_tdm_internal` as the TDM clock."]
    ClkTdmInternal = 0,
    #[doc = "1: Select `clk_tdm_ext` as the TDM clock."]
    ClkTdmExt = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_tdm_internal, clk_tdm_ext"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkTdmInternal),
            1 => Some(ClkMuxSel::ClkTdmExt),
            _ => None,
        }
    }
    #[doc = "Select `clk_tdm_internal` as the TDM clock."]
    #[inline(always)]
    pub fn is_clk_tdm_internal(&self) -> bool {
        *self == ClkMuxSel::ClkTdmInternal
    }
    #[doc = "Select `clk_tdm_ext` as the TDM clock."]
    #[inline(always)]
    pub fn is_clk_tdm_ext(&self) -> bool {
        *self == ClkMuxSel::ClkTdmExt
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_tdm_internal, clk_tdm_ext"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_tdm_internal` as the TDM clock."]
    #[inline(always)]
    pub fn clk_tdm_internal(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkTdmInternal)
    }
    #[doc = "Select `clk_tdm_ext` as the TDM clock."]
    #[inline(always)]
    pub fn clk_tdm_ext(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkTdmExt)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_tdm_internal, clk_tdm_ext"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_tdm_internal, clk_tdm_ext"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<TdmSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "Clock TDM (clock selector)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdmSpec;
impl crate::RegisterSpec for TdmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdm::R`](R) reader structure"]
impl crate::Readable for TdmSpec {}
#[doc = "`write(|w| ..)` method takes [`tdm::W`](W) writer structure"]
impl crate::Writable for TdmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tdm to value 0"]
impl crate::Resettable for TdmSpec {
    const RESET_VALUE: u32 = 0;
}
