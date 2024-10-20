#[doc = "Register `bus` reader"]
pub type R = crate::R<BusSpec>;
#[doc = "Register `bus` writer"]
pub type W = crate::W<BusSpec>;
#[doc = "Clock multiplexing selector: clk_osc_div2, clk_pll1_div2, clk_pll1_div4, clk_pll1_div8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_osc_div2` as the DDR Bus clock."]
    ClkOscDiv2 = 0,
    #[doc = "1: Select `clk_pll1_div2` as the DDR Bus clock."]
    ClkPll1Div2 = 1,
    #[doc = "2: Select `clk_pll1_div4` as the DDR Bus clock."]
    ClkPll1Div4 = 2,
    #[doc = "3: Select `clk_pll1_div8` as the DDR Bus clock."]
    ClkPll1Div8 = 3,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_osc_div2, clk_pll1_div2, clk_pll1_div4, clk_pll1_div8"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkOscDiv2),
            1 => Some(ClkMuxSel::ClkPll1Div2),
            2 => Some(ClkMuxSel::ClkPll1Div4),
            3 => Some(ClkMuxSel::ClkPll1Div8),
            _ => None,
        }
    }
    #[doc = "Select `clk_osc_div2` as the DDR Bus clock."]
    #[inline(always)]
    pub fn is_clk_osc_div2(&self) -> bool {
        *self == ClkMuxSel::ClkOscDiv2
    }
    #[doc = "Select `clk_pll1_div2` as the DDR Bus clock."]
    #[inline(always)]
    pub fn is_clk_pll1_div2(&self) -> bool {
        *self == ClkMuxSel::ClkPll1Div2
    }
    #[doc = "Select `clk_pll1_div4` as the DDR Bus clock."]
    #[inline(always)]
    pub fn is_clk_pll1_div4(&self) -> bool {
        *self == ClkMuxSel::ClkPll1Div4
    }
    #[doc = "Select `clk_pll1_div8` as the DDR Bus clock."]
    #[inline(always)]
    pub fn is_clk_pll1_div8(&self) -> bool {
        *self == ClkMuxSel::ClkPll1Div8
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_osc_div2, clk_pll1_div2, clk_pll1_div4, clk_pll1_div8"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_osc_div2` as the DDR Bus clock."]
    #[inline(always)]
    pub fn clk_osc_div2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkOscDiv2)
    }
    #[doc = "Select `clk_pll1_div2` as the DDR Bus clock."]
    #[inline(always)]
    pub fn clk_pll1_div2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkPll1Div2)
    }
    #[doc = "Select `clk_pll1_div4` as the DDR Bus clock."]
    #[inline(always)]
    pub fn clk_pll1_div4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkPll1Div4)
    }
    #[doc = "Select `clk_pll1_div8` as the DDR Bus clock."]
    #[inline(always)]
    pub fn clk_pll1_div8(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkPll1Div8)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc_div2, clk_pll1_div2, clk_pll1_div4, clk_pll1_div8"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_osc_div2, clk_pll1_div2, clk_pll1_div4, clk_pll1_div8"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<BusSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "clk_ddr_bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusSpec;
impl crate::RegisterSpec for BusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus::R`](R) reader structure"]
impl crate::Readable for BusSpec {}
#[doc = "`write(|w| ..)` method takes [`bus::W`](W) writer structure"]
impl crate::Writable for BusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bus to value 0"]
impl crate::Resettable for BusSpec {
    const RESET_VALUE: u32 = 0;
}
