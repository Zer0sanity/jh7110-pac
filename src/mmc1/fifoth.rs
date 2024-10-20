#[doc = "Register `fifoth` reader"]
pub type R = crate::R<FifothSpec>;
#[doc = "Register `fifoth` writer"]
pub type W = crate::W<FifothSpec>;
#[doc = "Field `tx_wmark` reader - MMC FIFOTH TX watermark"]
pub type TxWmarkR = crate::FieldReader<u16>;
#[doc = "Field `tx_wmark` writer - MMC FIFOTH TX watermark"]
pub type TxWmarkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `rx_wmark` reader - MMC FIFOTH RX watermark"]
pub type RxWmarkR = crate::FieldReader<u16>;
#[doc = "Field `rx_wmark` writer - MMC FIFOTH RX watermark"]
pub type RxWmarkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `msize` reader - MMC FIFOTH msize"]
pub type MsizeR = crate::FieldReader;
#[doc = "Field `msize` writer - MMC FIFOTH msize"]
pub type MsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - MMC FIFOTH TX watermark"]
    #[inline(always)]
    pub fn tx_wmark(&self) -> TxWmarkR {
        TxWmarkR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - MMC FIFOTH RX watermark"]
    #[inline(always)]
    pub fn rx_wmark(&self) -> RxWmarkR {
        RxWmarkR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - MMC FIFOTH msize"]
    #[inline(always)]
    pub fn msize(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - MMC FIFOTH TX watermark"]
    #[inline(always)]
    #[must_use]
    pub fn tx_wmark(&mut self) -> TxWmarkW<FifothSpec> {
        TxWmarkW::new(self, 0)
    }
    #[doc = "Bits 16:27 - MMC FIFOTH RX watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rx_wmark(&mut self) -> RxWmarkW<FifothSpec> {
        RxWmarkW::new(self, 16)
    }
    #[doc = "Bits 28:30 - MMC FIFOTH msize"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MsizeW<FifothSpec> {
        MsizeW::new(self, 28)
    }
}
#[doc = "MMC FIFOTH\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifoth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifoth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifothSpec;
impl crate::RegisterSpec for FifothSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoth::R`](R) reader structure"]
impl crate::Readable for FifothSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoth::W`](W) writer structure"]
impl crate::Writable for FifothSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fifoth to value 0"]
impl crate::Resettable for FifothSpec {
    const RESET_VALUE: u32 = 0;
}
