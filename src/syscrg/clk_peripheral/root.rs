#[doc = "Register `root` reader"]
pub type R = crate::R<RootSpec>;
#[doc = "Register `root` writer"]
pub type W = crate::W<RootSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Clock multiplexing selector: clk_pll2, clk_pll1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_pll2` as the Peripheral Root clock"]
    ClkPll2 = 0,
    #[doc = "1: Select `clk_pll1` as the Peripheral Root clock"]
    ClkPll1 = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_pll2, clk_pll1"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkPll2),
            1 => Some(ClkMuxSel::ClkPll1),
            _ => None,
        }
    }
    #[doc = "Select `clk_pll2` as the Peripheral Root clock"]
    #[inline(always)]
    pub fn is_clk_pll2(&self) -> bool {
        *self == ClkMuxSel::ClkPll2
    }
    #[doc = "Select `clk_pll1` as the Peripheral Root clock"]
    #[inline(always)]
    pub fn is_clk_pll1(&self) -> bool {
        *self == ClkMuxSel::ClkPll1
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_pll2, clk_pll1"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_pll2` as the Peripheral Root clock"]
    #[inline(always)]
    pub fn clk_pll2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkPll2)
    }
    #[doc = "Select `clk_pll1` as the Peripheral Root clock"]
    #[inline(always)]
    pub fn clk_pll1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkPll1)
    }
}
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_pll2, clk_pll1"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=2, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<RootSpec> {
        ClkDivcfgW::new(self, 0)
    }
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_pll2, clk_pll1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<RootSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "Clock Peripheral Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RootSpec;
impl crate::RegisterSpec for RootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root::R`](R) reader structure"]
impl crate::Readable for RootSpec {}
#[doc = "`write(|w| ..)` method takes [`root::W`](W) writer structure"]
impl crate::Writable for RootSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets root to value 0x02"]
impl crate::Resettable for RootSpec {
    const RESET_VALUE: u32 = 0x02;
}
