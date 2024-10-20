#[doc = "Register `gpi8` reader"]
pub type R = crate::R<Gpi8Spec>;
#[doc = "Register `gpi8` writer"]
pub type W = crate::W<Gpi8Spec>;
#[doc = "Field `i2srx_lrck_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2srxLrckSlv0R = crate::FieldReader;
#[doc = "Field `i2srx_lrck_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2srxLrckSlv0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2stx_bclk_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2stxBclkSlv0R = crate::FieldReader;
#[doc = "Field `i2stx_bclk_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2stxBclkSlv0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2stx_lrck_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2stxLrckSlv0R = crate::FieldReader;
#[doc = "Field `i2stx_lrck_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2stxLrckSlv0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tdm_clk_slv_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type TdmClkSlv0R = crate::FieldReader;
#[doc = "Field `tdm_clk_slv_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type TdmClkSlv0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2srx_lrck_slv_0(&self) -> I2srxLrckSlv0R {
        I2srxLrckSlv0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2stx_bclk_slv_0(&self) -> I2stxBclkSlv0R {
        I2stxBclkSlv0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2stx_lrck_slv_0(&self) -> I2stxLrckSlv0R {
        I2stxLrckSlv0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn tdm_clk_slv_0(&self) -> TdmClkSlv0R {
        TdmClkSlv0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2srx_lrck_slv_0(&mut self) -> I2srxLrckSlv0W<Gpi8Spec> {
        I2srxLrckSlv0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2stx_bclk_slv_0(&mut self) -> I2stxBclkSlv0W<Gpi8Spec> {
        I2stxBclkSlv0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2stx_lrck_slv_0(&mut self) -> I2stxLrckSlv0W<Gpi8Spec> {
        I2stxLrckSlv0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn tdm_clk_slv_0(&mut self) -> TdmClkSlv0W<Gpi8Spec> {
        TdmClkSlv0W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2srx_lrck_slv_0, i2stx_bclk_slv_0, i2stx_lrck_slv_0, tdm_clk_slv_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi8Spec;
impl crate::RegisterSpec for Gpi8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi8::R`](R) reader structure"]
impl crate::Readable for Gpi8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi8::W`](W) writer structure"]
impl crate::Writable for Gpi8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi8 to value 0"]
impl crate::Resettable for Gpi8Spec {
    const RESET_VALUE: u32 = 0;
}
