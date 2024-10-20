#[doc = "Register `syscfg8` reader"]
pub type R = crate::R<Syscfg8Spec>;
#[doc = "Register `syscfg8` writer"]
pub type W = crate::W<Syscfg8Spec>;
#[doc = "Field `rg_clane(_hs_clk_post_time,_hs_clk_pre_time,_hs_pre_time,_hs_trail_time)` reader - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
pub type RgClaneR = crate::FieldReader;
#[doc = "Field `rg_clane(_hs_clk_post_time,_hs_clk_pre_time,_hs_pre_time,_hs_trail_time)` writer - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
pub type RgClaneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_clane_hs_clk_post_time` field"]
    #[inline(always)]
    pub fn rg_clane(&self, n: u8) -> RgClaneR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RgClaneR::new(((self.bits >> (n * 8)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    pub fn rg_clane_iter(&self) -> impl Iterator<Item = RgClaneR> + '_ {
        (0..4).map(move |n| RgClaneR::new(((self.bits >> (n * 8)) & 0xff) as u8))
    }
    #[doc = "Bits 0:7 - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    pub fn rg_clane_hs_clk_post_time(&self) -> RgClaneR {
        RgClaneR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    pub fn rg_clane_hs_clk_pre_time(&self) -> RgClaneR {
        RgClaneR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    pub fn rg_clane_hs_pre_time(&self) -> RgClaneR {
        RgClaneR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    pub fn rg_clane_hs_trail_time(&self) -> RgClaneR {
        RgClaneR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rg_clane_hs_clk_post_time` field"]
    #[inline(always)]
    #[must_use]
    pub fn rg_clane(&mut self, n: u8) -> RgClaneW<Syscfg8Spec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        RgClaneW::new(self, n * 8)
    }
    #[doc = "Bits 0:7 - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    #[must_use]
    pub fn rg_clane_hs_clk_post_time(&mut self) -> RgClaneW<Syscfg8Spec> {
        RgClaneW::new(self, 0)
    }
    #[doc = "Bits 8:15 - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    #[must_use]
    pub fn rg_clane_hs_clk_pre_time(&mut self) -> RgClaneW<Syscfg8Spec> {
        RgClaneW::new(self, 8)
    }
    #[doc = "Bits 16:23 - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    #[must_use]
    pub fn rg_clane_hs_pre_time(&mut self) -> RgClaneW<Syscfg8Spec> {
        RgClaneW::new(self, 16)
    }
    #[doc = "Bits 24:31 - RG CLANE: u0_mipitx_dphy_RG_CLANE"]
    #[inline(always)]
    #[must_use]
    pub fn rg_clane_hs_trail_time(&mut self) -> RgClaneW<Syscfg8Spec> {
        RgClaneW::new(self, 24)
    }
}
#[doc = "MIPITX DPHY CLANE: mipitx_apbifsaif_syscfg_32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg8Spec;
impl crate::RegisterSpec for Syscfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg8::R`](R) reader structure"]
impl crate::Readable for Syscfg8Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg8::W`](W) writer structure"]
impl crate::Writable for Syscfg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg8 to value 0x530b_0000"]
impl crate::Resettable for Syscfg8Spec {
    const RESET_VALUE: u32 = 0x530b_0000;
}
