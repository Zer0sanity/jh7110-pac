#[doc = "Register `wave511_vce` reader"]
pub type R = crate::R<Wave511VceSpec>;
#[doc = "Register `wave511_vce` writer"]
pub type W = crate::W<Wave511VceSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=7, Default=2, Min=3, Typical=2"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=7, Default=2, Min=3, Typical=2"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Clock ICG enable.\n\nValue on reset: 0"]
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
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=7, Default=2, Min=3, Typical=2"]
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
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=7, Default=2, Min=3, Typical=2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<Wave511VceSpec> {
        ClkDivcfgW::new(self, 0)
    }
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> ClkIcgW<Wave511VceSpec> {
        ClkIcgW::new(self, 31)
    }
}
#[doc = "Clock WAVE511 VCE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wave511_vce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wave511_vce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wave511VceSpec;
impl crate::RegisterSpec for Wave511VceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wave511_vce::R`](R) reader structure"]
impl crate::Readable for Wave511VceSpec {}
#[doc = "`write(|w| ..)` method takes [`wave511_vce::W`](W) writer structure"]
impl crate::Writable for Wave511VceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wave511_vce to value 0x02"]
impl crate::Resettable for Wave511VceSpec {
    const RESET_VALUE: u32 = 0x02;
}
