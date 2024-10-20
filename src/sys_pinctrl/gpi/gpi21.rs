#[doc = "Register `gpi21` reader"]
pub type R = crate::R<Gpi21Spec>;
#[doc = "Register `gpi21` writer"]
pub type W = crate::W<Gpi21Spec>;
#[doc = "Field `spi_fssin_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin5R = crate::FieldReader;
#[doc = "Field `spi_fssin_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_rxd_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd5R = crate::FieldReader;
#[doc = "Field `spi_rxd_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_clk_6` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk6R = crate::FieldReader;
#[doc = "Field `i2c_clk_6` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_data_6` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData6R = crate::FieldReader;
#[doc = "Field `i2c_data_6` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_fssin_5(&self) -> SpiFssin5R {
        SpiFssin5R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_rxd_5(&self) -> SpiRxd5R {
        SpiRxd5R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_clk_6(&self) -> I2cClk6R {
        I2cClk6R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_6(&self) -> I2cData6R {
        I2cData6R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fssin_5(&mut self) -> SpiFssin5W<Gpi21Spec> {
        SpiFssin5W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rxd_5(&mut self) -> SpiRxd5W<Gpi21Spec> {
        SpiRxd5W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_6(&mut self) -> I2cClk6W<Gpi21Spec> {
        I2cClk6W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_6(&mut self) -> I2cData6W<Gpi21Spec> {
        I2cData6W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_fssin_5, spi_rxd_5, i2c_clk_6, i2c_data_6\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi21Spec;
impl crate::RegisterSpec for Gpi21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi21::R`](R) reader structure"]
impl crate::Readable for Gpi21Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi21::W`](W) writer structure"]
impl crate::Writable for Gpi21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi21 to value 0"]
impl crate::Resettable for Gpi21Spec {
    const RESET_VALUE: u32 = 0;
}
