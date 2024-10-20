#[doc = "Register `syscfg9` reader"]
pub type R = crate::R<Syscfg9Spec>;
#[doc = "Register `syscfg9` writer"]
pub type W = crate::W<Syscfg9Spec>;
#[doc = "Field `rg(_clane_hs_zero_time,_dlane_hs_pre_time,_dlane_hs_trail_time,_dlane_hs_zero_tim)` reader - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
pub type RgR = crate::FieldReader;
#[doc = "Field `rg(_clane_hs_zero_time,_dlane_hs_pre_time,_dlane_hs_trail_time,_dlane_hs_zero_tim)` writer - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
pub type RgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_clane_hs_zero_time` field"]
    #[inline(always)]
    pub fn rg(&self, n: u8) -> RgR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RgR::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    pub fn rg_iter(&self) -> impl Iterator<Item = RgR> + '_ {
        (0..4).map(move |n| RgR::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    pub fn rg_clane_hs_zero_time(&self) -> RgR {
        RgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    pub fn rg_dlane_hs_pre_time(&self) -> RgR {
        RgR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    pub fn rg_dlane_hs_trail_time(&self) -> RgR {
        RgR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    pub fn rg_dlane_hs_zero_tim(&self) -> RgR {
        RgR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_clane_hs_zero_time` field"]
    #[inline(always)]
    #[must_use]
    pub fn rg(&mut self, n: u8) -> RgW<Syscfg9Spec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RgW::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    #[must_use]
    pub fn rg_clane_hs_zero_time(&mut self) -> RgW<Syscfg9Spec> {
        RgW::new(self, 0)
    }
    #[doc = "Bits 8:15 - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    #[must_use]
    pub fn rg_dlane_hs_pre_time(&mut self) -> RgW<Syscfg9Spec> {
        RgW::new(self, 8)
    }
    #[doc = "Bits 16:23 - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    #[must_use]
    pub fn rg_dlane_hs_trail_time(&mut self) -> RgW<Syscfg9Spec> {
        RgW::new(self, 16)
    }
    #[doc = "Bits 24:31 - RG CLANE and DLANE TIME: u0_mipitx_dphy_RG_CLANE and u0_mipitx_dphy_RG_DLANE"]
    #[inline(always)]
    #[must_use]
    pub fn rg_dlane_hs_zero_tim(&mut self) -> RgW<Syscfg9Spec> {
        RgW::new(self, 24)
    }
}
#[doc = "MIPITX DPHY CLANE and DLANE: mipitx_apbifsaif_syscfg_36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg9Spec;
impl crate::RegisterSpec for Syscfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg9::R`](R) reader structure"]
impl crate::Readable for Syscfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg9::W`](W) writer structure"]
impl crate::Writable for Syscfg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg9 to value 0x2116_0e16"]
impl crate::Resettable for Syscfg9Spec {
    const RESET_VALUE: u32 = 0x2116_0e16;
}
