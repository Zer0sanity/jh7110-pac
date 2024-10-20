#[doc = "Register `last_single_req` reader"]
pub type R = crate::R<LastSingleReqSpec>;
#[doc = "Register `last_single_req` writer"]
pub type W = crate::W<LastSingleReqSpec>;
#[doc = "Field `last_single_req` reader - Software last single request."]
pub type LastSingleReqR = crate::FieldReader<u16>;
#[doc = "Field `last_single_req` writer - Software last single request."]
pub type LastSingleReqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Software last single request."]
    #[inline(always)]
    pub fn last_single_req(&self) -> LastSingleReqR {
        LastSingleReqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Software last single request."]
    #[inline(always)]
    #[must_use]
    pub fn last_single_req(&mut self) -> LastSingleReqW<LastSingleReqSpec> {
        LastSingleReqW::new(self, 0)
    }
}
#[doc = "Software Last Single Request Register - enables software to generate DMA last single requests. You can generate a DMA request for each source by writing a 1 to the corresponding register bit. A register bit is cleared when the transaction has completed. Writing 0 to this register has no effect. Reading the register indicates the sources that are requesting last single DMA transfers. You can generate a request from either a peripheral or the software request register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`last_single_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`last_single_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LastSingleReqSpec;
impl crate::RegisterSpec for LastSingleReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`last_single_req::R`](R) reader structure"]
impl crate::Readable for LastSingleReqSpec {}
#[doc = "`write(|w| ..)` method takes [`last_single_req::W`](W) writer structure"]
impl crate::Writable for LastSingleReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets last_single_req to value 0"]
impl crate::Resettable for LastSingleReqSpec {
    const RESET_VALUE: u32 = 0;
}
