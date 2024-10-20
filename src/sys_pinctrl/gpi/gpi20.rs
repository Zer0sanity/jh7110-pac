#[doc = "Register `gpi20` reader"]
pub type R = crate::R<Gpi20Spec>;
#[doc = "Register `gpi20` writer"]
pub type W = crate::W<Gpi20Spec>;
#[doc = "Field `i2c_data_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData5R = crate::FieldReader;
#[doc = "Field `i2c_data_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_cts_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartCts5R = crate::FieldReader;
#[doc = "Field `uart_cts_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartCts5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_sin_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin5R = crate::FieldReader;
#[doc = "Field `uart_sin_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_clkin_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin5R = crate::FieldReader;
#[doc = "Field `spi_clkin_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_5(&self) -> I2cData5R {
        I2cData5R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_cts_5(&self) -> UartCts5R {
        UartCts5R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_sin_5(&self) -> UartSin5R {
        UartSin5R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_clkin_5(&self) -> SpiClkin5R {
        SpiClkin5R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_5(&mut self) -> I2cData5W<Gpi20Spec> {
        I2cData5W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_cts_5(&mut self) -> UartCts5W<Gpi20Spec> {
        UartCts5W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_sin_5(&mut self) -> UartSin5W<Gpi20Spec> {
        UartSin5W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkin_5(&mut self) -> SpiClkin5W<Gpi20Spec> {
        SpiClkin5W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_5, uart_cts_5, uart_sin_5, spi_clkin_5\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi20Spec;
impl crate::RegisterSpec for Gpi20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi20::R`](R) reader structure"]
impl crate::Readable for Gpi20Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi20::W`](W) writer structure"]
impl crate::Writable for Gpi20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi20 to value 0"]
impl crate::Resettable for Gpi20Spec {
    const RESET_VALUE: u32 = 0;
}
