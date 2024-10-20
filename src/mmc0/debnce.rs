#[doc = "Register `debnce` reader"]
pub type R = crate::R<DebnceSpec>;
#[doc = "Register `debnce` writer"]
pub type W = crate::W<DebnceSpec>;
#[doc = "Field `debnce` reader - MMC debounce"]
pub type DebnceR = crate::FieldReader<u32>;
#[doc = "Field `debnce` writer - MMC debounce"]
pub type DebnceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC debounce"]
    #[inline(always)]
    pub fn debnce(&self) -> DebnceR {
        DebnceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC debounce"]
    #[inline(always)]
    #[must_use]
    pub fn debnce(&mut self) -> DebnceW<DebnceSpec> {
        DebnceW::new(self, 0)
    }
}
#[doc = "MMC debounce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debnce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debnce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebnceSpec;
impl crate::RegisterSpec for DebnceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debnce::R`](R) reader structure"]
impl crate::Readable for DebnceSpec {}
#[doc = "`write(|w| ..)` method takes [`debnce::W`](W) writer structure"]
impl crate::Writable for DebnceSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets debnce to value 0"]
impl crate::Resettable for DebnceSpec {
    const RESET_VALUE: u32 = 0;
}
