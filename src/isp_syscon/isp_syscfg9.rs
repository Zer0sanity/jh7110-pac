#[doc = "Register `isp_syscfg9` reader"]
pub type R = crate::R<IspSyscfg9Spec>;
#[doc = "Register `isp_syscfg9` writer"]
pub type W = crate::W<IspSyscfg9Spec>;
#[doc = "Field `u0_vin_cfg_color_bar_en` reader - Set this bit to 1 to use the color bar for test."]
pub type U0VinCfgColorBarEnR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_color_bar_en` writer - Set this bit to 1 to use the color bar for test."]
pub type U0VinCfgColorBarEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_dvp_hs_pos` reader - Use DVP to AXI - 0: HS is low valid, 1: HS is high valid."]
pub type U0VinCfgDvpHsPosR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_dvp_hs_pos` writer - Use DVP to AXI - 0: HS is low valid, 1: HS is high valid."]
pub type U0VinCfgDvpHsPosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_dvp_swap_en` reader - Set this bit to 1 to enable DVP swap."]
pub type U0VinCfgDvpSwapEnR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_dvp_swap_en` writer - Set this bit to 1 to enable DVP swap."]
pub type U0VinCfgDvpSwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_dvp_vs_pos` reader - Use DVP to AXI - 0: VS is low valid, 1: VS is high valid."]
pub type U0VinCfgDvpVsPosR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_dvp_vs_pos` writer - Use DVP to AXI - 0: VS is low valid, 1: VS is high valid."]
pub type U0VinCfgDvpVsPosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_gen_en_axird` reader - Set this bit to use AXI input for ISP and generate test image."]
pub type U0VinCfgGenEnAxirdR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_gen_en_axird` writer - Set this bit to use AXI input for ISP and generate test image."]
pub type U0VinCfgGenEnAxirdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_isp_dvp_en0` reader - Set this bit to use DVP input for ISP."]
pub type U0VinCfgIspDvpEn0R = crate::BitReader;
#[doc = "Field `u0_vin_cfg_isp_dvp_en0` writer - Set this bit to use DVP input for ISP."]
pub type U0VinCfgIspDvpEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_mipi_byte_en_isp0` reader - Set to 1 for dvp_clk_inv, the DVP clock inverter register."]
pub type U0VinCfgMipiByteEnIsp0R = crate::FieldReader;
#[doc = "Field `u0_vin_cfg_mipi_byte_en_isp0` writer - Set to 1 for dvp_clk_inv, the DVP clock inverter register."]
pub type U0VinCfgMipiByteEnIsp0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_vin_cfg_mipi_channel_sel0` reader - Select 1 channel output of the 8 MIPI channels."]
pub type U0VinCfgMipiChannelSel0R = crate::FieldReader;
#[doc = "Field `u0_vin_cfg_mipi_channel_sel0` writer - Select 1 channel output of the 8 MIPI channels."]
pub type U0VinCfgMipiChannelSel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u0_vin_cfg_mipi_header_en0` reader - Set this bit to 1 to add 10 bits to bit 2, so 1 pixel equals 12 bits."]
pub type U0VinCfgMipiHeaderEn0R = crate::BitReader;
#[doc = "Field `u0_vin_cfg_mipi_header_en0` writer - Set this bit to 1 to add 10 bits to bit 2, so 1 pixel equals 12 bits."]
pub type U0VinCfgMipiHeaderEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_pix_num` reader - VIN AXI to ISP MIPI port, 12-bit data configuration - 00: axi_data\\[11:0\\], 01: axi_data\\[9:0\\], 10: axi_data\\[7:0\\], 11: axi_data\\[5:0\\]"]
pub type U0VinCfgPixNumR = crate::FieldReader;
#[doc = "Field `u0_vin_cfg_pix_num` writer - VIN AXI to ISP MIPI port, 12-bit data configuration - 00: axi_data\\[11:0\\], 01: axi_data\\[9:0\\], 10: axi_data\\[7:0\\], 11: axi_data\\[5:0\\]"]
pub type U0VinCfgPixNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u0_vin_generic_sp` reader - This configuration is not used by JH7110."]
pub type U0VinGenericSpR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to use the color bar for test."]
    #[inline(always)]
    pub fn u0_vin_cfg_color_bar_en(&self) -> U0VinCfgColorBarEnR {
        U0VinCfgColorBarEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use DVP to AXI - 0: HS is low valid, 1: HS is high valid."]
    #[inline(always)]
    pub fn u0_vin_cfg_dvp_hs_pos(&self) -> U0VinCfgDvpHsPosR {
        U0VinCfgDvpHsPosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable DVP swap."]
    #[inline(always)]
    pub fn u0_vin_cfg_dvp_swap_en(&self) -> U0VinCfgDvpSwapEnR {
        U0VinCfgDvpSwapEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use DVP to AXI - 0: VS is low valid, 1: VS is high valid."]
    #[inline(always)]
    pub fn u0_vin_cfg_dvp_vs_pos(&self) -> U0VinCfgDvpVsPosR {
        U0VinCfgDvpVsPosR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to use AXI input for ISP and generate test image."]
    #[inline(always)]
    pub fn u0_vin_cfg_gen_en_axird(&self) -> U0VinCfgGenEnAxirdR {
        U0VinCfgGenEnAxirdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to use DVP input for ISP."]
    #[inline(always)]
    pub fn u0_vin_cfg_isp_dvp_en0(&self) -> U0VinCfgIspDvpEn0R {
        U0VinCfgIspDvpEn0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set to 1 for dvp_clk_inv, the DVP clock inverter register."]
    #[inline(always)]
    pub fn u0_vin_cfg_mipi_byte_en_isp0(&self) -> U0VinCfgMipiByteEnIsp0R {
        U0VinCfgMipiByteEnIsp0R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Select 1 channel output of the 8 MIPI channels."]
    #[inline(always)]
    pub fn u0_vin_cfg_mipi_channel_sel0(&self) -> U0VinCfgMipiChannelSel0R {
        U0VinCfgMipiChannelSel0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to 1 to add 10 bits to bit 2, so 1 pixel equals 12 bits."]
    #[inline(always)]
    pub fn u0_vin_cfg_mipi_header_en0(&self) -> U0VinCfgMipiHeaderEn0R {
        U0VinCfgMipiHeaderEn0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - VIN AXI to ISP MIPI port, 12-bit data configuration - 00: axi_data\\[11:0\\], 01: axi_data\\[9:0\\], 10: axi_data\\[7:0\\], 11: axi_data\\[5:0\\]"]
    #[inline(always)]
    pub fn u0_vin_cfg_pix_num(&self) -> U0VinCfgPixNumR {
        U0VinCfgPixNumR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:26 - This configuration is not used by JH7110."]
    #[inline(always)]
    pub fn u0_vin_generic_sp(&self) -> U0VinGenericSpR {
        U0VinGenericSpR::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to use the color bar for test."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_color_bar_en(&mut self) -> U0VinCfgColorBarEnW<IspSyscfg9Spec> {
        U0VinCfgColorBarEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Use DVP to AXI - 0: HS is low valid, 1: HS is high valid."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_dvp_hs_pos(&mut self) -> U0VinCfgDvpHsPosW<IspSyscfg9Spec> {
        U0VinCfgDvpHsPosW::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable DVP swap."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_dvp_swap_en(&mut self) -> U0VinCfgDvpSwapEnW<IspSyscfg9Spec> {
        U0VinCfgDvpSwapEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Use DVP to AXI - 0: VS is low valid, 1: VS is high valid."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_dvp_vs_pos(&mut self) -> U0VinCfgDvpVsPosW<IspSyscfg9Spec> {
        U0VinCfgDvpVsPosW::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to use AXI input for ISP and generate test image."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_gen_en_axird(&mut self) -> U0VinCfgGenEnAxirdW<IspSyscfg9Spec> {
        U0VinCfgGenEnAxirdW::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to use DVP input for ISP."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_isp_dvp_en0(&mut self) -> U0VinCfgIspDvpEn0W<IspSyscfg9Spec> {
        U0VinCfgIspDvpEn0W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Set to 1 for dvp_clk_inv, the DVP clock inverter register."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_mipi_byte_en_isp0(&mut self) -> U0VinCfgMipiByteEnIsp0W<IspSyscfg9Spec> {
        U0VinCfgMipiByteEnIsp0W::new(self, 6)
    }
    #[doc = "Bits 8:11 - Select 1 channel output of the 8 MIPI channels."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_mipi_channel_sel0(&mut self) -> U0VinCfgMipiChannelSel0W<IspSyscfg9Spec> {
        U0VinCfgMipiChannelSel0W::new(self, 8)
    }
    #[doc = "Bit 12 - Set this bit to 1 to add 10 bits to bit 2, so 1 pixel equals 12 bits."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_mipi_header_en0(&mut self) -> U0VinCfgMipiHeaderEn0W<IspSyscfg9Spec> {
        U0VinCfgMipiHeaderEn0W::new(self, 12)
    }
    #[doc = "Bits 13:16 - VIN AXI to ISP MIPI port, 12-bit data configuration - 00: axi_data\\[11:0\\], 01: axi_data\\[9:0\\], 10: axi_data\\[7:0\\], 11: axi_data\\[5:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_pix_num(&mut self) -> U0VinCfgPixNumW<IspSyscfg9Spec> {
        U0VinCfgPixNumW::new(self, 13)
    }
}
#[doc = "ISP SYSCFG 9: isp_sysconsaif_syscfg_36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg9Spec;
impl crate::RegisterSpec for IspSyscfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg9::R`](R) reader structure"]
impl crate::Readable for IspSyscfg9Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg9::W`](W) writer structure"]
impl crate::Writable for IspSyscfg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg9 to value 0"]
impl crate::Resettable for IspSyscfg9Spec {
    const RESET_VALUE: u32 = 0;
}
