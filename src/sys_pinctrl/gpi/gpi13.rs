#[doc = "Register `gpi13` reader"]
pub type R = crate::R<Gpi13Spec>;
#[doc = "Register `gpi13` writer"]
pub type W = crate::W<Gpi13Spec>;
#[doc = "Field `sdio_cdata_7` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata7R = crate::FieldReader;
#[doc = "Field `sdio_cdata_7` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_data_strobe` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioDataStrobeR = crate::FieldReader;
#[doc = "Field `sdio_data_strobe` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioDataStrobeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_cts_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartCts1R = crate::FieldReader;
#[doc = "Field `uart_cts_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartCts1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_sin_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin1R = crate::FieldReader;
#[doc = "Field `uart_sin_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_7(&self) -> SdioCdata7R {
        SdioCdata7R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_data_strobe(&self) -> SdioDataStrobeR {
        SdioDataStrobeR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_cts_1(&self) -> UartCts1R {
        UartCts1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_sin_1(&self) -> UartSin1R {
        UartSin1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_7(&mut self) -> SdioCdata7W<Gpi13Spec> {
        SdioCdata7W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_data_strobe(&mut self) -> SdioDataStrobeW<Gpi13Spec> {
        SdioDataStrobeW::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_cts_1(&mut self) -> UartCts1W<Gpi13Spec> {
        UartCts1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_sin_1(&mut self) -> UartSin1W<Gpi13Spec> {
        UartSin1W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_cdata_7, sdio_data_strobe, uart_cts_1, uart_sin_1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi13Spec;
impl crate::RegisterSpec for Gpi13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi13::R`](R) reader structure"]
impl crate::Readable for Gpi13Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi13::W`](W) writer structure"]
impl crate::Writable for Gpi13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi13 to value 0"]
impl crate::Resettable for Gpi13Spec {
    const RESET_VALUE: u32 = 0;
}
