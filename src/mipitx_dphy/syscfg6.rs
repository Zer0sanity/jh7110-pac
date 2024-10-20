#[doc = "Register `syscfg6` reader"]
pub type R = crate::R<Syscfg6Spec>;
#[doc = "Register `syscfg6` writer"]
pub type W = crate::W<Syscfg6Spec>;
#[doc = "Field `rg_cdtx_pll_fbk_int` reader - RG CDTX PLL FBK INT: u0_mipitx_dphy_RG_CDTX_PLL_FBK_INT"]
pub type RgCdtxPllFbkIntR = crate::FieldReader<u16>;
#[doc = "Field `rg_cdtx_pll_fbk_int` writer - RG CDTX PLL FBK INT: u0_mipitx_dphy_RG_CDTX_PLL_FBK_INT"]
pub type RgCdtxPllFbkIntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `rg_cdtx_pll_fm_en` reader - RG CDTX PLL FM EM: u0_mipitx_dphy_RG_CDTX_PLL_FM_EN"]
pub type RgCdtxPllFmEnR = crate::BitReader;
#[doc = "Field `rg_cdtx_pll_fm_en` writer - RG CDTX PLL FM EM: u0_mipitx_dphy_RG_CDTX_PLL_FM_EN"]
pub type RgCdtxPllFmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rg_cdtx_pll_ldo_stb_x2_en` reader - RG CDTX PLL LDO STB X2 EN: u0_mipitx_dphy_RG_CDTX_PLL_LDO_STB_X2_EN"]
pub type RgCdtxPllLdoStbX2EnR = crate::BitReader;
#[doc = "Field `rg_cdtx_pll_ldo_stb_x2_en` writer - RG CDTX PLL LDO STB X2 EN: u0_mipitx_dphy_RG_CDTX_PLL_LDO_STB_X2_EN"]
pub type RgCdtxPllLdoStbX2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rg_cdtx_pll_pre_div` reader - RG CDTX PLL PRE DIV: u0_mipitx_dphy_RG_CDTX_PLL_PRE_DIV"]
pub type RgCdtxPllPreDivR = crate::FieldReader;
#[doc = "Field `rg_cdtx_pll_pre_div` writer - RG CDTX PLL PRE DIV: u0_mipitx_dphy_RG_CDTX_PLL_PRE_DIV"]
pub type RgCdtxPllPreDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rg_cdtx_pll_ssc_delta` reader - RG CDTX PLL SSC DELTA: u0_mipitx_dphy_RG_CDTX_PLL_SSC_DELTA"]
pub type RgCdtxPllSscDeltaR = crate::FieldReader<u32>;
#[doc = "Field `rg_cdtx_pll_ssc_delta` writer - RG CDTX PLL SSC DELTA: u0_mipitx_dphy_RG_CDTX_PLL_SSC_DELTA"]
pub type RgCdtxPllSscDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:8 - RG CDTX PLL FBK INT: u0_mipitx_dphy_RG_CDTX_PLL_FBK_INT"]
    #[inline(always)]
    pub fn rg_cdtx_pll_fbk_int(&self) -> RgCdtxPllFbkIntR {
        RgCdtxPllFbkIntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - RG CDTX PLL FM EM: u0_mipitx_dphy_RG_CDTX_PLL_FM_EN"]
    #[inline(always)]
    pub fn rg_cdtx_pll_fm_en(&self) -> RgCdtxPllFmEnR {
        RgCdtxPllFmEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RG CDTX PLL LDO STB X2 EN: u0_mipitx_dphy_RG_CDTX_PLL_LDO_STB_X2_EN"]
    #[inline(always)]
    pub fn rg_cdtx_pll_ldo_stb_x2_en(&self) -> RgCdtxPllLdoStbX2EnR {
        RgCdtxPllLdoStbX2EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - RG CDTX PLL PRE DIV: u0_mipitx_dphy_RG_CDTX_PLL_PRE_DIV"]
    #[inline(always)]
    pub fn rg_cdtx_pll_pre_div(&self) -> RgCdtxPllPreDivR {
        RgCdtxPllPreDivR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:30 - RG CDTX PLL SSC DELTA: u0_mipitx_dphy_RG_CDTX_PLL_SSC_DELTA"]
    #[inline(always)]
    pub fn rg_cdtx_pll_ssc_delta(&self) -> RgCdtxPllSscDeltaR {
        RgCdtxPllSscDeltaR::new((self.bits >> 13) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:8 - RG CDTX PLL FBK INT: u0_mipitx_dphy_RG_CDTX_PLL_FBK_INT"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_fbk_int(&mut self) -> RgCdtxPllFbkIntW<Syscfg6Spec> {
        RgCdtxPllFbkIntW::new(self, 0)
    }
    #[doc = "Bit 9 - RG CDTX PLL FM EM: u0_mipitx_dphy_RG_CDTX_PLL_FM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_fm_en(&mut self) -> RgCdtxPllFmEnW<Syscfg6Spec> {
        RgCdtxPllFmEnW::new(self, 9)
    }
    #[doc = "Bit 10 - RG CDTX PLL LDO STB X2 EN: u0_mipitx_dphy_RG_CDTX_PLL_LDO_STB_X2_EN"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_ldo_stb_x2_en(&mut self) -> RgCdtxPllLdoStbX2EnW<Syscfg6Spec> {
        RgCdtxPllLdoStbX2EnW::new(self, 10)
    }
    #[doc = "Bits 11:12 - RG CDTX PLL PRE DIV: u0_mipitx_dphy_RG_CDTX_PLL_PRE_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_pre_div(&mut self) -> RgCdtxPllPreDivW<Syscfg6Spec> {
        RgCdtxPllPreDivW::new(self, 11)
    }
    #[doc = "Bits 13:30 - RG CDTX PLL SSC DELTA: u0_mipitx_dphy_RG_CDTX_PLL_SSC_DELTA"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_ssc_delta(&mut self) -> RgCdtxPllSscDeltaW<Syscfg6Spec> {
        RgCdtxPllSscDeltaW::new(self, 13)
    }
}
#[doc = "MIPITX DPHY SYSCFG 6: mipitx_apbifsaif_syscfg_24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg6Spec;
impl crate::RegisterSpec for Syscfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg6::R`](R) reader structure"]
impl crate::Readable for Syscfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg6::W`](W) writer structure"]
impl crate::Writable for Syscfg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg6 to value 0x0864"]
impl crate::Resettable for Syscfg6Spec {
    const RESET_VALUE: u32 = 0x0864;
}
