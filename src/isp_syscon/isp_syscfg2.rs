#[doc = "Register `isp_syscfg2` reader"]
pub type R = crate::R<IspSyscfg2Spec>;
#[doc = "Register `isp_syscfg2` writer"]
pub type W = crate::W<IspSyscfg2Spec>;
#[doc = "Field `u0_vin_cfg_axird_intr_clean` reader - Use this bit to clean the AXI output interrupt. Write 1 then write 0 to execute the cleaning."]
pub type U0VinCfgAxirdIntrCleanR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_axird_intr_clean` writer - Use this bit to clean the AXI output interrupt. Write 1 then write 0 to execute the cleaning."]
pub type U0VinCfgAxirdIntrCleanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_axird_intr_mask` reader - Use this bit to mask the AXI output interrupt."]
pub type U0VinCfgAxirdIntrMaskR = crate::BitReader;
#[doc = "Field `u0_vin_cfg_axird_intr_mask` writer - Use this bit to mask the AXI output interrupt."]
pub type U0VinCfgAxirdIntrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_vin_cfg_axird_line_cnt_end` reader - This bit represents the valid end pixel of the AXI input test image line."]
pub type U0VinCfgAxirdLineCntEndR = crate::FieldReader<u16>;
#[doc = "Field `u0_vin_cfg_axird_line_cnt_end` writer - This bit represents the valid end pixel of the AXI input test image line."]
pub type U0VinCfgAxirdLineCntEndW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `u0_vin_cfg_axird_line_cnt_start` reader - This bit represents the valid start pixel of the AXI input test image line."]
pub type U0VinCfgAxirdLineCntStartR = crate::FieldReader<u16>;
#[doc = "Field `u0_vin_cfg_axird_line_cnt_start` writer - This bit represents the valid start pixel of the AXI input test image line."]
pub type U0VinCfgAxirdLineCntStartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Use this bit to clean the AXI output interrupt. Write 1 then write 0 to execute the cleaning."]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_intr_clean(&self) -> U0VinCfgAxirdIntrCleanR {
        U0VinCfgAxirdIntrCleanR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use this bit to mask the AXI output interrupt."]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_intr_mask(&self) -> U0VinCfgAxirdIntrMaskR {
        U0VinCfgAxirdIntrMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:13 - This bit represents the valid end pixel of the AXI input test image line."]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_line_cnt_end(&self) -> U0VinCfgAxirdLineCntEndR {
        U0VinCfgAxirdLineCntEndR::new(((self.bits >> 2) & 0x0fff) as u16)
    }
    #[doc = "Bits 14:25 - This bit represents the valid start pixel of the AXI input test image line."]
    #[inline(always)]
    pub fn u0_vin_cfg_axird_line_cnt_start(&self) -> U0VinCfgAxirdLineCntStartR {
        U0VinCfgAxirdLineCntStartR::new(((self.bits >> 14) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Use this bit to clean the AXI output interrupt. Write 1 then write 0 to execute the cleaning."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_intr_clean(&mut self) -> U0VinCfgAxirdIntrCleanW<IspSyscfg2Spec> {
        U0VinCfgAxirdIntrCleanW::new(self, 0)
    }
    #[doc = "Bit 1 - Use this bit to mask the AXI output interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_intr_mask(&mut self) -> U0VinCfgAxirdIntrMaskW<IspSyscfg2Spec> {
        U0VinCfgAxirdIntrMaskW::new(self, 1)
    }
    #[doc = "Bits 2:13 - This bit represents the valid end pixel of the AXI input test image line."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_line_cnt_end(&mut self) -> U0VinCfgAxirdLineCntEndW<IspSyscfg2Spec> {
        U0VinCfgAxirdLineCntEndW::new(self, 2)
    }
    #[doc = "Bits 14:25 - This bit represents the valid start pixel of the AXI input test image line."]
    #[inline(always)]
    #[must_use]
    pub fn u0_vin_cfg_axird_line_cnt_start(
        &mut self,
    ) -> U0VinCfgAxirdLineCntStartW<IspSyscfg2Spec> {
        U0VinCfgAxirdLineCntStartW::new(self, 14)
    }
}
#[doc = "ISP SYSCFG 2: isp_sysconsaif_syscfg_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IspSyscfg2Spec;
impl crate::RegisterSpec for IspSyscfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isp_syscfg2::R`](R) reader structure"]
impl crate::Readable for IspSyscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`isp_syscfg2::W`](W) writer structure"]
impl crate::Writable for IspSyscfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets isp_syscfg2 to value 0"]
impl crate::Resettable for IspSyscfg2Spec {
    const RESET_VALUE: u32 = 0;
}
