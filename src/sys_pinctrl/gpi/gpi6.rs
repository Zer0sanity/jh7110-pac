#[doc = "Register `gpi6` reader"]
pub type R = crate::R<Gpi6Spec>;
#[doc = "Register `gpi6` writer"]
pub type W = crate::W<Gpi6Spec>;
#[doc = "Field `audio_i2srx_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AudioI2srx1R = crate::FieldReader;
#[doc = "Field `audio_i2srx_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AudioI2srx1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `audio_i2srx_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AudioI2srx2R = crate::FieldReader;
#[doc = "Field `audio_i2srx_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type AudioI2srx2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_clkin_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin0R = crate::FieldReader;
#[doc = "Field `spi_clkin_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_fssin_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin0R = crate::FieldReader;
#[doc = "Field `spi_fssin_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn audio_i2srx_1(&self) -> AudioI2srx1R {
        AudioI2srx1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn audio_i2srx_2(&self) -> AudioI2srx2R {
        AudioI2srx2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_clkin_0(&self) -> SpiClkin0R {
        SpiClkin0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_fssin_0(&self) -> SpiFssin0R {
        SpiFssin0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn audio_i2srx_1(&mut self) -> AudioI2srx1W<Gpi6Spec> {
        AudioI2srx1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn audio_i2srx_2(&mut self) -> AudioI2srx2W<Gpi6Spec> {
        AudioI2srx2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkin_0(&mut self) -> SpiClkin0W<Gpi6Spec> {
        SpiClkin0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fssin_0(&mut self) -> SpiFssin0W<Gpi6Spec> {
        SpiFssin0W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[audio_i2srx_1, audio_i2srx_2, spi_clkin_0, spi_fssin_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi6Spec;
impl crate::RegisterSpec for Gpi6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi6::R`](R) reader structure"]
impl crate::Readable for Gpi6Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi6::W`](W) writer structure"]
impl crate::Writable for Gpi6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi6 to value 0x3233_0000"]
impl crate::Resettable for Gpi6Spec {
    const RESET_VALUE: u32 = 0x3233_0000;
}
