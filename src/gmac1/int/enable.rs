#[doc = "Register `enable` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `enable` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `rgsmiis` reader - RGSMIIS"]
pub type RgsmiisR = crate::BitReader;
#[doc = "Field `rgsmiis` writer - RGSMIIS"]
pub type RgsmiisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pcs_link` reader - PCS Link"]
pub type PcsLinkR = crate::BitReader;
#[doc = "Field `pcs_link` writer - PCS Link"]
pub type PcsLinkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pcs_ane` reader - PCS ANE"]
pub type PcsAneR = crate::BitReader;
#[doc = "Field `pcs_ane` writer - PCS ANE"]
pub type PcsAneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pcs_phy` reader - PCS PHY"]
pub type PcsPhyR = crate::BitReader;
#[doc = "Field `pcs_phy` writer - PCS PHY"]
pub type PcsPhyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pmt_en` reader - PMT Enable"]
pub type PmtEnR = crate::BitReader;
#[doc = "Field `pmt_en` writer - PMT Enable"]
pub type PmtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lpi_en` reader - LPI Enable"]
pub type LpiEnR = crate::BitReader;
#[doc = "Field `lpi_en` writer - LPI Enable"]
pub type LpiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tsie` reader - TSIE"]
pub type TsieR = crate::BitReader;
#[doc = "Field `tsie` writer - TSIE"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - RGSMIIS"]
    #[inline(always)]
    #[must_use]
    pub fn rgsmiis(&mut self) -> RgsmiisW<EnableSpec> {
        RgsmiisW::new(self, 0)
    }
    #[doc = "Bit 1 - PCS Link"]
    #[inline(always)]
    #[must_use]
    pub fn pcs_link(&mut self) -> PcsLinkW<EnableSpec> {
        PcsLinkW::new(self, 1)
    }
    #[doc = "Bit 2 - PCS ANE"]
    #[inline(always)]
    #[must_use]
    pub fn pcs_ane(&mut self) -> PcsAneW<EnableSpec> {
        PcsAneW::new(self, 2)
    }
    #[doc = "Bit 3 - PCS PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pcs_phy(&mut self) -> PcsPhyW<EnableSpec> {
        PcsPhyW::new(self, 3)
    }
    #[doc = "Bit 4 - PMT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmt_en(&mut self) -> PmtEnW<EnableSpec> {
        PmtEnW::new(self, 4)
    }
    #[doc = "Bit 5 - LPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_en(&mut self) -> LpiEnW<EnableSpec> {
        LpiEnW::new(self, 5)
    }
    #[doc = "Bit 12 - TSIE"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TsieW<EnableSpec> {
        TsieW::new(self, 12)
    }
}
#[doc = "MAC Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets enable to value 0x1030"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0x1030;
}
