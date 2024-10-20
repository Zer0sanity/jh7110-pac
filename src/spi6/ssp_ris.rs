#[doc = "Register `ssp_ris` reader"]
pub type R = crate::R<SspRisSpec>;
#[doc = "Field `rorris` reader - Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
pub type RorrisR = crate::BitReader;
#[doc = "Field `rtris` reader - Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `rxris` reader - Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `txris` reader - Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
pub type TxrisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn rorris(&self) -> RorrisR {
        RorrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "The SSPRIS register is the raw interrupt status register. It is a RO register. On a read this register gives the current raw status value of the corresponding interrupt prior to masking. A write has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspRisSpec;
impl crate::RegisterSpec for SspRisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_ris::R`](R) reader structure"]
impl crate::Readable for SspRisSpec {}
#[doc = "`reset()` method sets ssp_ris to value 0"]
impl crate::Resettable for SspRisSpec {
    const RESET_VALUE: u16 = 0;
}
