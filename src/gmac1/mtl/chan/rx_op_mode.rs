#[doc = "Register `rx_op_mode` reader"]
pub type R = crate::R<RxOpModeSpec>;
#[doc = "Register `rx_op_mode` writer"]
pub type W = crate::W<RxOpModeSpec>;
#[doc = "MTL Channel RTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtc {
    #[doc = "0: MTL Channel RTC 64"]
    Rtc64 = 0,
    #[doc = "1: MTL Channel RTC 32"]
    Rtc32 = 1,
    #[doc = "2: MTL Channel RTC 96"]
    Rtc96 = 2,
    #[doc = "3: MTL Channel RTC 128"]
    Rtc128 = 3,
}
impl From<Rtc> for u8 {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtc {
    type Ux = u8;
}
#[doc = "Field `rtc` reader - MTL Channel RTC"]
pub type RtcR = crate::FieldReader<Rtc>;
impl RtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc {
        match self.bits {
            0 => Rtc::Rtc64,
            1 => Rtc::Rtc32,
            2 => Rtc::Rtc96,
            3 => Rtc::Rtc128,
            _ => unreachable!(),
        }
    }
    #[doc = "MTL Channel RTC 64"]
    #[inline(always)]
    pub fn is_rtc64(&self) -> bool {
        *self == Rtc::Rtc64
    }
    #[doc = "MTL Channel RTC 32"]
    #[inline(always)]
    pub fn is_rtc32(&self) -> bool {
        *self == Rtc::Rtc32
    }
    #[doc = "MTL Channel RTC 96"]
    #[inline(always)]
    pub fn is_rtc96(&self) -> bool {
        *self == Rtc::Rtc96
    }
    #[doc = "MTL Channel RTC 128"]
    #[inline(always)]
    pub fn is_rtc128(&self) -> bool {
        *self == Rtc::Rtc128
    }
}
#[doc = "Field `rtc` writer - MTL Channel RTC"]
pub type RtcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MTL Channel RTC 64"]
    #[inline(always)]
    pub fn rtc64(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Rtc64)
    }
    #[doc = "MTL Channel RTC 32"]
    #[inline(always)]
    pub fn rtc32(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Rtc32)
    }
    #[doc = "MTL Channel RTC 96"]
    #[inline(always)]
    pub fn rtc96(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Rtc96)
    }
    #[doc = "MTL Channel RTC 128"]
    #[inline(always)]
    pub fn rtc128(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Rtc128)
    }
}
#[doc = "Field `rsf` reader - MTL Channel RSF"]
pub type RsfR = crate::BitReader;
#[doc = "Field `rsf` writer - MTL Channel RSF"]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfa` reader - MTL Channel RX RFA"]
pub type RfaR = crate::FieldReader;
#[doc = "Field `rfa` writer - MTL Channel RX RFA"]
pub type RfaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rfd` reader - MTL Channel RX RFD"]
pub type RfdR = crate::FieldReader;
#[doc = "Field `rfd` writer - MTL Channel RX RFD"]
pub type RfdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rqs` reader - MTL Channel RQS"]
pub type RqsR = crate::FieldReader<u16>;
#[doc = "Field `rqs` writer - MTL Channel RQS"]
pub type RqsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 3:4 - MTL Channel RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - MTL Channel RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - MTL Channel RX RFA"]
    #[inline(always)]
    pub fn rfa(&self) -> RfaR {
        RfaR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - MTL Channel RX RFD"]
    #[inline(always)]
    pub fn rfd(&self) -> RfdR {
        RfdR::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:29 - MTL Channel RQS"]
    #[inline(always)]
    pub fn rqs(&self) -> RqsR {
        RqsR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:4 - MTL Channel RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RtcW<RxOpModeSpec> {
        RtcW::new(self, 3)
    }
    #[doc = "Bit 5 - MTL Channel RSF"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RsfW<RxOpModeSpec> {
        RsfW::new(self, 5)
    }
    #[doc = "Bits 8:13 - MTL Channel RX RFA"]
    #[inline(always)]
    #[must_use]
    pub fn rfa(&mut self) -> RfaW<RxOpModeSpec> {
        RfaW::new(self, 8)
    }
    #[doc = "Bits 14:19 - MTL Channel RX RFD"]
    #[inline(always)]
    #[must_use]
    pub fn rfd(&mut self) -> RfdW<RxOpModeSpec> {
        RfdW::new(self, 14)
    }
    #[doc = "Bits 20:29 - MTL Channel RQS"]
    #[inline(always)]
    #[must_use]
    pub fn rqs(&mut self) -> RqsW<RxOpModeSpec> {
        RqsW::new(self, 20)
    }
}
#[doc = "MTL Channel RX OP Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_op_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_op_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxOpModeSpec;
impl crate::RegisterSpec for RxOpModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_op_mode::R`](R) reader structure"]
impl crate::Readable for RxOpModeSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_op_mode::W`](W) writer structure"]
impl crate::Writable for RxOpModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rx_op_mode to value 0"]
impl crate::Resettable for RxOpModeSpec {
    const RESET_VALUE: u32 = 0;
}
