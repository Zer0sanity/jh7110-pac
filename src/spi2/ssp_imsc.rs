#[doc = "Register `ssp_imsc` reader"]
pub type R = crate::R<SspImscSpec>;
#[doc = "Register `ssp_imsc` writer"]
pub type W = crate::W<SspImscSpec>;
#[doc = "Field `rorim` reader - Receive overrun interrupt mask - 0: Receive FIFO written to while full condition interrupt is masked, 1: Receive FIFO written to while full condition interrupt is not masked"]
pub type RorimR = crate::BitReader;
#[doc = "Field `rorim` writer - Receive overrun interrupt mask - 0: Receive FIFO written to while full condition interrupt is masked, 1: Receive FIFO written to while full condition interrupt is not masked"]
pub type RorimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtim` reader - Receive timeout interrupt mask - 0: Receive FIFO not empty and no read prior to timeout period interrupt is masked, 1: Receive FIFO not empty and no read prior to timeout period interrupt is not masked"]
pub type RtimR = crate::BitReader;
#[doc = "Field `rtim` writer - Receive timeout interrupt mask - 0: Receive FIFO not empty and no read prior to timeout period interrupt is masked, 1: Receive FIFO not empty and no read prior to timeout period interrupt is not masked"]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxim` reader - Receive FIFO interrupt mask - 0: Receive FIFO half full or less condition interrupt is masked, 1: Receive FIFO half full or less condition interrupt is not masked"]
pub type RximR = crate::BitReader;
#[doc = "Field `rxim` writer - Receive FIFO interrupt mask - 0: Receive FIFO half full or less condition interrupt is masked, 1: Receive FIFO half full or less condition interrupt is not masked"]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txim` reader - Transmit FIFO interrupt mask - 0: Transmit FIFO half empty or less condition interrupt is masked, 1: Transmit FIFO half empty or less condition interrupt is not masked"]
pub type TximR = crate::BitReader;
#[doc = "Field `txim` writer - Transmit FIFO interrupt mask - 0: Transmit FIFO half empty or less condition interrupt is masked, 1: Transmit FIFO half empty or less condition interrupt is not masked"]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive overrun interrupt mask - 0: Receive FIFO written to while full condition interrupt is masked, 1: Receive FIFO written to while full condition interrupt is not masked"]
    #[inline(always)]
    pub fn rorim(&self) -> RorimR {
        RorimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask - 0: Receive FIFO not empty and no read prior to timeout period interrupt is masked, 1: Receive FIFO not empty and no read prior to timeout period interrupt is not masked"]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask - 0: Receive FIFO half full or less condition interrupt is masked, 1: Receive FIFO half full or less condition interrupt is not masked"]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO interrupt mask - 0: Transmit FIFO half empty or less condition interrupt is masked, 1: Transmit FIFO half empty or less condition interrupt is not masked"]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive overrun interrupt mask - 0: Receive FIFO written to while full condition interrupt is masked, 1: Receive FIFO written to while full condition interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rorim(&mut self) -> RorimW<SspImscSpec> {
        RorimW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive timeout interrupt mask - 0: Receive FIFO not empty and no read prior to timeout period interrupt is masked, 1: Receive FIFO not empty and no read prior to timeout period interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<SspImscSpec> {
        RtimW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO interrupt mask - 0: Receive FIFO half full or less condition interrupt is masked, 1: Receive FIFO half full or less condition interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<SspImscSpec> {
        RximW::new(self, 2)
    }
    #[doc = "Bit 2 - Transmit FIFO interrupt mask - 0: Transmit FIFO half empty or less condition interrupt is masked, 1: Transmit FIFO half empty or less condition interrupt is not masked"]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<SspImscSpec> {
        TximW::new(self, 2)
    }
}
#[doc = "The SSPIMSC register is the interrupt mask set or clear register. It is a RW register. On a read this register gives the current value of the mask on the relevant interrupt. A write of 1 to the particular bit sets the mask, enabling the interrupt to be read. A write of 0 clears the corresponding mask. All the bits are cleared to 0 when reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspImscSpec;
impl crate::RegisterSpec for SspImscSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_imsc::R`](R) reader structure"]
impl crate::Readable for SspImscSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_imsc::W`](W) writer structure"]
impl crate::Writable for SspImscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ssp_imsc to value 0"]
impl crate::Resettable for SspImscSpec {
    const RESET_VALUE: u16 = 0;
}
