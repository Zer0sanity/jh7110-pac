#[doc = "Register `syscfg5` reader"]
pub type R = crate::R<Syscfg5Spec>;
#[doc = "Register `syscfg5` writer"]
pub type W = crate::W<Syscfg5Spec>;
#[doc = "Field `rg_cdtx_pll_fbk_fra` reader - RG CDTX PLL FBK FRA: u0_mipitx_dphy_RG_CDTX_PLL_FBK_FRA"]
pub type RgCdtxPllFbkFraR = crate::FieldReader<u32>;
#[doc = "Field `rg_cdtx_pll_fbk_fra` writer - RG CDTX PLL FBK FRA: u0_mipitx_dphy_RG_CDTX_PLL_FBK_FRA"]
pub type RgCdtxPllFbkFraW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - RG CDTX PLL FBK FRA: u0_mipitx_dphy_RG_CDTX_PLL_FBK_FRA"]
    #[inline(always)]
    pub fn rg_cdtx_pll_fbk_fra(&self) -> RgCdtxPllFbkFraR {
        RgCdtxPllFbkFraR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - RG CDTX PLL FBK FRA: u0_mipitx_dphy_RG_CDTX_PLL_FBK_FRA"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_pll_fbk_fra(&mut self) -> RgCdtxPllFbkFraW<Syscfg5Spec> {
        RgCdtxPllFbkFraW::new(self, 0)
    }
}
#[doc = "MIPITX DPHY SYSCFG 5: mipitx_apbifsaif_syscfg_20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg5Spec;
impl crate::RegisterSpec for Syscfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg5::R`](R) reader structure"]
impl crate::Readable for Syscfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg5::W`](W) writer structure"]
impl crate::Writable for Syscfg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg5 to value 0"]
impl crate::Resettable for Syscfg5Spec {
    const RESET_VALUE: u32 = 0;
}
