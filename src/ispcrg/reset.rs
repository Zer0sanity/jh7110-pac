#[doc = "Register `reset[%s]` reader"]
pub type R = crate::R<ResetSpec>;
#[doc = "Register `reset[%s]` writer"]
pub type W = crate::W<ResetSpec>;
#[doc = "Field `rst_u0_ispv2_top_wrapper_rst_p` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstU0Ispv2TopWrapperRstPR = crate::BitReader;
#[doc = "Field `rst_u0_ispv2_top_wrapper_rst_p` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstU0Ispv2TopWrapperRstPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rst_u0_ispv2_top_wrapper_rst_c` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstU0Ispv2TopWrapperRstCR = crate::BitReader;
#[doc = "Field `rst_u0_ispv2_top_wrapper_rst_c` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstU0Ispv2TopWrapperRstCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_m31dphy_hw_rstn` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0M31dphyHwRstnR = crate::BitReader;
#[doc = "Field `rstn_u0_m31dphy_hw_rstn` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0M31dphyHwRstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_m31dphy_rstb09_aon` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0M31dphyRstb09AonR = crate::BitReader;
#[doc = "Field `rstn_u0_m31dphy_rstb09_aon` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0M31dphyRstb09AonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vin_rst_n_pclk` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPclkR = crate::BitReader;
#[doc = "Field `rstn_u0_vin_rst_n_pclk` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vin_rst_n_pixel_clk_if0` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPixelClkIf0R = crate::BitReader;
#[doc = "Field `rstn_u0_vin_rst_n_pixel_clk_if0` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPixelClkIf0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vin_rst_n_pixel_clk_if1` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPixelClkIf1R = crate::BitReader;
#[doc = "Field `rstn_u0_vin_rst_n_pixel_clk_if1` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPixelClkIf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vin_rst_n_pixel_clk_if2` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPixelClkIf2R = crate::BitReader;
#[doc = "Field `rstn_u0_vin_rst_n_pixel_clk_if2` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPixelClkIf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vin_rst_n_pixel_clk_if3` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPixelClkIf3R = crate::BitReader;
#[doc = "Field `rstn_u0_vin_rst_n_pixel_clk_if3` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNPixelClkIf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vin_rst_n_sys_clk` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNSysClkR = crate::BitReader;
#[doc = "Field `rstn_u0_vin_rst_n_sys_clk` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstNSysClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vin_rst_p_axird` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstPAxirdR = crate::BitReader;
#[doc = "Field `rstn_u0_vin_rst_p_axird` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstPAxirdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_vin_rst_p_axiwr` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstPAxiwrR = crate::BitReader;
#[doc = "Field `rstn_u0_vin_rst_p_axiwr` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0VinRstPAxiwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rst_u0_ispv2_top_wrapper_rst_p(&self) -> RstU0Ispv2TopWrapperRstPR {
        RstU0Ispv2TopWrapperRstPR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rst_u0_ispv2_top_wrapper_rst_c(&self) -> RstU0Ispv2TopWrapperRstCR {
        RstU0Ispv2TopWrapperRstCR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_m31dphy_hw_rstn(&self) -> RstnU0M31dphyHwRstnR {
        RstnU0M31dphyHwRstnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_m31dphy_rstb09_aon(&self) -> RstnU0M31dphyRstb09AonR {
        RstnU0M31dphyRstb09AonR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vin_rst_n_pclk(&self) -> RstnU0VinRstNPclkR {
        RstnU0VinRstNPclkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vin_rst_n_pixel_clk_if0(&self) -> RstnU0VinRstNPixelClkIf0R {
        RstnU0VinRstNPixelClkIf0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vin_rst_n_pixel_clk_if1(&self) -> RstnU0VinRstNPixelClkIf1R {
        RstnU0VinRstNPixelClkIf1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vin_rst_n_pixel_clk_if2(&self) -> RstnU0VinRstNPixelClkIf2R {
        RstnU0VinRstNPixelClkIf2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vin_rst_n_pixel_clk_if3(&self) -> RstnU0VinRstNPixelClkIf3R {
        RstnU0VinRstNPixelClkIf3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vin_rst_n_sys_clk(&self) -> RstnU0VinRstNSysClkR {
        RstnU0VinRstNSysClkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vin_rst_p_axird(&self) -> RstnU0VinRstPAxirdR {
        RstnU0VinRstPAxirdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_vin_rst_p_axiwr(&self) -> RstnU0VinRstPAxiwrR {
        RstnU0VinRstPAxiwrR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_ispv2_top_wrapper_rst_p(&mut self) -> RstU0Ispv2TopWrapperRstPW<ResetSpec> {
        RstU0Ispv2TopWrapperRstPW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_u0_ispv2_top_wrapper_rst_c(&mut self) -> RstU0Ispv2TopWrapperRstCW<ResetSpec> {
        RstU0Ispv2TopWrapperRstCW::new(self, 1)
    }
    #[doc = "Bit 2 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_m31dphy_hw_rstn(&mut self) -> RstnU0M31dphyHwRstnW<ResetSpec> {
        RstnU0M31dphyHwRstnW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_m31dphy_rstb09_aon(&mut self) -> RstnU0M31dphyRstb09AonW<ResetSpec> {
        RstnU0M31dphyRstb09AonW::new(self, 3)
    }
    #[doc = "Bit 4 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vin_rst_n_pclk(&mut self) -> RstnU0VinRstNPclkW<ResetSpec> {
        RstnU0VinRstNPclkW::new(self, 4)
    }
    #[doc = "Bit 5 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vin_rst_n_pixel_clk_if0(&mut self) -> RstnU0VinRstNPixelClkIf0W<ResetSpec> {
        RstnU0VinRstNPixelClkIf0W::new(self, 5)
    }
    #[doc = "Bit 6 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vin_rst_n_pixel_clk_if1(&mut self) -> RstnU0VinRstNPixelClkIf1W<ResetSpec> {
        RstnU0VinRstNPixelClkIf1W::new(self, 6)
    }
    #[doc = "Bit 7 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vin_rst_n_pixel_clk_if2(&mut self) -> RstnU0VinRstNPixelClkIf2W<ResetSpec> {
        RstnU0VinRstNPixelClkIf2W::new(self, 7)
    }
    #[doc = "Bit 8 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vin_rst_n_pixel_clk_if3(&mut self) -> RstnU0VinRstNPixelClkIf3W<ResetSpec> {
        RstnU0VinRstNPixelClkIf3W::new(self, 8)
    }
    #[doc = "Bit 9 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vin_rst_n_sys_clk(&mut self) -> RstnU0VinRstNSysClkW<ResetSpec> {
        RstnU0VinRstNSysClkW::new(self, 9)
    }
    #[doc = "Bit 10 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vin_rst_p_axird(&mut self) -> RstnU0VinRstPAxirdW<ResetSpec> {
        RstnU0VinRstPAxirdW::new(self, 10)
    }
    #[doc = "Bit 11 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_vin_rst_p_axiwr(&mut self) -> RstnU0VinRstPAxiwrW<ResetSpec> {
        RstnU0VinRstPAxiwrW::new(self, 11)
    }
}
#[doc = "ISPCRG Reset registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetSpec;
impl crate::RegisterSpec for ResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset::R`](R) reader structure"]
impl crate::Readable for ResetSpec {}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for ResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets reset[%s]
to value 0x0fff"]
impl crate::Resettable for ResetSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
