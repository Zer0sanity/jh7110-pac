#[doc = "Register `gpi4` reader"]
pub type R = crate::R<Gpi4Spec>;
#[doc = "Register `gpi4` writer"]
pub type W = crate::W<Gpi4Spec>;
#[doc = "Field `hifi4_jtdi` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Hifi4JtdiR = crate::FieldReader;
#[doc = "Field `hifi4_jtdi` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Hifi4JtdiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hifi4_jtms` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Hifi4JtmsR = crate::FieldReader;
#[doc = "Field `hifi4_jtms` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Hifi4JtmsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `hifi4_jtrstn` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Hifi4JtrstnR = crate::FieldReader;
#[doc = "Field `hifi4_jtrstn` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type Hifi4JtrstnW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `jtag_tdi` reader - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JtagTdiR = crate::FieldReader;
#[doc = "Field `jtag_tdi` writer - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
pub type JtagTdiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hifi4_jtdi(&self) -> Hifi4JtdiR {
        Hifi4JtdiR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hifi4_jtms(&self) -> Hifi4JtmsR {
        Hifi4JtmsR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn hifi4_jtrstn(&self) -> Hifi4JtrstnR {
        Hifi4JtrstnR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    pub fn jtag_tdi(&self) -> JtagTdiR {
        JtagTdiR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_jtdi(&mut self) -> Hifi4JtdiW<Gpi4Spec> {
        Hifi4JtdiW::new(self, 0)
    }
    #[doc = "Bits 8:14 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_jtms(&mut self) -> Hifi4JtmsW<Gpi4Spec> {
        Hifi4JtmsW::new(self, 8)
    }
    #[doc = "Bits 16:22 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_jtrstn(&mut self) -> Hifi4JtrstnW<Gpi4Spec> {
        Hifi4JtrstnW::new(self, 16)
    }
    #[doc = "Bits 24:30 - The register value indicates the selected GPIO number + 2 (GPIO2 - GPIO63, GPIO0 and GPIO1 are not available) for the input signal."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_tdi(&mut self) -> JtagTdiW<Gpi4Spec> {
        JtagTdiW::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[hifi4_jtdi, hifi4_jtms, hifi4_jtrstn, jtag_tdi\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpi4Spec;
impl crate::RegisterSpec for Gpi4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpi4::R`](R) reader structure"]
impl crate::Readable for Gpi4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpi4::W`](W) writer structure"]
impl crate::Writable for Gpi4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpi4 to value 0x040f_0e0c"]
impl crate::Resettable for Gpi4Spec {
    const RESET_VALUE: u32 = 0x040f_0e0c;
}
