#[doc = "Register `ssp_sr` reader"]
pub type R = crate::R<SspSrSpec>;
#[doc = "Field `tfe` reader - Transmit FIFO empty, RO - 0: Transmit FIFO is not empty, 1: Transmit FIFO is empty."]
pub type TfeR = crate::BitReader;
#[doc = "Field `tnf` reader - Transmit FIFO not full, RO - 0: Transmit FIFO is full, 1: Transmit FIFO is not full."]
pub type TnfR = crate::BitReader;
#[doc = "Field `rne` reader - Receive FIFO not empty, RO - 0: Receive FIFO is empty, 1: Receive FIFO is not empty."]
pub type RneR = crate::BitReader;
#[doc = "Field `rff` reader - Receive FIFO full, RO - 0: Receive FIFO is not full, 1: Receive FIFO is full."]
pub type RffR = crate::BitReader;
#[doc = "Field `bsy` reader - PrimeCell SSP busy flag, RO - 0: SSP is idle, 1: SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub type BsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty, RO - 0: Transmit FIFO is not empty, 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full, RO - 0: Transmit FIFO is full, 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO not empty, RO - 0: Receive FIFO is empty, 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO full, RO - 0: Receive FIFO is not full, 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PrimeCell SSP busy flag, RO - 0: SSP is idle, 1: SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "SSPSR is a RO status register that contains bits that indicate the FIFO fill status and the PrimeCell SSP busy status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspSrSpec;
impl crate::RegisterSpec for SspSrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_sr::R`](R) reader structure"]
impl crate::Readable for SspSrSpec {}
#[doc = "`reset()` method sets ssp_sr to value 0"]
impl crate::Resettable for SspSrSpec {
    const RESET_VALUE: u16 = 0;
}
