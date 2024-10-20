#[doc = "Register `gpi10` reader"]
pub type R = crate::R<Gpi10Spec>;
#[doc = "Register `gpi10` writer"]
pub type W = crate::W<Gpi10Spec>;
#[doc = "Field `i2c_data_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData1R = crate::FieldReader;
#[doc = "Field `i2c_data_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cData1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_detect_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioDetect1R = crate::FieldReader;
#[doc = "Field `sdio_detect_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioDetect1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_int_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioInt1R = crate::FieldReader;
#[doc = "Field `sdio_int_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioInt1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_write_prt_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioWritePrt1R = crate::FieldReader;
#[doc = "Field `sdio_write_prt_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioWritePrt1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_data_1(&self) -> I2cData1R {
        I2cData1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_detect_1(&self) -> SdioDetect1R {
        SdioDetect1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_int_1(&self) -> SdioInt1R {
        SdioInt1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_write_prt_1(&self) -> SdioWritePrt1R {
        SdioWritePrt1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_data_1(&mut self) -> I2cData1W<Gpi10Spec> {
        I2cData1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_detect_1(&mut self) -> SdioDetect1W<Gpi10Spec> {
        SdioDetect1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_1(&mut self) -> SdioInt1W<Gpi10Spec> {
        SdioInt1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_write_prt_1(&mut self) -> SdioWritePrt1W<Gpi10Spec> {
        SdioWritePrt1W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_1, sdio_detect_1, sdio_int_1, sdio_write_prt_1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi10Spec;
impl crate::RegisterSpec for Gpi10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi10::R`](R) reader structure"]
impl crate::Readable for Gpi10Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi10::W`](W) writer structure"]
impl crate::Writable for Gpi10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi10 to value 0"]
impl crate::Resettable for Gpi10Spec {
    const RESET_VALUE: u32 = 0;
}
