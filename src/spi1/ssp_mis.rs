#[doc = "Register `ssp_mis` reader"]
pub type R = crate::R<SspMisSpec>;
#[doc = "Field `rormis` reader - Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
pub type RormisR = crate::BitReader;
#[doc = "Field `rtmis` reader - Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
pub type RtmisR = crate::BitReader;
#[doc = "Field `rxmis` reader - Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
pub type RxmisR = crate::BitReader;
#[doc = "Field `txmis` reader - Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
pub type TxmisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn rormis(&self) -> RormisR {
        RormisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "The SSPMIS register is the masked interrupt status register. It is a RO register. On a read this register gives the current masked status value of the corresponding interrupt. A write has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspMisSpec;
impl crate::RegisterSpec for SspMisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_mis::R`](R) reader structure"]
impl crate::Readable for SspMisSpec {}
#[doc = "`reset()` method sets ssp_mis to value 0"]
impl crate::Resettable for SspMisSpec {
    const RESET_VALUE: u16 = 0;
}
