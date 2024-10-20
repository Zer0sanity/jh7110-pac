#[doc = "Register `gpi9` reader"]
pub type R = crate::R<Gpi9Spec>;
#[doc = "Register `gpi9` writer"]
pub type W = crate::W<Gpi9Spec>;
#[doc = "Field `pcm_rxd_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PcmRxd0R = crate::FieldReader;
#[doc = "Field `pcm_rxd_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PcmRxd0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `pcm_synon_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PcmSynon0R = crate::FieldReader;
#[doc = "Field `pcm_synon_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type PcmSynon0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `can_rxd_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type CanRxd1R = crate::FieldReader;
#[doc = "Field `can_rxd_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type CanRxd1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `i2c_clk_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk1R = crate::FieldReader;
#[doc = "Field `i2c_clk_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type I2cClk1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn pcm_rxd_0(&self) -> PcmRxd0R {
        PcmRxd0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn pcm_synon_0(&self) -> PcmSynon0R {
        PcmSynon0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn can_rxd_1(&self) -> CanRxd1R {
        CanRxd1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn i2c_clk_1(&self) -> I2cClk1R {
        I2cClk1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn pcm_rxd_0(&mut self) -> PcmRxd0W<Gpi9Spec> {
        PcmRxd0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn pcm_synon_0(&mut self) -> PcmSynon0W<Gpi9Spec> {
        PcmSynon0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn can_rxd_1(&mut self) -> CanRxd1W<Gpi9Spec> {
        CanRxd1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_clk_1(&mut self) -> I2cClk1W<Gpi9Spec> {
        I2cClk1W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[pcm_rxd_0, pcm_synon_0, can_rxd_1, i2c_clk_1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi9Spec;
impl crate::RegisterSpec for Gpi9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi9::R`](R) reader structure"]
impl crate::Readable for Gpi9Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi9::W`](W) writer structure"]
impl crate::Writable for Gpi9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi9 to value 0"]
impl crate::Resettable for Gpi9Spec {
    const RESET_VALUE: u32 = 0;
}
