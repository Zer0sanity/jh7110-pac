#[doc = "Register `gpo_dout4` reader"]
pub type R = crate::R<GpoDout4Spec>;
#[doc = "Register `gpo_dout4` writer"]
pub type W = crate::W<GpoDout4Spec>;
#[doc = "Field `dout16` reader - The selected output signal for GPIO16. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout16R = crate::FieldReader;
#[doc = "Field `dout16` writer - The selected output signal for GPIO16. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout16W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout17` reader - The selected output signal for GPIO17. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout17R = crate::FieldReader;
#[doc = "Field `dout17` writer - The selected output signal for GPIO17. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout17W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout18` reader - The selected output signal for GPIO18. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout18R = crate::FieldReader;
#[doc = "Field `dout18` writer - The selected output signal for GPIO18. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout18W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout19` reader - The selected output signal for GPIO19. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout19R = crate::FieldReader;
#[doc = "Field `dout19` writer - The selected output signal for GPIO19. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout19W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO16. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout16(&self) -> Dout16R {
        Dout16R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO17. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout17(&self) -> Dout17R {
        Dout17R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO18. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout18(&self) -> Dout18R {
        Dout18R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO19. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout19(&self) -> Dout19R {
        Dout19R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO16. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout16(&mut self) -> Dout16W<GpoDout4Spec> {
        Dout16W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO17. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout17(&mut self) -> Dout17W<GpoDout4Spec> {
        Dout17W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO18. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout18(&mut self) -> Dout18W<GpoDout4Spec> {
        Dout18W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO19. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout19(&mut self) -> Dout19W<GpoDout4Spec> {
        Dout19W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout4Spec;
impl crate::RegisterSpec for GpoDout4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout4::R`](R) reader structure"]
impl crate::Readable for GpoDout4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout4::W`](W) writer structure"]
impl crate::Writable for GpoDout4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout4 to value 0x2000_0000"]
impl crate::Resettable for GpoDout4Spec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
