#[doc = "Register `gpi19` reader"]
pub type R = crate::R<Gpi19Spec>;
#[doc = "Register `gpi19` writer"]
pub type W = crate::W<Gpi19Spec>;
#[doc = "Field `spi_clkin_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin4R = crate::FieldReader;
#[doc = "Field `spi_clkin_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_fssin_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin4R = crate::FieldReader;
#[doc = "Field `spi_fssin_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_rxd_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd4R = crate::FieldReader;
#[doc = "Field `spi_rxd_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_clk_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk5R = crate::FieldReader;
#[doc = "Field `i2c_clk_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_clkin_4(&self) -> SpiClkin4R {
        SpiClkin4R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_fssin_4(&self) -> SpiFssin4R {
        SpiFssin4R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_rxd_4(&self) -> SpiRxd4R {
        SpiRxd4R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_clk_5(&self) -> I2cClk5R {
        I2cClk5R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkin_4(&mut self) -> SpiClkin4W<Gpi19Spec> {
        SpiClkin4W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fssin_4(&mut self) -> SpiFssin4W<Gpi19Spec> {
        SpiFssin4W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rxd_4(&mut self) -> SpiRxd4W<Gpi19Spec> {
        SpiRxd4W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_5(&mut self) -> I2cClk5W<Gpi19Spec> {
        I2cClk5W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_4, spi_fssin_4, spi_rxd_4, i2c_clk_5\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi19Spec;
impl crate::RegisterSpec for Gpi19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi19::R`](R) reader structure"]
impl crate::Readable for Gpi19Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi19::W`](W) writer structure"]
impl crate::Writable for Gpi19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi19 to value 0x0040_3e3f"]
impl crate::Resettable for Gpi19Spec {
    const RESET_VALUE: u32 = 0x0040_3e3f;
}
