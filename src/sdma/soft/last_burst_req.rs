#[doc = "Register `last_burst_req` reader"]
pub type R = crate::R<LastBurstReqSpec>;
#[doc = "Register `last_burst_req` writer"]
pub type W = crate::W<LastBurstReqSpec>;
#[doc = "Field `last_burst_req` reader - Software last burst request."]
pub type LastBurstReqR = crate::FieldReader<u16>;
#[doc = "Field `last_burst_req` writer - Software last burst request."]
pub type LastBurstReqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Software last burst request."]
    #[inline(always)]
    pub fn last_burst_req(&self) -> LastBurstReqR {
        LastBurstReqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Software last burst request."]
    #[inline(always)]
    #[must_use]
    pub fn last_burst_req(&mut self) -> LastBurstReqW<LastBurstReqSpec> {
        LastBurstReqW::new(self, 0)
    }
}
#[doc = "Software Last Burst Request Register - enables software to generate DMA last burst requests. You can generate a DMA request for each source by writing a 1 to the corresponding register bit. A register bit is cleared when the transaction has completed. Writing 0 to this register has no effect. Reading the register indicates the sources that are requesting last burst DMA transfers. You can generate a request from either a peripheral or the software request register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`last_burst_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`last_burst_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LastBurstReqSpec;
impl crate::RegisterSpec for LastBurstReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`last_burst_req::R`](R) reader structure"]
impl crate::Readable for LastBurstReqSpec {}
#[doc = "`write(|w| ..)` method takes [`last_burst_req::W`](W) writer structure"]
impl crate::Writable for LastBurstReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets last_burst_req to value 0"]
impl crate::Resettable for LastBurstReqSpec {
    const RESET_VALUE: u32 = 0;
}
