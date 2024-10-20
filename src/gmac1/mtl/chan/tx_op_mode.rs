#[doc = "Register `tx_op_mode` reader"]
pub type R = crate::R<TxOpModeSpec>;
#[doc = "Register `tx_op_mode` writer"]
pub type W = crate::W<TxOpModeSpec>;
#[doc = "Field `tsf` reader - MTL Channel TSF"]
pub type TsfR = crate::BitReader;
#[doc = "Field `tsf` writer - MTL Channel TSF"]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txqen_av` reader - MTL Channel TXQEN AV"]
pub type TxqenAvR = crate::BitReader;
#[doc = "Field `txqen_av` writer - MTL Channel TXQEN AV"]
pub type TxqenAvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txqen` reader - MTL Channel TXQEN"]
pub type TxqenR = crate::BitReader;
#[doc = "Field `txqen` writer - MTL Channel TXQEN"]
pub type TxqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MTL Channel TTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ttc {
    #[doc = "0: MTL Channel TTC 32"]
    Ttc32 = 0,
    #[doc = "1: MTL Channel TTC 64"]
    Ttc64 = 1,
    #[doc = "2: MTL Channel TTC 96"]
    Ttc96 = 2,
    #[doc = "3: MTL Channel TTC 128"]
    Ttc128 = 3,
    #[doc = "4: MTL Channel TTC 192"]
    Ttc192 = 4,
    #[doc = "5: MTL Channel TTC 256"]
    Ttc256 = 5,
    #[doc = "6: MTL Channel TTC 384"]
    Ttc384 = 6,
    #[doc = "7: MTL Channel TTC 512"]
    Ttc512 = 7,
}
impl From<Ttc> for u8 {
    #[inline(always)]
    fn from(variant: Ttc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ttc {
    type Ux = u8;
}
#[doc = "Field `ttc` reader - MTL Channel TTC"]
pub type TtcR = crate::FieldReader<Ttc>;
impl TtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttc {
        match self.bits {
            0 => Ttc::Ttc32,
            1 => Ttc::Ttc64,
            2 => Ttc::Ttc96,
            3 => Ttc::Ttc128,
            4 => Ttc::Ttc192,
            5 => Ttc::Ttc256,
            6 => Ttc::Ttc384,
            7 => Ttc::Ttc512,
            _ => unreachable!(),
        }
    }
    #[doc = "MTL Channel TTC 32"]
    #[inline(always)]
    pub fn is_ttc32(&self) -> bool {
        *self == Ttc::Ttc32
    }
    #[doc = "MTL Channel TTC 64"]
    #[inline(always)]
    pub fn is_ttc64(&self) -> bool {
        *self == Ttc::Ttc64
    }
    #[doc = "MTL Channel TTC 96"]
    #[inline(always)]
    pub fn is_ttc96(&self) -> bool {
        *self == Ttc::Ttc96
    }
    #[doc = "MTL Channel TTC 128"]
    #[inline(always)]
    pub fn is_ttc128(&self) -> bool {
        *self == Ttc::Ttc128
    }
    #[doc = "MTL Channel TTC 192"]
    #[inline(always)]
    pub fn is_ttc192(&self) -> bool {
        *self == Ttc::Ttc192
    }
    #[doc = "MTL Channel TTC 256"]
    #[inline(always)]
    pub fn is_ttc256(&self) -> bool {
        *self == Ttc::Ttc256
    }
    #[doc = "MTL Channel TTC 384"]
    #[inline(always)]
    pub fn is_ttc384(&self) -> bool {
        *self == Ttc::Ttc384
    }
    #[doc = "MTL Channel TTC 512"]
    #[inline(always)]
    pub fn is_ttc512(&self) -> bool {
        *self == Ttc::Ttc512
    }
}
#[doc = "Field `ttc` writer - MTL Channel TTC"]
pub type TtcW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Ttc>;
impl<'a, REG> TtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MTL Channel TTC 32"]
    #[inline(always)]
    pub fn ttc32(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Ttc32)
    }
    #[doc = "MTL Channel TTC 64"]
    #[inline(always)]
    pub fn ttc64(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Ttc64)
    }
    #[doc = "MTL Channel TTC 96"]
    #[inline(always)]
    pub fn ttc96(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Ttc96)
    }
    #[doc = "MTL Channel TTC 128"]
    #[inline(always)]
    pub fn ttc128(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Ttc128)
    }
    #[doc = "MTL Channel TTC 192"]
    #[inline(always)]
    pub fn ttc192(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Ttc192)
    }
    #[doc = "MTL Channel TTC 256"]
    #[inline(always)]
    pub fn ttc256(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Ttc256)
    }
    #[doc = "MTL Channel TTC 384"]
    #[inline(always)]
    pub fn ttc384(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Ttc384)
    }
    #[doc = "MTL Channel TTC 512"]
    #[inline(always)]
    pub fn ttc512(self) -> &'a mut crate::W<REG> {
        self.variant(Ttc::Ttc512)
    }
}
#[doc = "Field `tqs` reader - MTL Channel TQS"]
pub type TqsR = crate::FieldReader<u16>;
#[doc = "Field `tqs` writer - MTL Channel TQS"]
pub type TqsW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 1 - MTL Channel TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MTL Channel TXQEN AV"]
    #[inline(always)]
    pub fn txqen_av(&self) -> TxqenAvR {
        TxqenAvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MTL Channel TXQEN"]
    #[inline(always)]
    pub fn txqen(&self) -> TxqenR {
        TxqenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - MTL Channel TTC"]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:24 - MTL Channel TQS"]
    #[inline(always)]
    pub fn tqs(&self) -> TqsR {
        TqsR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - MTL Channel TSF"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TsfW<TxOpModeSpec> {
        TsfW::new(self, 1)
    }
    #[doc = "Bit 2 - MTL Channel TXQEN AV"]
    #[inline(always)]
    #[must_use]
    pub fn txqen_av(&mut self) -> TxqenAvW<TxOpModeSpec> {
        TxqenAvW::new(self, 2)
    }
    #[doc = "Bit 3 - MTL Channel TXQEN"]
    #[inline(always)]
    #[must_use]
    pub fn txqen(&mut self) -> TxqenW<TxOpModeSpec> {
        TxqenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - MTL Channel TTC"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TtcW<TxOpModeSpec> {
        TtcW::new(self, 4)
    }
    #[doc = "Bits 16:24 - MTL Channel TQS"]
    #[inline(always)]
    #[must_use]
    pub fn tqs(&mut self) -> TqsW<TxOpModeSpec> {
        TqsW::new(self, 16)
    }
}
#[doc = "MTL Channel TX OP Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_op_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_op_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxOpModeSpec;
impl crate::RegisterSpec for TxOpModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_op_mode::R`](R) reader structure"]
impl crate::Readable for TxOpModeSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_op_mode::W`](W) writer structure"]
impl crate::Writable for TxOpModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_op_mode to value 0"]
impl crate::Resettable for TxOpModeSpec {
    const RESET_VALUE: u32 = 0;
}
