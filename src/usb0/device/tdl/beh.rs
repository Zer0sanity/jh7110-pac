#[doc = "Register `beh` reader"]
pub type R = crate::R<BehSpec>;
#[doc = "Register `beh` writer"]
pub type W = crate::W<BehSpec>;
#[doc = "Field `beh` reader - TDL behavior configuration."]
pub type BehR = crate::FieldReader<u32>;
#[doc = "Field `beh` writer - TDL behavior configuration."]
pub type BehW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TDL behavior configuration."]
    #[inline(always)]
    pub fn beh(&self) -> BehR {
        BehR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TDL behavior configuration."]
    #[inline(always)]
    #[must_use]
    pub fn beh(&mut self) -> BehW<BehSpec> {
        BehW::new(self, 0)
    }
}
#[doc = "TDL behavior configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`beh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`beh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BehSpec;
impl crate::RegisterSpec for BehSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`beh::R`](R) reader structure"]
impl crate::Readable for BehSpec {}
#[doc = "`write(|w| ..)` method takes [`beh::W`](W) writer structure"]
impl crate::Writable for BehSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets beh to value 0"]
impl crate::Resettable for BehSpec {
    const RESET_VALUE: u32 = 0;
}
