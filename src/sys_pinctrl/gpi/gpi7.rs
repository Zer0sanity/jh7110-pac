#[doc = "Register `gpi7` reader"]
pub type R = crate::R<Gpi7Spec>;
#[doc = "Register `gpi7` writer"]
pub type W = crate::W<Gpi7Spec>;
#[doc = "Field `spi_rxd_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd0R = crate::FieldReader;
#[doc = "Field `spi_rxd_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SpiRxd0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `jtag_tck` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JtagTckR = crate::FieldReader;
#[doc = "Field `jtag_tck` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JtagTckW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `mclk` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type MclkR = crate::FieldReader;
#[doc = "Field `mclk` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type MclkW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2srx_bclk_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2srxBclkSlv0R = crate::FieldReader;
#[doc = "Field `i2srx_bclk_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2srxBclkSlv0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn spi_rxd_0(&self) -> SpiRxd0R {
        SpiRxd0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn jtag_tck(&self) -> JtagTckR {
        JtagTckR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn mclk(&self) -> MclkR {
        MclkR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2srx_bclk_slv_0(&self) -> I2srxBclkSlv0R {
        I2srxBclkSlv0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rxd_0(&mut self) -> SpiRxd0W<Gpi7Spec> {
        SpiRxd0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_tck(&mut self) -> JtagTckW<Gpi7Spec> {
        JtagTckW::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MclkW<Gpi7Spec> {
        MclkW::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2srx_bclk_slv_0(&mut self) -> I2srxBclkSlv0W<Gpi7Spec> {
        I2srxBclkSlv0W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_rxd_0, jtag_tck, mclk, i2srx_bclk_slv_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi7Spec;
impl crate::RegisterSpec for Gpi7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi7::R`](R) reader structure"]
impl crate::Readable for Gpi7Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi7::W`](W) writer structure"]
impl crate::Writable for Gpi7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi7 to value 0x0334"]
impl crate::Resettable for Gpi7Spec {
    const RESET_VALUE: u32 = 0x0334;
}
