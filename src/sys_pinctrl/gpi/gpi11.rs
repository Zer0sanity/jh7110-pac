#[doc = "Register `gpi11` reader"]
pub type R = crate::R<Gpi11Spec>;
#[doc = "Register `gpi11` writer"]
pub type W = crate::W<Gpi11Spec>;
#[doc = "Field `sdio_ccmd_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCcmd1R = crate::FieldReader;
#[doc = "Field `sdio_ccmd_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCcmd1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_0` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata0R = crate::FieldReader;
#[doc = "Field `sdio_cdata_0` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_1` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata1R = crate::FieldReader;
#[doc = "Field `sdio_cdata_1` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdio_cdata_2` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata2R = crate::FieldReader;
#[doc = "Field `sdio_cdata_2` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type SdioCdata2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_ccmd_1(&self) -> SdioCcmd1R {
        SdioCcmd1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_0(&self) -> SdioCdata0R {
        SdioCdata0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_1(&self) -> SdioCdata1R {
        SdioCdata1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn sdio_cdata_2(&self) -> SdioCdata2R {
        SdioCdata2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ccmd_1(&mut self) -> SdioCcmd1W<Gpi11Spec> {
        SdioCcmd1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_0(&mut self) -> SdioCdata0W<Gpi11Spec> {
        SdioCdata0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_1(&mut self) -> SdioCdata1W<Gpi11Spec> {
        SdioCdata1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cdata_2(&mut self) -> SdioCdata2W<Gpi11Spec> {
        SdioCdata2W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_ccmd_1, sdio_cdata_0, sdio_cdata_1, sdio_cdata_2\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi11Spec;
impl crate::RegisterSpec for Gpi11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi11::R`](R) reader structure"]
impl crate::Readable for Gpi11Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi11::W`](W) writer structure"]
impl crate::Writable for Gpi11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi11 to value 0"]
impl crate::Resettable for Gpi11Spec {
    const RESET_VALUE: u32 = 0;
}
