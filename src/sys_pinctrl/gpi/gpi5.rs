#[doc = "Register `gpi5` reader"]
pub type R = crate::R<Gpi5Spec>;
#[doc = "Register `gpi5` writer"]
pub type W = crate::W<Gpi5Spec>;
#[doc = "Field `jtag_tms` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JtagTmsR = crate::FieldReader;
#[doc = "Field `jtag_tms` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JtagTmsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `pdm_dmic_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PdmDmic0R = crate::FieldReader;
#[doc = "Field `pdm_dmic_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PdmDmic0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `pdm_dmic_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PdmDmic1R = crate::FieldReader;
#[doc = "Field `pdm_dmic_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PdmDmic1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `audio_i2srx_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AudioI2srx0R = crate::FieldReader;
#[doc = "Field `audio_i2srx_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AudioI2srx0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn jtag_tms(&self) -> JtagTmsR {
        JtagTmsR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn pdm_dmic_0(&self) -> PdmDmic0R {
        PdmDmic0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn pdm_dmic_1(&self) -> PdmDmic1R {
        PdmDmic1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn audio_i2srx_0(&self) -> AudioI2srx0R {
        AudioI2srx0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_tms(&mut self) -> JtagTmsW<Gpi5Spec> {
        JtagTmsW::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn pdm_dmic_0(&mut self) -> PdmDmic0W<Gpi5Spec> {
        PdmDmic0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn pdm_dmic_1(&mut self) -> PdmDmic1W<Gpi5Spec> {
        PdmDmic1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn audio_i2srx_0(&mut self) -> AudioI2srx0W<Gpi5Spec> {
        AudioI2srx0W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[jtag_tms, pdm_dmic_0, pdm_dmic_1, audio_i2srx_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi5Spec;
impl crate::RegisterSpec for Gpi5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi5::R`](R) reader structure"]
impl crate::Readable for Gpi5Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi5::W`](W) writer structure"]
impl crate::Writable for Gpi5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi5 to value 0x06"]
impl crate::Resettable for Gpi5Spec {
    const RESET_VALUE: u32 = 0x06;
}
