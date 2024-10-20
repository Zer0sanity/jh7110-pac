#[doc = "Register `isp_syscfg7` reader"]
pub type R = crate::R<IspSyscfg7Spec>;
#[doc = "Register `isp_syscfg7` writer"]
pub type W = crate::W<IspSyscfg7Spec>;
#[doc = "Field `u0_vin_cfg_axiwr0_intr_clean` reader - Use this bit to clean the AXI output interrupt. Write 1 then write 0 to execute the cleaning."]
pub type U0VinCfgAxiwr0IntrCleanR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_axiwr0_intr_clean` writer - Use this bit to clean the AXI output interrupt. Write 1 then write 0 to execute the cleaning."]
pub type U0VinCfgAxiwr0IntrCleanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_axiwr0_intr_mask` reader - Use this bit to mask the AXI output interrupt."]
pub type U0VinCfgAxiwr0IntrMaskR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_axiwr0_intr_mask` writer - Use this bit to mask the AXI output interrupt."]
pub type U0VinCfgAxiwr0IntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_axiwr0_pix_cnt_end` reader - This bit represents the valid end pixel of the AXI input test image line."]
pub type U0VinCfgAxiwr0PixCntEndR = crate::FieldReader<u16>;
#[doc = "Field `u0_vin_cfg_axiwr0_pix_cnt_end` writer - This bit represents the valid end pixel of the AXI input test image line."]
pub type U0VinCfgAxiwr0PixCntEndW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `u0_vin_cfg_axiwr0_pix_ct` reader - 00: 1 64-bit equals to 2 pixels, 01: 1 64-bit equals to 4 pixels, 10: 1 64-bit equals to 8 pixels"]
pub type U0VinCfgAxiwr0PixCtR = crate::FieldReader;
#[doc = "Field `u0_vin_cfg_axiwr0_pix_ct` writer - 00: 1 64-bit equals to 2 pixels, 01: 1 64-bit equals to 4 pixels, 10: 1 64-bit equals to 8 pixels"]
pub type U0VinCfgAxiwr0PixCtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_vin_cfg_axiwr0_pixel_high_bit_sel` reader - When you configure the above bit as '10' - 1 64-bit equals to 8 pixels, the 8 pixels will use some of the RAW data - 00: 1 64-bit pix_data\\[7:0\\], 01: 1 pix_data\\[9:2\\], 10: pix_data\\[11:4\\], 11: pix_data\\[13:6\\]"]
pub type U0VinCfgAxiwr0PixelHighBitSelR = crate::FieldReader;
#[doc = "Field `u0_vin_cfg_axiwr0_pixel_high_bit_sel` writer - When you configure the above bit as '10' - 1 64-bit equals to 8 pixels, the 8 pixels will use some of the RAW data - 00: 1 64-bit pix_data\\[7:0\\], 01: 1 pix_data\\[9:2\\], 10: pix_data\\[11:4\\], 11: pix_data\\[13:6\\]"]
pub type U0VinCfgAxiwr0PixelHighBitSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Use this bit to clean the AXI output interrupt. Write 1 then write 0 to execute the cleaning."]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_intr_clean(&self) -> U0VinCfgAxiwr0IntrCleanR {
        U0VinCfgAxiwr0IntrCleanR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use this bit to mask the AXI output interrupt."]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_intr_mask(&self) -> U0VinCfgAxiwr0IntrMaskR {
        U0VinCfgAxiwr0IntrMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:12 - This bit represents the valid end pixel of the AXI input test image line."]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_pix_cnt_end(&self) -> U0VinCfgAxiwr0PixCntEndR {
        U0VinCfgAxiwr0PixCntEndR::new(((self.bits >> 2) & 0x07ff) as u16)
    }
    #[doc = "Bits 13:14 - 00: 1 64-bit equals to 2 pixels, 01: 1 64-bit equals to 4 pixels, 10: 1 64-bit equals to 8 pixels"]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_pix_ct(&self) -> U0VinCfgAxiwr0PixCtR {
        U0VinCfgAxiwr0PixCtR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - When you configure the above bit as '10' - 1 64-bit equals to 8 pixels, the 8 pixels will use some of the RAW data - 00: 1 64-bit pix_data\\[7:0\\], 01: 1 pix_data\\[9:2\\], 10: pix_data\\[11:4\\], 11: pix_data\\[13:6\\]"]
    #[inline(always)]
    pub fn u0_vin_cfg_axiwr0_pixel_high_bit_sel(&self) -> U0VinCfgAxiwr0PixelHighBitSelR {
        U0VinCfgAxiwr0PixelHighBitSelR::new(((self.bits >> 15) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Use this bit to clean the AXI output interrupt. Write 1 then write 0 to execute the cleaning."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_intr_clean(&mut self) -> U0VinCfgAxiwr0IntrCleanW<IspSyscfg7Spec> {
        U0VinCfgAxiwr0IntrCleanW::new(self, 0)
    }
    #[doc = "Bit 1 - Use this bit to mask the AXI output interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_intr_mask(&mut self) -> U0VinCfgAxiwr0IntrMaskW<IspSyscfg7Spec> {
        U0VinCfgAxiwr0IntrMaskW::new(self, 1)
    }
    #[doc = "Bits 2:12 - This bit represents the valid end pixel of the AXI input test image line."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_pix_cnt_end(&mut self) -> U0VinCfgAxiwr0PixCntEndW<IspSyscfg7Spec> {
        U0VinCfgAxiwr0PixCntEndW::new(self, 2)
    }
    #[doc = "Bits 13:14 - 00: 1 64-bit equals to 2 pixels, 01: 1 64-bit equals to 4 pixels, 10: 1 64-bit equals to 8 pixels"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_pix_ct(&mut self) -> U0VinCfgAxiwr0PixCtW<IspSyscfg7Spec> {
        U0VinCfgAxiwr0PixCtW::new(self, 13)
    }
    #[doc = "Bits 15:16 - When you configure the above bit as '10' - 1 64-bit equals to 8 pixels, the 8 pixels will use some of the RAW data - 00: 1 64-bit pix_data\\[7:0\\], 01: 1 pix_data\\[9:2\\], 10: pix_data\\[11:4\\], 11: pix_data\\[13:6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axiwr0_pixel_high_bit_sel(
        &mut self,
    ) -> U0VinCfgAxiwr0PixelHighBitSelW<IspSyscfg7Spec> {
        U0VinCfgAxiwr0PixelHighBitSelW::new(self, 15)
    }
}
#[doc = "ISP SYSCFG 7: isp_sysconsaif_syscfg_28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg7Spec;
impl crate::RegisterSpec for IspSyscfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg7::R`](R) reader structure"]
impl crate::Readable for IspSyscfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg7::W`](W) writer structure"]
impl crate::Writable for IspSyscfg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg7 to value 0"]
impl crate::Resettable for IspSyscfg7Spec {
    const RESET_VALUE: u32 = 0;
}
