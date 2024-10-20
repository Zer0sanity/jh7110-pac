#[doc = "Register `gpi3` reader"]
pub type R = crate::R<Gpi3Spec>;
#[doc = "Register `gpi3` writer"]
pub type W = crate::W<Gpi3Spec>;
#[doc = "Field `sdio_int_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioInt0R = crate::FieldReader;
#[doc = "Field `sdio_int_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioInt0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_write_prt_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioWritePrt0R = crate::FieldReader;
#[doc = "Field `sdio_write_prt_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioWritePrt0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `uart_sin_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin0R = crate::FieldReader;
#[doc = "Field `uart_sin_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type UartSin0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hifi4_jtck_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Hifi4Jtck0R = crate::FieldReader;
#[doc = "Field `hifi4_jtck_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Hifi4Jtck0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_int_0(&self) -> SdioInt0R {
        SdioInt0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_write_prt_0(&self) -> SdioWritePrt0R {
        SdioWritePrt0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn uart_sin_0(&self) -> UartSin0R {
        UartSin0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hifi4_jtck_0(&self) -> Hifi4Jtck0R {
        Hifi4Jtck0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_0(&mut self) -> SdioInt0W<Gpi3Spec> {
        SdioInt0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_write_prt_0(&mut self) -> SdioWritePrt0W<Gpi3Spec> {
        SdioWritePrt0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn uart_sin_0(&mut self) -> UartSin0W<Gpi3Spec> {
        UartSin0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_jtck_0(&mut self) -> Hifi4Jtck0W<Gpi3Spec> {
        Hifi4Jtck0W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_int_0, sdio_write_prt_0, uart_sin_0, hifi4_jtck_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi3Spec;
impl crate::RegisterSpec for Gpi3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi3::R`](R) reader structure"]
impl crate::Readable for Gpi3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi3::W`](W) writer structure"]
impl crate::Writable for Gpi3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi3 to value 0x0b08_0000"]
impl crate::Resettable for Gpi3Spec {
    const RESET_VALUE: u32 = 0x0b08_0000;
}
