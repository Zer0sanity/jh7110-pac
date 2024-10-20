#[doc = "Register `sar` reader"]
pub type R = crate::R<SarSpec>;
#[doc = "Register `sar` writer"]
pub type W = crate::W<SarSpec>;
#[doc = "Field `address_7bit` reader - Slave address, 7-bit mode"]
pub type Address7bitR = crate::FieldReader;
#[doc = "Field `address_7bit` writer - Slave address, 7-bit mode"]
pub type Address7bitW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `address_10bit` reader - Slave address, 10-bit mode"]
pub type Address10bitR = crate::FieldReader<u16>;
#[doc = "Field `address_10bit` writer - Slave address, 10-bit mode"]
pub type Address10bitW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:6 - Slave address, 7-bit mode"]
    #[inline(always)]
    pub fn address_7bit(&self) -> Address7bitR {
        Address7bitR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 0:9 - Slave address, 10-bit mode"]
    #[inline(always)]
    pub fn address_10bit(&self) -> Address10bitR {
        Address10bitR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave address, 7-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn address_7bit(&mut self) -> Address7bitW<SarSpec> {
        Address7bitW::new(self, 0)
    }
    #[doc = "Bits 0:9 - Slave address, 10-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn address_10bit(&mut self) -> Address10bitW<SarSpec> {
        Address10bitW::new(self, 0)
    }
}
#[doc = "DesignWare I2C SAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarSpec;
impl crate::RegisterSpec for SarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SarSpec {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sar to value 0"]
impl crate::Resettable for SarSpec {
    const RESET_VALUE: u32 = 0;
}
