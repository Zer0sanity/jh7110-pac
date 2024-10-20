#[doc = "Register `load` reader"]
pub type R = crate::R<LoadSpec>;
#[doc = "Register `load` writer"]
pub type W = crate::W<LoadSpec>;
#[doc = "Field `load` reader - Watchdog Load value"]
pub type LoadR = crate::FieldReader<u32>;
#[doc = "Field `load` writer - Watchdog Load value"]
pub type LoadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Watchdog Load value"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Load value"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LoadW<LoadSpec> {
        LoadW::new(self, 0)
    }
}
#[doc = "StarFive JH7110 Watchdog Load register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoadSpec;
impl crate::RegisterSpec for LoadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load::R`](R) reader structure"]
impl crate::Readable for LoadSpec {}
#[doc = "`write(|w| ..)` method takes [`load::W`](W) writer structure"]
impl crate::Writable for LoadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets load to value 0"]
impl crate::Resettable for LoadSpec {
    const RESET_VALUE: u32 = 0;
}
