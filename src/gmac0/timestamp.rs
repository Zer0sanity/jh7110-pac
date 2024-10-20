#[doc = "Register `timestamp` reader"]
pub type R = crate::R<TimestampSpec>;
#[doc = "Register `timestamp` writer"]
pub type W = crate::W<TimestampSpec>;
#[doc = "Field `auxtstrig` reader - AUX Timestamp Trigger"]
pub type AuxtstrigR = crate::BitReader;
#[doc = "Field `auxtstrig` writer - AUX Timestamp Trigger"]
pub type AuxtstrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `atsns` reader - AUX Timestamp Nanosecond"]
pub type AtsnsR = crate::FieldReader;
#[doc = "Field `atsns` writer - AUX Timestamp Nanosecond"]
pub type AtsnsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 2 - AUX Timestamp Trigger"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AuxtstrigR {
        AuxtstrigR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 25:29 - AUX Timestamp Nanosecond"]
    #[inline(always)]
    pub fn atsns(&self) -> AtsnsR {
        AtsnsR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - AUX Timestamp Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn auxtstrig(&mut self) -> AuxtstrigW<TimestampSpec> {
        AuxtstrigW::new(self, 2)
    }
    #[doc = "Bits 25:29 - AUX Timestamp Nanosecond"]
    #[inline(always)]
    #[must_use]
    pub fn atsns(&mut self) -> AtsnsW<TimestampSpec> {
        AtsnsW::new(self, 25)
    }
}
#[doc = "MAC Timestamp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimestampSpec;
impl crate::RegisterSpec for TimestampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp::R`](R) reader structure"]
impl crate::Readable for TimestampSpec {}
#[doc = "`write(|w| ..)` method takes [`timestamp::W`](W) writer structure"]
impl crate::Writable for TimestampSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timestamp to value 0"]
impl crate::Resettable for TimestampSpec {
    const RESET_VALUE: u32 = 0;
}
