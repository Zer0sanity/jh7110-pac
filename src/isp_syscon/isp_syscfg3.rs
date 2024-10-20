#[doc = "Register `isp_syscfg3` reader"]
pub type R = crate::R<IspSyscfg3Spec>;
#[doc = "Register `isp_syscfg3` writer"]
pub type W = crate::W<IspSyscfg3Spec>;
#[doc = "Field `u0_vin_cfg_axird_pix_cnt_end` reader - This bit represents the valid end pixel of the AXI input test image line."]
pub type U0VinCfgAxirdPixCntEndR = crate::FieldReader<u16>;
#[doc = "Field `u0_vin_cfg_axird_pix_cnt_end` writer - This bit represents the valid end pixel of the AXI input test image line."]
pub type U0VinCfgAxirdPixCntEndW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `u0_vin_cfg_axird_pix_cnt_start` reader - This bit represents the valid start pixel of the AXI input test image line."]
pub type U0VinCfgAxirdPixCntStartR = crate::FieldReader<u16>;
#[doc = "Field `u0_vin_cfg_axird_pix_cnt_start` writer - This bit represents the valid start pixel of the AXI input test image line."]
pub type U0VinCfgAxirdPixCntStartW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `u0_vin_cfg_axird_pix_ct` reader - 00: 1 64-bit equals to 2 pixels, 01: 1 64-bit equals to 4 pixels, 10: 1 64-bit equals to 8 pixels"]
pub type U0VinCfgAxirdPixCtR = crate::FieldReader;
#[doc = "Field `u0_vin_cfg_axird_pix_ct` writer - 00: 1 64-bit equals to 2 pixels, 01: 1 64-bit equals to 4 pixels, 10: 1 64-bit equals to 8 pixels"]
pub type U0VinCfgAxirdPixCtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:12 - This bit represents the valid end pixel of the AXI input test image line."]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_pix_cnt_end(&self) -> U0VinCfgAxirdPixCntEndR {
        U0VinCfgAxirdPixCntEndR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:25 - This bit represents the valid start pixel of the AXI input test image line."]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_pix_cnt_start(&self) -> U0VinCfgAxirdPixCntStartR {
        U0VinCfgAxirdPixCntStartR::new(((self.bits >> 13) & 0x1fff) as u16)
    }
    #[doc = "Bits 26:27 - 00: 1 64-bit equals to 2 pixels, 01: 1 64-bit equals to 4 pixels, 10: 1 64-bit equals to 8 pixels"]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_pix_ct(&self) -> U0VinCfgAxirdPixCtR {
        U0VinCfgAxirdPixCtR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - This bit represents the valid end pixel of the AXI input test image line."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_pix_cnt_end(&mut self) -> U0VinCfgAxirdPixCntEndW<IspSyscfg3Spec> {
        U0VinCfgAxirdPixCntEndW::new(self, 0)
    }
    #[doc = "Bits 13:25 - This bit represents the valid start pixel of the AXI input test image line."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_pix_cnt_start(&mut self) -> U0VinCfgAxirdPixCntStartW<IspSyscfg3Spec> {
        U0VinCfgAxirdPixCntStartW::new(self, 13)
    }
    #[doc = "Bits 26:27 - 00: 1 64-bit equals to 2 pixels, 01: 1 64-bit equals to 4 pixels, 10: 1 64-bit equals to 8 pixels"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_pix_ct(&mut self) -> U0VinCfgAxirdPixCtW<IspSyscfg3Spec> {
        U0VinCfgAxirdPixCtW::new(self, 26)
    }
}
#[doc = "ISP SYSCFG 3: isp_sysconsaif_syscfg_12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg3Spec;
impl crate::RegisterSpec for IspSyscfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg3::R`](R) reader structure"]
impl crate::Readable for IspSyscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg3::W`](W) writer structure"]
impl crate::Writable for IspSyscfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg3 to value 0"]
impl crate::Resettable for IspSyscfg3Spec {
    const RESET_VALUE: u32 = 0;
}
