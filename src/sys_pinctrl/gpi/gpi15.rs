#[doc = "Register `gpi15` reader"]
pub type R = crate::R<Gpi15Spec>;
#[doc = "Register `gpi15` writer"]
pub type W = crate::W<Gpi15Spec>;
#[doc = "Field `i2c_data_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData2R = crate::FieldReader;
#[doc = "Field `i2c_data_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_cts_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartCts2R = crate::FieldReader;
#[doc = "Field `uart_cts_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartCts2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_sin_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin2R = crate::FieldReader;
#[doc = "Field `uart_sin_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_clkin_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin2R = crate::FieldReader;
#[doc = "Field `spi_clkin_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_2(&self) -> I2cData2R {
        I2cData2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_cts_2(&self) -> UartCts2R {
        UartCts2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_sin_2(&self) -> UartSin2R {
        UartSin2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_clkin_2(&self) -> SpiClkin2R {
        SpiClkin2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_2(&mut self) -> I2cData2W<Gpi15Spec> {
        I2cData2W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_cts_2(&mut self) -> UartCts2W<Gpi15Spec> {
        UartCts2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_sin_2(&mut self) -> UartSin2W<Gpi15Spec> {
        UartSin2W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkin_2(&mut self) -> SpiClkin2W<Gpi15Spec> {
        SpiClkin2W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_2, uart_cts_2, uart_sin_2, spi_clkin_2\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi15Spec;
impl crate::RegisterSpec for Gpi15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi15::R`](R) reader structure"]
impl crate::Readable for Gpi15Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi15::W`](W) writer structure"]
impl crate::Writable for Gpi15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi15 to value 0x002a_2d00"]
impl crate::Resettable for Gpi15Spec {
    const RESET_VALUE: u32 = 0x002a_2d00;
}
