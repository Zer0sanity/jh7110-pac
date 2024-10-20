#[doc = "Register `ssp_dr` reader"]
pub type R = crate::R<SspDrSpec>;
#[doc = "Register `ssp_dr` writer"]
pub type W = crate::W<SspDrSpec>;
#[doc = "Field `data` reader - Transmit/Receive FIFO - Read: Receive FIFO, Write: Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `data` writer - Transmit/Receive FIFO - Read: Receive FIFO, Write: Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit/Receive FIFO - Read: Receive FIFO, Write: Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit/Receive FIFO - Read: Receive FIFO, Write: Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<SspDrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "SSPDR is the data register and is 16-bits wide. When SSPDR is read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the PrimeCell SSP receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When SSPDR is written to, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the SSPTXD pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspDrSpec;
impl crate::RegisterSpec for SspDrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_dr::R`](R) reader structure"]
impl crate::Readable for SspDrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_dr::W`](W) writer structure"]
impl crate::Writable for SspDrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ssp_dr to value 0"]
impl crate::Resettable for SspDrSpec {
    const RESET_VALUE: u16 = 0;
}
