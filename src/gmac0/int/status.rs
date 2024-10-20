#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `rgsmiis` reader - RGSMIIS"]
pub type RgsmiisR = crate::BitReader;
#[doc = "Field `pcs_link` reader - PCS Link"]
pub type PcsLinkR = crate::BitReader;
#[doc = "Field `pcs_ane` reader - PCS ANE"]
pub type PcsAneR = crate::BitReader;
#[doc = "Field `pcs_phy` reader - PCS PHY"]
pub type PcsPhyR = crate::BitReader;
#[doc = "Field `pmt_en` reader - PMT Enable"]
pub type PmtEnR = crate::BitReader;
#[doc = "Field `lpi_en` reader - LPI Enable"]
pub type LpiEnR = crate::BitReader;
#[doc = "Field `tsie` reader - TSIE"]
pub type TsieR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RGSMIIS"]
    #[inline(always)]
    pub fn rgsmiis(&self) -> RgsmiisR {
        RgsmiisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCS Link"]
    #[inline(always)]
    pub fn pcs_link(&self) -> PcsLinkR {
        PcsLinkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCS ANE"]
    #[inline(always)]
    pub fn pcs_ane(&self) -> PcsAneR {
        PcsAneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCS PHY"]
    #[inline(always)]
    pub fn pcs_phy(&self) -> PcsPhyR {
        PcsPhyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMT Enable"]
    #[inline(always)]
    pub fn pmt_en(&self) -> PmtEnR {
        PmtEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPI Enable"]
    #[inline(always)]
    pub fn lpi_en(&self) -> LpiEnR {
        LpiEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "MAC Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status to value 0x07"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x07;
}
