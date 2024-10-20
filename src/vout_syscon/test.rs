#[doc = "Register `test[%s]` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `test[%s]` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `test` reader - "]
pub type TestR = crate::FieldReader<u32>;
#[doc = "Field `test` writer - "]
pub type TestW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TestW<TestSpec> {
        TestW::new(self, 0)
    }
}
#[doc = "VOUT SYSCFG 3-6: dom_vout_sysconsaif_12 - dom_vout_sysonsaif_24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets test[%s]
to value 0"]
impl crate::Resettable for TestSpec {
    const RESET_VALUE: u32 = 0;
}
