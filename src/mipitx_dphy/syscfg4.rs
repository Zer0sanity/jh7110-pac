#[doc = "Register `syscfg4` reader"]
pub type R = crate::R<Syscfg4Spec>;
#[doc = "Register `syscfg4` writer"]
pub type W = crate::W<Syscfg4Spec>;
#[doc = "Field `rg_cdtx(_l4n_hstx_res,_l4p_hstx_res)` reader - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
pub type RgCdtxR = crate::FieldReader;
#[doc = "Field `rg_cdtx(_l4n_hstx_res,_l4p_hstx_res)` writer - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
pub type RgCdtxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_cdtx_l4n_hstx_res` field"]
    #[inline(always)]
    pub fn rg_cdtx(&self, n: u8) -> RgCdtxR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RgCdtxR::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_iter(&self) -> impl Iterator<Item = RgCdtxR> + '_ {
        (0..2).map(move |n| RgCdtxR::new(((self.bits >> (n * 5)) & 0x1f) as u8))
    }
    #[doc = "Bits 0:4 - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l4n_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l4p_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_cdtx_l4n_hstx_res` field"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx(&mut self, n: u8) -> RgCdtxW<Syscfg4Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RgCdtxW::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l4n_hstx_res(&mut self) -> RgCdtxW<Syscfg4Spec> {
        RgCdtxW::new(self, 0)
    }
    #[doc = "Bits 5:9 - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l4p_hstx_res(&mut self) -> RgCdtxW<Syscfg4Spec> {
        RgCdtxW::new(self, 5)
    }
}
#[doc = "MIPITX DPHY SYSCFG 4: mipitx_apbifsaif_syscfg_16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg4Spec;
impl crate::RegisterSpec for Syscfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg4::R`](R) reader structure"]
impl crate::Readable for Syscfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg4::W`](W) writer structure"]
impl crate::Writable for Syscfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg4 to value 0x0210"]
impl crate::Resettable for Syscfg4Spec {
    const RESET_VALUE: u32 = 0x0210;
}
