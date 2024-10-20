#[doc = "Register `gpo_dout13` reader"]
pub type R = crate::R<GpoDout13Spec>;
#[doc = "Register `gpo_dout13` writer"]
pub type W = crate::W<GpoDout13Spec>;
#[doc = "Field `dout52` reader - The selected output signal for GPIO52. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout52R = crate::FieldReader;
#[doc = "Field `dout52` writer - The selected output signal for GPIO52. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout52W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout53` reader - The selected output signal for GPIO53. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout53R = crate::FieldReader;
#[doc = "Field `dout53` writer - The selected output signal for GPIO53. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout53W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout54` reader - The selected output signal for GPIO54. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout54R = crate::FieldReader;
#[doc = "Field `dout54` writer - The selected output signal for GPIO54. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout54W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout55` reader - The selected output signal for GPIO55. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout55R = crate::FieldReader;
#[doc = "Field `dout55` writer - The selected output signal for GPIO55. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout55W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO52. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout52(&self) -> Dout52R {
        Dout52R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO53. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout53(&self) -> Dout53R {
        Dout53R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO54. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout54(&self) -> Dout54R {
        Dout54R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO55. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout55(&self) -> Dout55R {
        Dout55R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO52. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout52(&mut self) -> Dout52W<GpoDout13Spec> {
        Dout52W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO53. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout53(&mut self) -> Dout53W<GpoDout13Spec> {
        Dout53W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO54. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout54(&mut self) -> Dout54W<GpoDout13Spec> {
        Dout54W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO55. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout55(&mut self) -> Dout55W<GpoDout13Spec> {
        Dout55W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout13Spec;
impl crate::RegisterSpec for GpoDout13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout13::R`](R) reader structure"]
impl crate::Readable for GpoDout13Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout13::W`](W) writer structure"]
impl crate::Writable for GpoDout13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout13 to value 0x4b00_494a"]
impl crate::Resettable for GpoDout13Spec {
    const RESET_VALUE: u32 = 0x4b00_494a;
}
