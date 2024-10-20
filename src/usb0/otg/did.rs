#[doc = "Register `did` reader"]
pub type R = crate::R<DidSpec>;
#[doc = "Register `did` writer"]
pub type W = crate::W<DidSpec>;
#[doc = "Field `did` reader - USB3 OTG VID."]
pub type DidR = crate::FieldReader<u32>;
#[doc = "Field `did` writer - USB3 OTG VID."]
pub type DidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 OTG VID."]
    #[inline(always)]
    pub fn did(&self) -> DidR {
        DidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB3 OTG VID."]
    #[inline(always)]
    #[must_use]
    pub fn did(&mut self) -> DidW<DidSpec> {
        DidW::new(self, 0)
    }
}
#[doc = "USB3 OTG VID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`did::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`did::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DidSpec;
impl crate::RegisterSpec for DidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`did::R`](R) reader structure"]
impl crate::Readable for DidSpec {}
#[doc = "`write(|w| ..)` method takes [`did::W`](W) writer structure"]
impl crate::Writable for DidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets did to value 0"]
impl crate::Resettable for DidSpec {
    const RESET_VALUE: u32 = 0;
}
