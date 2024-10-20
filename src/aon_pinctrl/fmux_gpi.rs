#[doc = "Register `fmux_gpi` reader"]
pub type R = crate::R<FmuxGpiSpec>;
#[doc = "Register `fmux_gpi` writer"]
pub type W = crate::W<FmuxGpiSpec>;
#[doc = "Field `gpi_pmu_wakeup0` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GpiPmuWakeup0R = crate::FieldReader;
#[doc = "Field `gpi_pmu_wakeup0` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GpiPmuWakeup0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpi_pmu_wakeup1` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GpiPmuWakeup1R = crate::FieldReader;
#[doc = "Field `gpi_pmu_wakeup1` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GpiPmuWakeup1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpi_pmu_wakeup2` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GpiPmuWakeup2R = crate::FieldReader;
#[doc = "Field `gpi_pmu_wakeup2` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GpiPmuWakeup2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpi_pmu_wakeup3` reader - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GpiPmuWakeup3R = crate::FieldReader;
#[doc = "Field `gpi_pmu_wakeup3` writer - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type GpiPmuWakeup3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn gpi_pmu_wakeup0(&self) -> GpiPmuWakeup0R {
        GpiPmuWakeup0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn gpi_pmu_wakeup1(&self) -> GpiPmuWakeup1R {
        GpiPmuWakeup1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn gpi_pmu_wakeup2(&self) -> GpiPmuWakeup2R {
        GpiPmuWakeup2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn gpi_pmu_wakeup3(&self) -> GpiPmuWakeup3R {
        GpiPmuWakeup3R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn gpi_pmu_wakeup0(&mut self) -> GpiPmuWakeup0W<FmuxGpiSpec> {
        GpiPmuWakeup0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn gpi_pmu_wakeup1(&mut self) -> GpiPmuWakeup1W<FmuxGpiSpec> {
        GpiPmuWakeup1W::new(self, 8)
    }
    #[doc = "Bits 16:18 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn gpi_pmu_wakeup2(&mut self) -> GpiPmuWakeup2W<FmuxGpiSpec> {
        GpiPmuWakeup2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - The register value indicates the selected GPIO number + 2 (GPIO2-GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn gpi_pmu_wakeup3(&mut self) -> GpiPmuWakeup3W<FmuxGpiSpec> {
        GpiPmuWakeup3W::new(self, 24)
    }
}
#[doc = "The register can be used to configure the selected GPIO connector number for input signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_gpi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_gpi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmuxGpiSpec;
impl crate::RegisterSpec for FmuxGpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmux_gpi::R`](R) reader structure"]
impl crate::Readable for FmuxGpiSpec {}
#[doc = "`write(|w| ..)` method takes [`fmux_gpi::W`](W) writer structure"]
impl crate::Writable for FmuxGpiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fmux_gpi to value 0x0504_0302"]
impl crate::Resettable for FmuxGpiSpec {
    const RESET_VALUE: u32 = 0x0504_0302;
}
