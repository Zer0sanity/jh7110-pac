#[doc = "Register `polling_status` reader"]
pub type R = crate::R<PollingStatusSpec>;
#[doc = "Register `polling_status` writer"]
pub type W = crate::W<PollingStatusSpec>;
#[doc = "Field `status` reader - "]
pub type StatusR = crate::FieldReader<u16>;
#[doc = "Field `status` writer - "]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<PollingStatusSpec> {
        StatusW::new(self, 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn dummy(&mut self) -> DummyW<PollingStatusSpec> {
        DummyW::new(self, 16)
    }
}
#[doc = "Cadence QSPI Polling Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polling_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polling_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PollingStatusSpec;
impl crate::RegisterSpec for PollingStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polling_status::R`](R) reader structure"]
impl crate::Readable for PollingStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`polling_status::W`](W) writer structure"]
impl crate::Writable for PollingStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets polling_status to value 0"]
impl crate::Resettable for PollingStatusSpec {
    const RESET_VALUE: u32 = 0;
}
