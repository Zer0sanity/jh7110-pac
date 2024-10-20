#[doc = "Register `gpi18` reader"]
pub type R = crate::R<Gpi18Spec>;
#[doc = "Register `gpi18` writer"]
pub type W = crate::W<Gpi18Spec>;
#[doc = "Field `i2c_clk_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk4R = crate::FieldReader;
#[doc = "Field `i2c_clk_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_data_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData4R = crate::FieldReader;
#[doc = "Field `i2c_data_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_cts_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartCts4R = crate::FieldReader;
#[doc = "Field `uart_cts_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartCts4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_sin_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin4R = crate::FieldReader;
#[doc = "Field `uart_sin_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_clk_4(&self) -> I2cClk4R {
        I2cClk4R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_4(&self) -> I2cData4R {
        I2cData4R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_cts_4(&self) -> UartCts4R {
        UartCts4R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_sin_4(&self) -> UartSin4R {
        UartSin4R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_4(&mut self) -> I2cClk4W<Gpi18Spec> {
        I2cClk4W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_4(&mut self) -> I2cData4W<Gpi18Spec> {
        I2cData4W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_cts_4(&mut self) -> UartCts4W<Gpi18Spec> {
        UartCts4W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_sin_4(&mut self) -> UartSin4W<Gpi18Spec> {
        UartSin4W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_clk_4, i2c_data_4, uart_cts_4, uart_sin_4\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi18Spec;
impl crate::RegisterSpec for Gpi18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi18::R`](R) reader structure"]
impl crate::Readable for Gpi18Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi18::W`](W) writer structure"]
impl crate::Writable for Gpi18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi18 to value 0x2e31_0000"]
impl crate::Resettable for Gpi18Spec {
    const RESET_VALUE: u32 = 0x2e31_0000;
}
