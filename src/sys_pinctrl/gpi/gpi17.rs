#[doc = "Register `gpi17` reader"]
pub type R = crate::R<Gpi17Spec>;
#[doc = "Register `gpi17` writer"]
pub type W = crate::W<Gpi17Spec>;
#[doc = "Field `uart_sin_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin3R = crate::FieldReader;
#[doc = "Field `uart_sin_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_clkin_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin3R = crate::FieldReader;
#[doc = "Field `spi_clkin_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_fssin_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin3R = crate::FieldReader;
#[doc = "Field `spi_fssin_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_rxd_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd3R = crate::FieldReader;
#[doc = "Field `spi_rxd_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_sin_3(&self) -> UartSin3R {
        UartSin3R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_clkin_3(&self) -> SpiClkin3R {
        SpiClkin3R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_fssin_3(&self) -> SpiFssin3R {
        SpiFssin3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_rxd_3(&self) -> SpiRxd3R {
        SpiRxd3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_sin_3(&mut self) -> UartSin3W<Gpi17Spec> {
        UartSin3W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkin_3(&mut self) -> SpiClkin3W<Gpi17Spec> {
        SpiClkin3W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fssin_3(&mut self) -> SpiFssin3W<Gpi17Spec> {
        SpiFssin3W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rxd_3(&mut self) -> SpiRxd3W<Gpi17Spec> {
        SpiRxd3W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[uart_sin_3, spi_clkin_3, spi_fssin_3, spi_rxd_3\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi17Spec;
impl crate::RegisterSpec for Gpi17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi17::R`](R) reader structure"]
impl crate::Readable for Gpi17Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi17::W`](W) writer structure"]
impl crate::Writable for Gpi17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi17 to value 0x3c3a_3b15"]
impl crate::Resettable for Gpi17Spec {
    const RESET_VALUE: u32 = 0x3c3a_3b15;
}
