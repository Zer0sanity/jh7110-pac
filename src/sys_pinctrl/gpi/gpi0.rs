#[doc = "Register `gpi0` reader"]
pub type R = crate::R<Gpi0Spec>;
#[doc = "Register `gpi0` writer"]
pub type W = crate::W<Gpi0Spec>;
#[doc = "Field `wave511_uart_rxsin` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Wave511UartRxsinR = crate::FieldReader;
#[doc = "Field `wave511_uart_rxsin` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Wave511UartRxsinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `can_rxd_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type CanRxd0R = crate::FieldReader;
#[doc = "Field `can_rxd_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type CanRxd0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `usb_over_current` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UsbOverCurrentR = crate::FieldReader;
#[doc = "Field `usb_over_current` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UsbOverCurrentW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `spdif_spdi_fi` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpdifSpdiFiR = crate::FieldReader;
#[doc = "Field `spdif_spdi_fi` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpdifSpdiFiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn wave511_uart_rxsin(&self) -> Wave511UartRxsinR {
        Wave511UartRxsinR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn can_rxd_0(&self) -> CanRxd0R {
        CanRxd0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn usb_over_current(&self) -> UsbOverCurrentR {
        UsbOverCurrentR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spdif_spdi_fi(&self) -> SpdifSpdiFiR {
        SpdifSpdiFiR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn wave511_uart_rxsin(&mut self) -> Wave511UartRxsinW<Gpi0Spec> {
        Wave511UartRxsinW::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn can_rxd_0(&mut self) -> CanRxd0W<Gpi0Spec> {
        CanRxd0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn usb_over_current(&mut self) -> UsbOverCurrentW<Gpi0Spec> {
        UsbOverCurrentW::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spdif_spdi_fi(&mut self) -> SpdifSpdiFiW<Gpi0Spec> {
        SpdifSpdiFiW::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[wave511_uart_rxsin, can_rxd_0, usb_over_current, spdif_spdi_fi\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi0Spec;
impl crate::RegisterSpec for Gpi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi0::R`](R) reader structure"]
impl crate::Readable for Gpi0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi0::W`](W) writer structure"]
impl crate::Writable for Gpi0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi0 to value 0"]
impl crate::Resettable for Gpi0Spec {
    const RESET_VALUE: u32 = 0;
}
