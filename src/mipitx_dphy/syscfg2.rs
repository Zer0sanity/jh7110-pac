#[doc = "Register `syscfg2` reader"]
pub type R = crate::R<Syscfg2Spec>;
#[doc = "Register `syscfg2` writer"]
pub type W = crate::W<Syscfg2Spec>;
#[doc = "Field `mposv(32-46)` reader - MPOSV: u0_mipitx_dphy_MPOSVn"]
pub type MposvR = crate::BitReader;
#[doc = "Field `rgs_cdtx(_pll_fm_cplt,_pll_fm_over,_pll_fm_under,_pll_unlock)` reader - RGS CDTX: u0_mipitx_dphy_RGS_CDTX"]
pub type RgsCdtxR = crate::BitReader;
#[doc = "Field `rg_cdtx(_l0n_hstx_res,_l0p_hstx_res)` reader - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
pub type RgCdtxR = crate::FieldReader;
#[doc = "Field `rg_cdtx(_l0n_hstx_res,_l0p_hstx_res)` writer - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
pub type RgCdtxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `mposv32` field"]
    #[inline(always)]
    pub fn mposv(&self, n: u8) -> MposvR {
        #[allow(clippy::no_effect)]
        [(); 15][n as usize];
        MposvR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv_iter(&self) -> impl Iterator<Item = MposvR> + '_ {
        (0..15).map(move |n| MposvR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv32(&self) -> MposvR {
        MposvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv33(&self) -> MposvR {
        MposvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv34(&self) -> MposvR {
        MposvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv35(&self) -> MposvR {
        MposvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv36(&self) -> MposvR {
        MposvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv37(&self) -> MposvR {
        MposvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv38(&self) -> MposvR {
        MposvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv39(&self) -> MposvR {
        MposvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv40(&self) -> MposvR {
        MposvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv41(&self) -> MposvR {
        MposvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv42(&self) -> MposvR {
        MposvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv43(&self) -> MposvR {
        MposvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv44(&self) -> MposvR {
        MposvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv45(&self) -> MposvR {
        MposvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv46(&self) -> MposvR {
        MposvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "RGS CDTX: u0_mipitx_dphy_RGS_CDTX"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rgs_cdtx_pll_fm_cplt` field"]
    #[inline(always)]
    pub fn rgs_cdtx(&self, n: u8) -> RgsCdtxR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RgsCdtxR::new(((self.bits >> (n + 15)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "RGS CDTX: u0_mipitx_dphy_RGS_CDTX"]
    #[inline(always)]
    pub fn rgs_cdtx_iter(&self) -> impl Iterator<Item = RgsCdtxR> + '_ {
        (0..4).map(move |n| RgsCdtxR::new(((self.bits >> (n + 15)) & 1) != 0))
    }
    #[doc = "Bit 15 - RGS CDTX: u0_mipitx_dphy_RGS_CDTX"]
    #[inline(always)]
    pub fn rgs_cdtx_pll_fm_cplt(&self) -> RgsCdtxR {
        RgsCdtxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RGS CDTX: u0_mipitx_dphy_RGS_CDTX"]
    #[inline(always)]
    pub fn rgs_cdtx_pll_fm_over(&self) -> RgsCdtxR {
        RgsCdtxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RGS CDTX: u0_mipitx_dphy_RGS_CDTX"]
    #[inline(always)]
    pub fn rgs_cdtx_pll_fm_under(&self) -> RgsCdtxR {
        RgsCdtxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RGS CDTX: u0_mipitx_dphy_RGS_CDTX"]
    #[inline(always)]
    pub fn rgs_cdtx_pll_unlock(&self) -> RgsCdtxR {
        RgsCdtxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_cdtx_l0n_hstx_res` field"]
    #[inline(always)]
    pub fn rg_cdtx(&self, n: u8) -> RgCdtxR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RgCdtxR::new(((self.bits >> (n * 5 + 19)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_iter(&self) -> impl Iterator<Item = RgCdtxR> + '_ {
        (0..2).map(move |n| RgCdtxR::new(((self.bits >> (n * 5 + 19)) & 0x1f) as u8))
    }
    #[doc = "Bits 19:23 - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l0n_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    pub fn rg_cdtx_l0p_hstx_res(&self) -> RgCdtxR {
        RgCdtxR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_cdtx_l0n_hstx_res` field"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx(&mut self, n: u8) -> RgCdtxW<Syscfg2Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RgCdtxW::new(self, n * 5 + 19)
    }
    #[doc = "Bits 19:23 - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l0n_hstx_res(&mut self) -> RgCdtxW<Syscfg2Spec> {
        RgCdtxW::new(self, 19)
    }
    #[doc = "Bits 24:28 - RG CDTX: u0_mipitx_dphy_RG_CDTX"]
    #[inline(always)]
    #[must_use]
    pub fn rg_cdtx_l0p_hstx_res(&mut self) -> RgCdtxW<Syscfg2Spec> {
        RgCdtxW::new(self, 24)
    }
}
#[doc = "MIPITX DPHY SYSCFG 2: mipitx_apbifsaif_syscfg_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg2Spec;
impl crate::RegisterSpec for Syscfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg2::R`](R) reader structure"]
impl crate::Readable for Syscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg2::W`](W) writer structure"]
impl crate::Writable for Syscfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg2 to value 0x1080_0000"]
impl crate::Resettable for Syscfg2Spec {
    const RESET_VALUE: u32 = 0x1080_0000;
}
