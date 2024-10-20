#[doc = "Register `tfr` reader"]
pub type R = crate::R<TfrSpec>;
#[doc = "Field `tfr` reader - Transmit FIFO Read. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, reading this register gives the data at the top of the transmit FIFO. Each consecutive read pops the transmit FIFO and gives the next data value that is currently at the top of the FIFO. When FIFOs are not implemented or not enabled, reading this register gives the data in the THR."]
pub type TfrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Read. These bits are only valid when FIFO access mode is enabled (FAR\\[0\\]
is set to one). When FIFOs are implemented and enabled, reading this register gives the data at the top of the transmit FIFO. Each consecutive read pops the transmit FIFO and gives the next data value that is currently at the top of the FIFO. When FIFOs are not implemented or not enabled, reading this register gives the data in the THR."]
    #[inline(always)]
    pub fn tfr(&self) -> TfrR {
        TfrR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Transmit FIFO Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TfrSpec;
impl crate::RegisterSpec for TfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfr::R`](R) reader structure"]
impl crate::Readable for TfrSpec {}
#[doc = "`reset()` method sets tfr to value 0"]
impl crate::Resettable for TfrSpec {
    const RESET_VALUE: u32 = 0;
}
