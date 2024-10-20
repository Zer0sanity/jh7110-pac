#[doc = "Register `clk_aon_apb` reader"]
pub type R = crate::R<ClkAonApbSpec>;
#[doc = "Register `clk_aon_apb` writer"]
pub type W = crate::W<ClkAonApbSpec>;
#[doc = "Clock multiplexing selector: clk_osc_div4, clk_osc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_osc_div4` as AON APB clock."]
    ClkOscDiv4 = 0,
    #[doc = "1: Select `clk_osc` as AON APB clock."]
    ClkOsc = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_osc_div4, clk_osc"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkOscDiv4),
            1 => Some(ClkMuxSel::ClkOsc),
            _ => None,
        }
    }
    #[doc = "Select `clk_osc_div4` as AON APB clock."]
    #[inline(always)]
    pub fn is_clk_osc_div4(&self) -> bool {
        *self == ClkMuxSel::ClkOscDiv4
    }
    #[doc = "Select `clk_osc` as AON APB clock."]
    #[inline(always)]
    pub fn is_clk_osc(&self) -> bool {
        *self == ClkMuxSel::ClkOsc
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_osc_div4, clk_osc"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_osc_div4` as AON APB clock."]
    #[inline(always)]
    pub fn clk_osc_div4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkOscDiv4)
    }
    #[doc = "Select `clk_osc` as AON APB clock."]
    #[inline(always)]
    pub fn clk_osc(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkOsc)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc_div4, clk_osc"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc_div4, clk_osc"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<ClkAonApbSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "AON APB Function Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_aon_apb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_aon_apb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkAonApbSpec;
impl crate::RegisterSpec for ClkAonApbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_aon_apb::R`](R) reader structure"]
impl crate::Readable for ClkAonApbSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_aon_apb::W`](W) writer structure"]
impl crate::Writable for ClkAonApbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_aon_apb to value 0"]
impl crate::Resettable for ClkAonApbSpec {
    const RESET_VALUE: u32 = 0;
}
