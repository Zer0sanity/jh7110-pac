#[doc = "Register `sda_hold` reader"]
pub type R = crate::R<SdaHoldSpec>;
#[doc = "Register `sda_hold` writer"]
pub type W = crate::W<SdaHoldSpec>;
#[doc = "Field `sda_hold` reader - "]
pub type SdaHoldR = crate::FieldReader<u32>;
#[doc = "Field `sda_hold` writer - "]
pub type SdaHoldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sda_hold(&self) -> SdaHoldR {
        SdaHoldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sda_hold(&mut self) -> SdaHoldW<SdaHoldSpec> {
        SdaHoldW::new(self, 0)
    }
}
#[doc = "DesignWare I2C SDA Hold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdaHoldSpec;
impl crate::RegisterSpec for SdaHoldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_hold::R`](R) reader structure"]
impl crate::Readable for SdaHoldSpec {}
#[doc = "`write(|w| ..)` method takes [`sda_hold::W`](W) writer structure"]
impl crate::Writable for SdaHoldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sda_hold to value 0"]
impl crate::Resettable for SdaHoldSpec {
    const RESET_VALUE: u32 = 0;
}
