#[doc = "Register `clk_ref` reader"]
pub type R = crate::R<ClkRefSpec>;
#[doc = "Register `clk_ref` writer"]
pub type W = crate::W<ClkRefSpec>;
#[doc = "Clock multiplexing selector: clk_osc, clk_qspi_ref_src\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_osc` as the QSPI Reference clock."]
    ClkOsc = 0,
    #[doc = "1: Select `clk_qspi_ref_src` as the QSPI Reference clock."]
    ClkQspiRefSrc = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_osc, clk_qspi_ref_src"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkOsc),
            1 => Some(ClkMuxSel::ClkQspiRefSrc),
            _ => None,
        }
    }
    #[doc = "Select `clk_osc` as the QSPI Reference clock."]
    #[inline(always)]
    pub fn is_clk_osc(&self) -> bool {
        *self == ClkMuxSel::ClkOsc
    }
    #[doc = "Select `clk_qspi_ref_src` as the QSPI Reference clock."]
    #[inline(always)]
    pub fn is_clk_qspi_ref_src(&self) -> bool {
        *self == ClkMuxSel::ClkQspiRefSrc
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_osc, clk_qspi_ref_src"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_osc` as the QSPI Reference clock."]
    #[inline(always)]
    pub fn clk_osc(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkOsc)
    }
    #[doc = "Select `clk_qspi_ref_src` as the QSPI Reference clock."]
    #[inline(always)]
    pub fn clk_qspi_ref_src(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkQspiRefSrc)
    }
}
#[doc = "Clock ICG enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkIcg {
    #[doc = "0: Disable the clock."]
    Disable = 0,
    #[doc = "1: Enable the clock."]
    Enable = 1,
}
impl From<ClkIcg> for bool {
    #[inline(always)]
    fn from(variant: ClkIcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clk_icg` reader - Clock ICG enable."]
pub type ClkIcgR = crate::BitReader<ClkIcg>;
impl ClkIcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkIcg {
        match self.bits {
            false => ClkIcg::Disable,
            true => ClkIcg::Enable,
        }
    }
    #[doc = "Disable the clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ClkIcg::Disable
    }
    #[doc = "Enable the clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ClkIcg::Enable
    }
}
#[doc = "Field `clk_icg` writer - Clock ICG enable."]
pub type ClkIcgW<'a, REG> = crate::BitWriter<'a, REG, ClkIcg>;
impl<'a, REG> ClkIcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIcg::Disable)
    }
    #[doc = "Enable the clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIcg::Enable)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc, clk_qspi_ref_src"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    pub fn clk_icg(&self) -> ClkIcgR {
        ClkIcgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc, clk_qspi_ref_src"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<ClkRefSpec> {
        ClkMuxSelW::new(self, 24)
    }
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> ClkIcgW<ClkRefSpec> {
        ClkIcgW::new(self, 31)
    }
}
#[doc = "Clock QSPI Reference\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkRefSpec;
impl crate::RegisterSpec for ClkRefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ref::R`](R) reader structure"]
impl crate::Readable for ClkRefSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ref::W`](W) writer structure"]
impl crate::Writable for ClkRefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_ref to value 0x8000_0000"]
impl crate::Resettable for ClkRefSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
