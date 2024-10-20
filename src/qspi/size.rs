#[doc = "Register `size` reader"]
pub type R = crate::R<SizeSpec>;
#[doc = "Register `size` writer"]
pub type W = crate::W<SizeSpec>;
#[doc = "Field `address` reader - Address Size in Bytes"]
pub type AddressR = crate::FieldReader;
#[doc = "Field `address` writer - Address Size in Bytes"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `page` reader - Page Size in Bytes"]
pub type PageR = crate::FieldReader<u16>;
#[doc = "Field `page` writer - Page Size in Bytes"]
pub type PageW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `block` reader - Block Size in Bytes"]
pub type BlockR = crate::FieldReader;
#[doc = "Field `block` writer - Block Size in Bytes"]
pub type BlockW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - Address Size in Bytes"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Page Size in Bytes"]
    #[inline(always)]
    pub fn page(&self) -> PageR {
        PageR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Block Size in Bytes"]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address Size in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<SizeSpec> {
        AddressW::new(self, 0)
    }
    #[doc = "Bits 4:15 - Page Size in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn page(&mut self) -> PageW<SizeSpec> {
        PageW::new(self, 4)
    }
    #[doc = "Bits 16:21 - Block Size in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BlockW<SizeSpec> {
        BlockW::new(self, 16)
    }
}
#[doc = "Cadence QSPI Size Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SizeSpec;
impl crate::RegisterSpec for SizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`size::R`](R) reader structure"]
impl crate::Readable for SizeSpec {}
#[doc = "`write(|w| ..)` method takes [`size::W`](W) writer structure"]
impl crate::Writable for SizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets size to value 0"]
impl crate::Resettable for SizeSpec {
    const RESET_VALUE: u32 = 0;
}
