#[doc = "Register `clk_u0_vin_pixel_clk_if[%s]` reader"]
pub type R = crate::R<ClkU0VinPixelClkIfSpec>;
#[doc = "Register `clk_u0_vin_pixel_clk_if[%s]` writer"]
pub type W = crate::W<ClkU0VinPixelClkIfSpec>;
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
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    pub fn clk_icg(&self) -> ClkIcgR {
        ClkIcgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> ClkIcgW<ClkU0VinPixelClkIfSpec> {
        ClkIcgW::new(self, 31)
    }
}
#[doc = "Clock Video In Pixel Clock Interfaces\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_vin_pixel_clk_if::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_vin_pixel_clk_if::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkU0VinPixelClkIfSpec;
impl crate::RegisterSpec for ClkU0VinPixelClkIfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_u0_vin_pixel_clk_if::R`](R) reader structure"]
impl crate::Readable for ClkU0VinPixelClkIfSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_u0_vin_pixel_clk_if::W`](W) writer structure"]
impl crate::Writable for ClkU0VinPixelClkIfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_u0_vin_pixel_clk_if[%s]
to value 0x8000_0000"]
impl crate::Resettable for ClkU0VinPixelClkIfSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
