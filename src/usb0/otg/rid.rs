#[doc = "Register `rid` reader"]
pub type R = crate::R<RidSpec>;
#[doc = "Register `rid` writer"]
pub type W = crate::W<RidSpec>;
#[doc = "Field `rid` reader - USB3 OTG RID."]
pub type RidR = crate::FieldReader<u16>;
#[doc = "Field `rid` writer - USB3 OTG RID."]
pub type RidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - USB3 OTG RID."]
    #[inline(always)]
    pub fn rid(&self) -> RidR {
        RidR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - USB3 OTG RID."]
    #[inline(always)]
    #[must_use]
    pub fn rid(&mut self) -> RidW<RidSpec> {
        RidW::new(self, 0)
    }
}
#[doc = "USB3 OTG RID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RidSpec;
impl crate::RegisterSpec for RidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rid::R`](R) reader structure"]
impl crate::Readable for RidSpec {}
#[doc = "`write(|w| ..)` method takes [`rid::W`](W) writer structure"]
impl crate::Writable for RidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rid to value 0"]
impl crate::Resettable for RidSpec {
    const RESET_VALUE: u32 = 0;
}
