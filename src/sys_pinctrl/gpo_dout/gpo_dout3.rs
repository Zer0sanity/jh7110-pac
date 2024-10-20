#[doc = "Register `gpo_dout3` reader"]
pub type R = crate::R<GpoDout3Spec>;
#[doc = "Register `gpo_dout3` writer"]
pub type W = crate::W<GpoDout3Spec>;
#[doc = "Field `dout12` reader - The selected output signal for GPIO12. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout12R = crate::FieldReader;
#[doc = "Field `dout12` writer - The selected output signal for GPIO12. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout12W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout13` reader - The selected output signal for GPIO13. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout13R = crate::FieldReader;
#[doc = "Field `dout13` writer - The selected output signal for GPIO13. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout13W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout14` reader - The selected output signal for GPIO14. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout14R = crate::FieldReader;
#[doc = "Field `dout14` writer - The selected output signal for GPIO14. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout14W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout15` reader - The selected output signal for GPIO15. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout15R = crate::FieldReader;
#[doc = "Field `dout15` writer - The selected output signal for GPIO15. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout15W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO12. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout12(&self) -> Dout12R {
        Dout12R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO13. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout13(&self) -> Dout13R {
        Dout13R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO14. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout14(&self) -> Dout14R {
        Dout14R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO15. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout15(&self) -> Dout15R {
        Dout15R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO12. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout12(&mut self) -> Dout12W<GpoDout3Spec> {
        Dout12W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO13. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout13(&mut self) -> Dout13W<GpoDout3Spec> {
        Dout13W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO14. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout14(&mut self) -> Dout14W<GpoDout3Spec> {
        Dout14W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO15. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout15(&mut self) -> Dout15W<GpoDout3Spec> {
        Dout15W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout3Spec;
impl crate::RegisterSpec for GpoDout3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout3::R`](R) reader structure"]
impl crate::Readable for GpoDout3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout3::W`](W) writer structure"]
impl crate::Writable for GpoDout3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout3 to value 0"]
impl crate::Resettable for GpoDout3Spec {
    const RESET_VALUE: u32 = 0;
}
