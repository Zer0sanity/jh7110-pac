#[doc = "Register `beh2` reader"]
pub type R = crate::R<Beh2Spec>;
#[doc = "Register `beh2` writer"]
pub type W = crate::W<Beh2Spec>;
#[doc = "Field `beh2` reader - TDL behavior 2 configuration."]
pub type Beh2R = crate::FieldReader<u32>;
#[doc = "Field `beh2` writer - TDL behavior 2 configuration."]
pub type Beh2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TDL behavior 2 configuration."]
    #[inline(always)]
    pub fn beh2(&self) -> Beh2R {
        Beh2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TDL behavior 2 configuration."]
    #[inline(always)]
    #[must_use]
    pub fn beh2(&mut self) -> Beh2W<Beh2Spec> {
        Beh2W::new(self, 0)
    }
}
#[doc = "TDL behavior 2 configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`beh2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`beh2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Beh2Spec;
impl crate::RegisterSpec for Beh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`beh2::R`](R) reader structure"]
impl crate::Readable for Beh2Spec {}
#[doc = "`write(|w| ..)` method takes [`beh2::W`](W) writer structure"]
impl crate::Writable for Beh2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets beh2 to value 0"]
impl crate::Resettable for Beh2Spec {
    const RESET_VALUE: u32 = 0;
}
