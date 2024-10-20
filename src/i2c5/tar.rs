#[doc = "Register `tar` reader"]
pub type R = crate::R<TarSpec>;
#[doc = "Register `tar` writer"]
pub type W = crate::W<TarSpec>;
#[doc = "Field `address_7bit` reader - Target address, 7-bit mode"]
pub type Address7bitR = crate::FieldReader;
#[doc = "Field `address_7bit` writer - Target address, 7-bit mode"]
pub type Address7bitW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `address_10bit` reader - Target address, 10-bit mode"]
pub type Address10bitR = crate::FieldReader<u16>;
#[doc = "Field `address_10bit` writer - Target address, 10-bit mode"]
pub type Address10bitW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `mode` reader - Target addressing mode - 0: 7-bit, 1: 10-bit"]
pub type ModeR = crate::BitReader;
#[doc = "Field `mode` writer - Target addressing mode - 0: 7-bit, 1: 10-bit"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Target address, 7-bit mode"]
    #[inline(always)]
    pub fn address_7bit(&self) -> Address7bitR {
        Address7bitR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 0:9 - Target address, 10-bit mode"]
    #[inline(always)]
    pub fn address_10bit(&self) -> Address10bitR {
        Address10bitR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - Target addressing mode - 0: 7-bit, 1: 10-bit"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Target address, 7-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn address_7bit(&mut self) -> Address7bitW<TarSpec> {
        Address7bitW::new(self, 0)
    }
    #[doc = "Bits 0:9 - Target address, 10-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn address_10bit(&mut self) -> Address10bitW<TarSpec> {
        Address10bitW::new(self, 0)
    }
    #[doc = "Bit 12 - Target addressing mode - 0: 7-bit, 1: 10-bit"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<TarSpec> {
        ModeW::new(self, 12)
    }
}
#[doc = "DesignWare I2C TAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TarSpec;
impl crate::RegisterSpec for TarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar::R`](R) reader structure"]
impl crate::Readable for TarSpec {}
#[doc = "`write(|w| ..)` method takes [`tar::W`](W) writer structure"]
impl crate::Writable for TarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tar to value 0"]
impl crate::Resettable for TarSpec {
    const RESET_VALUE: u32 = 0;
}
