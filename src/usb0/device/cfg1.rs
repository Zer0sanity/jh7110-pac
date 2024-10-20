#[doc = "Register `cfg1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `cfg1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `cfg1` reader - Device configuration 1."]
pub type Cfg1R = crate::FieldReader<u32>;
#[doc = "Field `cfg1` writer - Device configuration 1."]
pub type Cfg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device configuration 1."]
    #[inline(always)]
    pub fn cfg1(&self) -> Cfg1R {
        Cfg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device configuration 1."]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> Cfg1W<Cfg1Spec> {
        Cfg1W::new(self, 0)
    }
}
#[doc = "Device configuration 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cfg1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
