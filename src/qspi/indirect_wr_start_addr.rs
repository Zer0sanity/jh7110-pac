#[doc = "Register `indirect_wr_start_addr` reader"]
pub type R = crate::R<IndirectWrStartAddrSpec>;
#[doc = "Register `indirect_wr_start_addr` writer"]
pub type W = crate::W<IndirectWrStartAddrSpec>;
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
    pub fn address(&mut self) -> AddressW<IndirectWrStartAddrSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Indirect Write Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_start_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_start_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectWrStartAddrSpec;
impl crate::RegisterSpec for IndirectWrStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_wr_start_addr::R`](R) reader structure"]
impl crate::Readable for IndirectWrStartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`indirect_wr_start_addr::W`](W) writer structure"]
impl crate::Writable for IndirectWrStartAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets indirect_wr_start_addr to value 0"]
impl crate::Resettable for IndirectWrStartAddrSpec {
    const RESET_VALUE: u32 = 0;
}
