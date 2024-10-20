#[doc = "Register `gpi16` reader"]
pub type R = crate::R<Gpi16Spec>;
#[doc = "Register `gpi16` writer"]
pub type W = crate::W<Gpi16Spec>;
#[doc = "Field `spi_fssin_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin2R = crate::FieldReader;
#[doc = "Field `spi_fssin_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_rxd_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd2R = crate::FieldReader;
#[doc = "Field `spi_rxd_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_clk_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk3R = crate::FieldReader;
#[doc = "Field `i2c_clk_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_data_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData3R = crate::FieldReader;
#[doc = "Field `i2c_data_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_fssin_2(&self) -> SpiFssin2R {
        SpiFssin2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_rxd_2(&self) -> SpiRxd2R {
        SpiRxd2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_clk_3(&self) -> I2cClk3R {
        I2cClk3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_3(&self) -> I2cData3R {
        I2cData3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fssin_2(&mut self) -> SpiFssin2W<Gpi16Spec> {
        SpiFssin2W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rxd_2(&mut self) -> SpiRxd2W<Gpi16Spec> {
        SpiRxd2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_3(&mut self) -> I2cClk3W<Gpi16Spec> {
        I2cClk3W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_3(&mut self) -> I2cData3W<Gpi16Spec> {
        I2cData3W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_fssin_2, spi_rxd_2, i2c_clk_3, i2c_data_3\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi16Spec;
impl crate::RegisterSpec for Gpi16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi16::R`](R) reader structure"]
impl crate::Readable for Gpi16Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi16::W`](W) writer structure"]
impl crate::Writable for Gpi16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi16 to value 0x2928_0000"]
impl crate::Resettable for Gpi16Spec {
    const RESET_VALUE: u32 = 0x2928_0000;
}
