#[doc = "Register `syscfg3` reader"]
pub type R = crate::R<Syscfg3Spec>;
#[doc = "Register `syscfg3` writer"]
pub type W = crate::W<Syscfg3Spec>;
#[doc = "Field `rg_cdtx(_l1n_hstx_res,_l1p_hstx_res,_l2n_hstx_res,_l2p_hstx_res,_l3n_hstx_res,_l3p_hstx_res)` reader - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
pub type RgCdtxR = crate::FieldReader;
#[doc = "Field `rg_cdtx(_l1n_hstx_res,_l1p_hstx_res,_l2n_hstx_res,_l2p_hstx_res,_l3n_hstx_res,_l3p_hstx_res)` writer - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
pub type RgCdtxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_cdtx_l1n_hstx_res` field"]
    #[inline(always)]
    pub fn rg_cdtx(&self, n: u8) -> RgCdtxR {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        RgCdtxR::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_iter(&self) -> impl Iterator<Item = RgCdtxR> + '_ {
        (0..6).map(move |n| RgCdtxR::new(((self.bits >> (n * 5)) & 0x1f) as u8))
    }
    #[doc = "Bits 0:4 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l1n_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l1p_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l2n_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l2p_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l3n_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l3p_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_cdtx_l1n_hstx_res` field"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx(&mut self, n: u8) -> RgCdtxW<Syscfg3Spec> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        RgCdtxW::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l1n_hstx_res(&mut self) -> RgCdtxW<Syscfg3Spec> {
        RgCdtxW::new(self, 0)
    }
    #[doc = "Bits 5:9 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l1p_hstx_res(&mut self) -> RgCdtxW<Syscfg3Spec> {
        RgCdtxW::new(self, 5)
    }
    #[doc = "Bits 10:14 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l2n_hstx_res(&mut self) -> RgCdtxW<Syscfg3Spec> {
        RgCdtxW::new(self, 10)
    }
    #[doc = "Bits 15:19 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l2p_hstx_res(&mut self) -> RgCdtxW<Syscfg3Spec> {
        RgCdtxW::new(self, 15)
    }
    #[doc = "Bits 20:24 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l3n_hstx_res(&mut self) -> RgCdtxW<Syscfg3Spec> {
        RgCdtxW::new(self, 20)
    }
    #[doc = "Bits 25:29 - RG CDTX L1-L3: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l3p_hstx_res(&mut self) -> RgCdtxW<Syscfg3Spec> {
        RgCdtxW::new(self, 25)
    }
}
#[doc = "MIPITX DPHY SYSCFG 3: mipitx_apbifsaif_syscfg_12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg3Spec;
impl crate::RegisterSpec for Syscfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg3::R`](R) reader structure"]
impl crate::Readable for Syscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg3::W`](W) writer structure"]
impl crate::Writable for Syscfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg3 to value 0x2108_4210"]
impl crate::Resettable for Syscfg3Spec {
    const RESET_VALUE: u32 = 0x2108_4210;
}
