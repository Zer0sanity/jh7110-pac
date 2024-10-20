#[doc = "Register `capabilities` reader"]
pub type R = crate::R<CapabilitiesSpec>;
#[doc = "Register `capabilities` writer"]
pub type W = crate::W<CapabilitiesSpec>;
#[doc = "Field `capabilitites` reader - USB3 OTG capabilitites."]
pub type CapabilititesR = crate::FieldReader<u32>;
#[doc = "Field `capabilitites` writer - USB3 OTG capabilitites."]
pub type CapabilititesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 OTG capabilitites."]
    #[inline(always)]
    pub fn capabilitites(&self) -> CapabilititesR {
        CapabilititesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB3 OTG capabilitites."]
    #[inline(always)]
    #[must_use]
    pub fn capabilitites(&mut self) -> CapabilititesW<CapabilitiesSpec> {
        CapabilititesW::new(self, 0)
    }
}
#[doc = "USB3 OTG capabilities.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capabilities::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capabilities::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapabilitiesSpec;
impl crate::RegisterSpec for CapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capabilities::R`](R) reader structure"]
impl crate::Readable for CapabilitiesSpec {}
#[doc = "`write(|w| ..)` method takes [`capabilities::W`](W) writer structure"]
impl crate::Writable for CapabilitiesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets capabilities to value 0"]
impl crate::Resettable for CapabilitiesSpec {
    const RESET_VALUE: u32 = 0;
}
