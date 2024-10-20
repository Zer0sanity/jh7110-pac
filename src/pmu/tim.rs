#[doc = "Register `tim` reader"]
pub type R = crate::R<TimSpec>;
#[doc = "Register `tim` writer"]
pub type W = crate::W<TimSpec>;
#[doc = "Field `seq_done_mask` reader - Mask the sequence complete event. 0: mask, 1: unmask"]
pub type SeqDoneMaskR = crate::BitReader;
#[doc = "Field `seq_done_mask` writer - Mask the sequence complete event. 0: mask, 1: unmask"]
pub type SeqDoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hw_req_mask` reader - Mask the hardware encouragement request. 0: mask, 1: unmask"]
pub type HwReqMaskR = crate::BitReader;
#[doc = "Field `hw_req_mask` writer - Mask the hardware encouragement request. 0: mask, 1: unmask"]
pub type HwReqMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sw_fail_mask` reader - Mask the software encouragement failure event. 0: mask, 1: unmask"]
pub type SwFailMaskR = crate::FieldReader;
#[doc = "Field `sw_fail_mask` writer - Mask the software encouragement failure event. 0: mask, 1: unmask"]
pub type SwFailMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `hw_fail_mask` reader - Mask the hardware encouragement failure event. 0: mask, 1: unmask"]
pub type HwFailMaskR = crate::FieldReader;
#[doc = "Field `hw_fail_mask` writer - Mask the hardware encouragement failure event. 0: mask, 1: unmask"]
pub type HwFailMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pch_fail_mask` reader - Mask the P-channel encouragement failure event. 0: mask, 1: unmask"]
pub type PchFailMaskR = crate::FieldReader;
#[doc = "Field `pch_fail_mask` writer - Mask the P-channel encouragement failure event. 0: mask, 1: unmask"]
pub type PchFailMaskW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Mask the sequence complete event. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn seq_done_mask(&self) -> SeqDoneMaskR {
        SeqDoneMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask the hardware encouragement request. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn hw_req_mask(&self) -> HwReqMaskR {
        HwReqMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Mask the software encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn sw_fail_mask(&self) -> SwFailMaskR {
        SwFailMaskR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Mask the hardware encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn hw_fail_mask(&self) -> HwFailMaskR {
        HwFailMaskR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - Mask the P-channel encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    pub fn pch_fail_mask(&self) -> PchFailMaskR {
        PchFailMaskR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mask the sequence complete event. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn seq_done_mask(&mut self) -> SeqDoneMaskW<TimSpec> {
        SeqDoneMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask the hardware encouragement request. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn hw_req_mask(&mut self) -> HwReqMaskW<TimSpec> {
        HwReqMaskW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Mask the software encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fail_mask(&mut self) -> SwFailMaskW<TimSpec> {
        SwFailMaskW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mask the hardware encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn hw_fail_mask(&mut self) -> HwFailMaskW<TimSpec> {
        HwFailMaskW::new(self, 4)
    }
    #[doc = "Bits 6:8 - Mask the P-channel encouragement failure event. 0: mask, 1: unmask"]
    #[inline(always)]
    #[must_use]
    pub fn pch_fail_mask(&mut self) -> PchFailMaskW<TimSpec> {
        PchFailMaskW::new(self, 6)
    }
}
#[doc = "Timer Interrupt Mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimSpec;
impl crate::RegisterSpec for TimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim::R`](R) reader structure"]
impl crate::Readable for TimSpec {}
#[doc = "`write(|w| ..)` method takes [`tim::W`](W) writer structure"]
impl crate::Writable for TimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tim to value 0"]
impl crate::Resettable for TimSpec {
    const RESET_VALUE: u32 = 0;
}
