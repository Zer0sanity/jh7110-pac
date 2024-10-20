#[doc = "Register `cmd_address` reader"]
pub type R = crate::R<CmdAddressSpec>;
#[doc = "Register `cmd_address` writer"]
pub type W = crate::W<CmdAddressSpec>;
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
    pub fn address(&mut self) -> AddressW<CmdAddressSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Command Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdAddressSpec;
impl crate::RegisterSpec for CmdAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_address::R`](R) reader structure"]
impl crate::Readable for CmdAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_address::W`](W) writer structure"]
impl crate::Writable for CmdAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd_address to value 0"]
impl crate::Resettable for CmdAddressSpec {
    const RESET_VALUE: u32 = 0;
}
