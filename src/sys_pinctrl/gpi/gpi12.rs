#[doc = "Register `gpi12` reader"]
pub type R = crate::R<Gpi12Spec>;
#[doc = "Register `gpi12` writer"]
pub type W = crate::W<Gpi12Spec>;
#[doc = "Field `sdio_cdata_3` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata3R = crate::FieldReader;
#[doc = "Field `sdio_cdata_3` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_4` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata4R = crate::FieldReader;
#[doc = "Field `sdio_cdata_4` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_5` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata5R = crate::FieldReader;
#[doc = "Field `sdio_cdata_5` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_6` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata6R = crate::FieldReader;
#[doc = "Field `sdio_cdata_6` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_3(&self) -> SdioCdata3R {
        SdioCdata3R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_4(&self) -> SdioCdata4R {
        SdioCdata4R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_5(&self) -> SdioCdata5R {
        SdioCdata5R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_6(&self) -> SdioCdata6R {
        SdioCdata6R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_3(&mut self) -> SdioCdata3W<Gpi12Spec> {
        SdioCdata3W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_4(&mut self) -> SdioCdata4W<Gpi12Spec> {
        SdioCdata4W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_5(&mut self) -> SdioCdata5W<Gpi12Spec> {
        SdioCdata5W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_6(&mut self) -> SdioCdata6W<Gpi12Spec> {
        SdioCdata6W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_cdata_3, sdio_cdata_4, sdio_cdata_5, sdio_cdata_6\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi12Spec;
impl crate::RegisterSpec for Gpi12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi12::R`](R) reader structure"]
impl crate::Readable for Gpi12Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi12::W`](W) writer structure"]
impl crate::Writable for Gpi12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi12 to value 0"]
impl crate::Resettable for Gpi12Spec {
    const RESET_VALUE: u32 = 0;
}
