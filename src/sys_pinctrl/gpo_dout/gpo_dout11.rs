#[doc = "Register `gpo_dout11` reader"]
pub type R = crate::R<GpoDout11Spec>;
#[doc = "Register `gpo_dout11` writer"]
pub type W = crate::W<GpoDout11Spec>;
#[doc = "Field `dout44` reader - The selected output signal for GPIO44. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout44R = crate::FieldReader;
#[doc = "Field `dout44` writer - The selected output signal for GPIO44. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout44W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout45` reader - The selected output signal for GPIO45. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout45R = crate::FieldReader;
#[doc = "Field `dout45` writer - The selected output signal for GPIO45. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout45W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout46` reader - The selected output signal for GPIO46. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout46R = crate::FieldReader;
#[doc = "Field `dout46` writer - The selected output signal for GPIO46. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout46W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout47` reader - The selected output signal for GPIO47. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout47R = crate::FieldReader;
#[doc = "Field `dout47` writer - The selected output signal for GPIO47. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout47W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO44. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout44(&self) -> Dout44R {
        Dout44R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO45. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout45(&self) -> Dout45R {
        Dout45R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO46. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout46(&self) -> Dout46R {
        Dout46R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO47. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout47(&self) -> Dout47R {
        Dout47R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO44. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout44(&mut self) -> Dout44W<GpoDout11Spec> {
        Dout44W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO45. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout45(&mut self) -> Dout45W<GpoDout11Spec> {
        Dout45W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO46. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout46(&mut self) -> Dout46W<GpoDout11Spec> {
        Dout46W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO47. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout47(&mut self) -> Dout47W<GpoDout11Spec> {
        Dout47W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout11Spec;
impl crate::RegisterSpec for GpoDout11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout11::R`](R) reader structure"]
impl crate::Readable for GpoDout11Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout11::W`](W) writer structure"]
impl crate::Writable for GpoDout11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout11 to value 0x005b_5c00"]
impl crate::Resettable for GpoDout11Spec {
    const RESET_VALUE: u32 = 0x005b_5c00;
}
