#[doc = "Register `gmac1[%s]` reader"]
pub type R = crate::R<Gmac1Spec>;
#[doc = "Register `gmac1[%s]` writer"]
pub type W = crate::W<Gmac1Spec>;
#[doc = "Field `cfg` reader - "]
pub type CfgR = crate::FieldReader;
#[doc = "Field `cfg` writer - "]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<Gmac1Spec> {
        CfgW::new(self, 0)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG PADCFG: GPIO GMAC1 pads\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gmac1Spec;
impl crate::RegisterSpec for Gmac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac1::R`](R) reader structure"]
impl crate::Readable for Gmac1Spec {}
#[doc = "`write(|w| ..)` method takes [`gmac1::W`](W) writer structure"]
impl crate::Writable for Gmac1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gmac1[%s]
to value 0x02"]
impl crate::Resettable for Gmac1Spec {
    const RESET_VALUE: u32 = 0x02;
}
