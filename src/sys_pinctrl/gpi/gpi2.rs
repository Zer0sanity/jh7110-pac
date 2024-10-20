#[doc = "Register `gpi2` reader"]
pub type R = crate::R<Gpi2Spec>;
#[doc = "Register `gpi2` writer"]
pub type W = crate::W<Gpi2Spec>;
#[doc = "Field `hdmi_hpd` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HdmiHpdR = crate::FieldReader;
#[doc = "Field `hdmi_hpd` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type HdmiHpdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_clk_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk0R = crate::FieldReader;
#[doc = "Field `i2c_clk_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_data_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData0R = crate::FieldReader;
#[doc = "Field `i2c_data_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_detect_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioDetect0R = crate::FieldReader;
#[doc = "Field `sdio_detect_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioDetect0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hdmi_hpd(&self) -> HdmiHpdR {
        HdmiHpdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_clk_0(&self) -> I2cClk0R {
        I2cClk0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_0(&self) -> I2cData0R {
        I2cData0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_detect_0(&self) -> SdioDetect0R {
        SdioDetect0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_hpd(&mut self) -> HdmiHpdW<Gpi2Spec> {
        HdmiHpdW::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_0(&mut self) -> I2cClk0W<Gpi2Spec> {
        I2cClk0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_0(&mut self) -> I2cData0W<Gpi2Spec> {
        I2cData0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_detect_0(&mut self) -> SdioDetect0W<Gpi2Spec> {
        SdioDetect0W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[hdmi_hpd, i2c_clk_0, i2c_data_0, sdio_detect_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi2Spec;
impl crate::RegisterSpec for Gpi2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi2::R`](R) reader structure"]
impl crate::Readable for Gpi2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi2::W`](W) writer structure"]
impl crate::Writable for Gpi2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi2 to value 0x0027_2600"]
impl crate::Resettable for Gpi2Spec {
    const RESET_VALUE: u32 = 0x0027_2600;
}
