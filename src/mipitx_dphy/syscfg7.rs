#[doc = "Register `syscfg7` reader"]
pub type R = crate::R<Syscfg7Spec>;
#[doc = "Register `syscfg7` writer"]
pub type W = crate::W<Syscfg7Spec>;
#[doc = "Field `rg_cdtx_pll_ssc_delta_init` reader - RG CDTX PLL SSC DELTA INIT: u0_mipitx_dphy_RG_CDTX_PLL_SSC_DELTA_INIT"]
pub type RgCdtxPllSscDeltaInitR = crate::FieldReader<u32>;
#[doc = "Field `rg_cdtx_pll_ssc_delta_init` writer - RG CDTX PLL SSC DELTA INIT: u0_mipitx_dphy_RG_CDTX_PLL_SSC_DELTA_INIT"]
pub type RgCdtxPllSscDeltaInitW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `rg_cdtx_pll_ssc_en` reader - RG CDTX PLL SSC EN: u0_mipitx_dphy_RG_CDTX_PLL_SSC_EN"]
pub type RgCdtxPllSscEnR = crate::BitReader;
#[doc = "Field `rg_cdtx_pll_ssc_en` writer - RG CDTX PLL SSC EN: u0_mipitx_dphy_RG_CDTX_PLL_SSC_EN"]
pub type RgCdtxPllSscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rg_cdtx_pll_ssc_prd` reader - RG CDTX PLL SSC PRD: u0_mipitx_dphy_RG_CDTX_PLL_SSC_PRD"]
pub type RgCdtxPllSscPrdR = crate::FieldReader<u16>;
#[doc = "Field `rg_cdtx_pll_ssc_prd` writer - RG CDTX PLL SSC PRD: u0_mipitx_dphy_RG_CDTX_PLL_SSC_PRD"]
pub type RgCdtxPllSscPrdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:17 - RG CDTX PLL SSC DELTA INIT: u0_mipitx_dphy_RG_CDTX_PLL_SSC_DELTA_INIT"]
    #[inline(always)]
    pub fn rg_cdtx_pll_ssc_delta_init(&self) -> RgCdtxPllSscDeltaInitR {
        RgCdtxPllSscDeltaInitR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bit 18 - RG CDTX PLL SSC EN: u0_mipitx_dphy_RG_CDTX_PLL_SSC_EN"]
    #[inline(always)]
    pub fn rg_cdtx_pll_ssc_en(&self) -> RgCdtxPllSscEnR {
        RgCdtxPllSscEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:28 - RG CDTX PLL SSC PRD: u0_mipitx_dphy_RG_CDTX_PLL_SSC_PRD"]
    #[inline(always)]
    pub fn rg_cdtx_pll_ssc_prd(&self) -> RgCdtxPllSscPrdR {
        RgCdtxPllSscPrdR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - RG CDTX PLL SSC DELTA INIT: u0_mipitx_dphy_RG_CDTX_PLL_SSC_DELTA_INIT"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_ssc_delta_init(&mut self) -> RgCdtxPllSscDeltaInitW<Syscfg7Spec> {
        RgCdtxPllSscDeltaInitW::new(self, 0)
    }
    #[doc = "Bit 18 - RG CDTX PLL SSC EN: u0_mipitx_dphy_RG_CDTX_PLL_SSC_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_ssc_en(&mut self) -> RgCdtxPllSscEnW<Syscfg7Spec> {
        RgCdtxPllSscEnW::new(self, 18)
    }
    #[doc = "Bits 19:28 - RG CDTX PLL SSC PRD: u0_mipitx_dphy_RG_CDTX_PLL_SSC_PRD"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_ssc_prd(&mut self) -> RgCdtxPllSscPrdW<Syscfg7Spec> {
        RgCdtxPllSscPrdW::new(self, 19)
    }
}
#[doc = "MIPITX DPHY SYSCFG 7: mipitx_apbifsaif_syscfg_28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg7Spec;
impl crate::RegisterSpec for Syscfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg7::R`](R) reader structure"]
impl crate::Readable for Syscfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg7::W`](W) writer structure"]
impl crate::Writable for Syscfg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg7 to value 0"]
impl crate::Resettable for Syscfg7Spec {
    const RESET_VALUE: u32 = 0;
}
