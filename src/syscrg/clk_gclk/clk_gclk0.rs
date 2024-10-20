#[doc = "Register `clk_gclk0` reader"]
pub type R = crate::R<ClkGclk0Spec>;
#[doc = "Register `clk_gclk0` writer"]
pub type W = crate::W<ClkGclk0Spec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=62, Default=20, Min=16, Typical=20"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=62, Default=20, Min=16, Typical=20"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
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
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=62, Default=20, Min=16, Typical=20"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    pub fn clk_icg(&self) -> ClkIcgR {
        ClkIcgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=62, Default=20, Min=16, Typical=20"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkGclk0Spec> {
        ClkDivcfgW::new(self, 0)
    }
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> ClkIcgW<ClkGclk0Spec> {
        ClkIcgW::new(self, 31)
    }
}
#[doc = "Clock GCLK 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gclk0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gclk0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkGclk0Spec;
impl crate::RegisterSpec for ClkGclk0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gclk0::R`](R) reader structure"]
impl crate::Readable for ClkGclk0Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_gclk0::W`](W) writer structure"]
impl crate::Writable for ClkGclk0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_gclk0 to value 0x8000_0014"]
impl crate::Resettable for ClkGclk0Spec {
    const RESET_VALUE: u32 = 0x8000_0014;
}