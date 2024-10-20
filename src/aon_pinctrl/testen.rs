#[doc = "Register `testen` reader"]
pub type R = crate::R<TestenSpec>;
#[doc = "Register `testen` writer"]
pub type W = crate::W<TestenSpec>;
#[doc = "Field `testen` reader - Power-on-Start (POS) enabler - 0: Active pull-down capability disabled, 1: Enable active pull down for loss of core power."]
pub type TestenR = crate::BitReader;
#[doc = "Field `testen` writer - Power-on-Start (POS) enabler - 0: Active pull-down capability disabled, 1: Enable active pull down for loss of core power."]
pub type TestenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power-on-Start (POS) enabler - 0: Active pull-down capability disabled, 1: Enable active pull down for loss of core power."]
    #[inline(always)]
    pub fn testen(&self) -> TestenR {
        TestenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power-on-Start (POS) enabler - 0: Active pull-down capability disabled, 1: Enable active pull down for loss of core power."]
    #[inline(always)]
    #[must_use]
    pub fn testen(&mut self) -> TestenW<TestenSpec> {
        TestenW::new(self, 0)
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG 48 - Enable test Power-on-Start (POS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`testen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`testen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestenSpec;
impl crate::RegisterSpec for TestenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`testen::R`](R) reader structure"]
impl crate::Readable for TestenSpec {}
#[doc = "`write(|w| ..)` method takes [`testen::W`](W) writer structure"]
impl crate::Writable for TestenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets testen to value 0"]
impl crate::Resettable for TestenSpec {
    const RESET_VALUE: u32 = 0;
}
