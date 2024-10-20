#[doc = "Register `way_enable` reader"]
pub type R = crate::R<WayEnableSpec>;
#[doc = "Register `way_enable` writer"]
pub type W = crate::W<WayEnableSpec>;
#[doc = "Field `way_enable` reader - The index of the largest way which has been enabled. May only be increased."]
pub type WayEnableR = crate::FieldReader;
#[doc = "Field `way_enable` writer - The index of the largest way which has been enabled. May only be increased."]
pub type WayEnableW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The index of the largest way which has been enabled. May only be increased."]
    #[inline(always)]
    pub fn way_enable(&self) -> WayEnableR {
        WayEnableR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The index of the largest way which has been enabled. May only be increased."]
    #[inline(always)]
    #[must_use]
    pub fn way_enable(&mut self) -> WayEnableW<WayEnableSpec> {
        WayEnableW::new(self, 0)
    }
}
#[doc = "L2 Cache Control Way Enable register. Determines which ways of the Level 2 Cache Controller are enabled as cache.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`way_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`way_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WayEnableSpec;
impl crate::RegisterSpec for WayEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`way_enable::R`](R) reader structure"]
impl crate::Readable for WayEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`way_enable::W`](W) writer structure"]
impl crate::Writable for WayEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets way_enable to value 0"]
impl crate::Resettable for WayEnableSpec {
    const RESET_VALUE: u32 = 0;
}
