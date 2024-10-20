#[doc = "Register `gpi1` reader"]
pub type R = crate::R<Gpi1Spec>;
#[doc = "Register `gpi1` writer"]
pub type W = crate::W<Gpi1Spec>;
#[doc = "Field `jtag_trstn` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JtagTrstnR = crate::FieldReader;
#[doc = "Field `jtag_trstn` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JtagTrstnW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hdmi_cec_sda` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HdmiCecSdaR = crate::FieldReader;
#[doc = "Field `hdmi_cec_sda` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HdmiCecSdaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hdmi_ddc_scl` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HdmiDdcSclR = crate::FieldReader;
#[doc = "Field `hdmi_ddc_scl` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HdmiDdcSclW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hdmi_ddc_sda` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HdmiDdcSdaR = crate::FieldReader;
#[doc = "Field `hdmi_ddc_sda` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HdmiDdcSdaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn jtag_trstn(&self) -> JtagTrstnR {
        JtagTrstnR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hdmi_cec_sda(&self) -> HdmiCecSdaR {
        HdmiCecSdaR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hdmi_ddc_scl(&self) -> HdmiDdcSclR {
        HdmiDdcSclR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hdmi_ddc_sda(&self) -> HdmiDdcSdaR {
        HdmiDdcSdaR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_trstn(&mut self) -> JtagTrstnW<Gpi1Spec> {
        JtagTrstnW::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec_sda(&mut self) -> HdmiCecSdaW<Gpi1Spec> {
        HdmiCecSdaW::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_ddc_scl(&mut self) -> HdmiDdcSclW<Gpi1Spec> {
        HdmiDdcSclW::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_ddc_sda(&mut self) -> HdmiDdcSdaW<Gpi1Spec> {
        HdmiDdcSdaW::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[jtag_trstn, hdmi_cec_sda, hdmi_ddc_scl, hdmi_ddc_sda\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi1Spec;
impl crate::RegisterSpec for Gpi1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi1::R`](R) reader structure"]
impl crate::Readable for Gpi1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi1::W`](W) writer structure"]
impl crate::Writable for Gpi1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi1 to value 0x02"]
impl crate::Resettable for Gpi1Spec {
    const RESET_VALUE: u32 = 0x02;
}
