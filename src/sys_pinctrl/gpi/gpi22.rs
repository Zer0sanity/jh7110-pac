#[doc = "Register `gpi22` reader"]
pub type R = crate::R<Gpi22Spec>;
#[doc = "Register `gpi22` writer"]
pub type W = crate::W<Gpi22Spec>;
#[doc = "Field `spi_clkin_6` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin6R = crate::FieldReader;
#[doc = "Field `spi_clkin_6` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiClkin6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_fssin_6` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin6R = crate::FieldReader;
#[doc = "Field `spi_fssin_6` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiFssin6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spi_rxd_6` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd6R = crate::FieldReader;
#[doc = "Field `spi_rxd_6` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_clkin_6(&self) -> SpiClkin6R {
        SpiClkin6R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_fssin_6(&self) -> SpiFssin6R {
        SpiFssin6R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_rxd_6(&self) -> SpiRxd6R {
        SpiRxd6R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_clkin_6(&mut self) -> SpiClkin6W<Gpi22Spec> {
        SpiClkin6W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fssin_6(&mut self) -> SpiFssin6W<Gpi22Spec> {
        SpiFssin6W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rxd_6(&mut self) -> SpiRxd6W<Gpi22Spec> {
        SpiRxd6W::new(self, 16)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_6, spi_fssin_6, spi_rxd_6\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi22Spec;
impl crate::RegisterSpec for Gpi22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi22::R`](R) reader structure"]
impl crate::Readable for Gpi22Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi22::W`](W) writer structure"]
impl crate::Writable for Gpi22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi22 to value 0"]
impl crate::Resettable for Gpi22Spec {
    const RESET_VALUE: u32 = 0;
}
