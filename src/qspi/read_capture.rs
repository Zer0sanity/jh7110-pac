#[doc = "Register `read_capture` reader"]
pub type R = crate::R<ReadCaptureSpec>;
#[doc = "Register `read_capture` writer"]
pub type W = crate::W<ReadCaptureSpec>;
#[doc = "Field `bypass` reader - Bypass the Read Capture"]
pub type BypassR = crate::BitReader;
#[doc = "Field `bypass` writer - Bypass the Read Capture"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `delay` reader - Read Capture Delay Value"]
pub type DelayR = crate::FieldReader;
#[doc = "Field `delay` writer - Read Capture Delay Value"]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Bypass the Read Capture"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Read Capture Delay Value"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass the Read Capture"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<ReadCaptureSpec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Read Capture Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DelayW<ReadCaptureSpec> {
        DelayW::new(self, 1)
    }
}
#[doc = "Cadence QSPI Read Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read_capture::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`read_capture::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadCaptureSpec;
impl crate::RegisterSpec for ReadCaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_capture::R`](R) reader structure"]
impl crate::Readable for ReadCaptureSpec {}
#[doc = "`write(|w| ..)` method takes [`read_capture::W`](W) writer structure"]
impl crate::Writable for ReadCaptureSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets read_capture to value 0"]
impl crate::Resettable for ReadCaptureSpec {
    const RESET_VALUE: u32 = 0;
}
