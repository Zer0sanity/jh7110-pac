#[doc = "Register `indirect_trigger` reader"]
pub type R = crate::R<IndirectTriggerSpec>;
#[doc = "Register `indirect_trigger` writer"]
pub type W = crate::W<IndirectTriggerSpec>;
#[doc = "Field `address` reader - "]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `address` writer - "]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<IndirectTriggerSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Indirect Trigger Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_trigger::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_trigger::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectTriggerSpec;
impl crate::RegisterSpec for IndirectTriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_trigger::R`](R) reader structure"]
impl crate::Readable for IndirectTriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`indirect_trigger::W`](W) writer structure"]
impl crate::Writable for IndirectTriggerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets indirect_trigger to value 0"]
impl crate::Resettable for IndirectTriggerSpec {
    const RESET_VALUE: u32 = 0;
}
