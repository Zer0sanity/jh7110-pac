#[doc = "Register `ssp_cr1` reader"]
pub type R = crate::R<SspCr1Spec>;
#[doc = "Register `ssp_cr1` writer"]
pub type W = crate::W<SspCr1Spec>;
#[doc = "Field `lbm` reader - Loop back mode - 0: Normal serial port operation enabled, 1: Output of transmit serial shifter is connected to input of receive serial shifter internally"]
pub type LbmR = crate::BitReader;
#[doc = "Field `lbm` writer - Loop back mode - 0: Normal serial port operation enabled, 1: Output of transmit serial shifter is connected to input of receive serial shifter internally"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sse` reader - Synchronous serial port enable - 0: SSP operation disabled, 1: SSP operation enabled"]
pub type SseR = crate::BitReader;
#[doc = "Field `sse` writer - Synchronous serial port enable - 0: SSP operation disabled, 1: SSP operation enabled"]
pub type SseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ms` reader - Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0 - 0: Device configured as master (default), 1: Device configured as slave"]
pub type MsR = crate::BitReader;
#[doc = "Field `ms` writer - Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0 - 0: Device configured as master (default), 1: Device configured as slave"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sod` reader - Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line - 0: SSP can drive the SSPTXD output in slave mode, 1: SSP must not drive the SSPTXD output in slave mode"]
pub type SodR = crate::BitReader;
#[doc = "Field `sod` writer - Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line - 0: SSP can drive the SSPTXD output in slave mode, 1: SSP must not drive the SSPTXD output in slave mode"]
pub type SodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Loop back mode - 0: Normal serial port operation enabled, 1: Output of transmit serial shifter is connected to input of receive serial shifter internally"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronous serial port enable - 0: SSP operation disabled, 1: SSP operation enabled"]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0 - 0: Device configured as master (default), 1: Device configured as slave"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line - 0: SSP can drive the SSPTXD output in slave mode, 1: SSP must not drive the SSPTXD output in slave mode"]
    #[inline(always)]
    pub fn sod(&self) -> SodR {
        SodR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Loop back mode - 0: Normal serial port operation enabled, 1: Output of transmit serial shifter is connected to input of receive serial shifter internally"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LbmW<SspCr1Spec> {
        LbmW::new(self, 0)
    }
    #[doc = "Bit 1 - Synchronous serial port enable - 0: SSP operation disabled, 1: SSP operation enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SseW<SspCr1Spec> {
        SseW::new(self, 1)
    }
    #[doc = "Bit 2 - Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0 - 0: Device configured as master (default), 1: Device configured as slave"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<SspCr1Spec> {
        MsW::new(self, 2)
    }
    #[doc = "Bit 3 - Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line - 0: SSP can drive the SSPTXD output in slave mode, 1: SSP must not drive the SSPTXD output in slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn sod(&mut self) -> SodW<SspCr1Spec> {
        SodW::new(self, 3)
    }
}
#[doc = "SSPCR1 is the control register 1 and contains four different bit fields, that control various functions within the PrimeCell SSP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspCr1Spec;
impl crate::RegisterSpec for SspCr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_cr1::R`](R) reader structure"]
impl crate::Readable for SspCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ssp_cr1::W`](W) writer structure"]
impl crate::Writable for SspCr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ssp_cr1 to value 0"]
impl crate::Resettable for SspCr1Spec {
    const RESET_VALUE: u16 = 0;
}
