#[doc = "Register `gpo_dout7` reader"]
pub type R = crate::R<GpoDout7Spec>;
#[doc = "Register `gpo_dout7` writer"]
pub type W = crate::W<GpoDout7Spec>;
#[doc = "Field `dout28` reader - The selected output signal for GPIO28. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout28R = crate::FieldReader;
#[doc = "Field `dout28` writer - The selected output signal for GPIO28. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout28W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout29` reader - The selected output signal for GPIO29. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout29R = crate::FieldReader;
#[doc = "Field `dout29` writer - The selected output signal for GPIO29. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout29W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout30` reader - The selected output signal for GPIO30. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout30R = crate::FieldReader;
#[doc = "Field `dout30` writer - The selected output signal for GPIO30. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout30W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout31` reader - The selected output signal for GPIO31. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout31R = crate::FieldReader;
#[doc = "Field `dout31` writer - The selected output signal for GPIO31. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout31W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO28. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout28(&self) -> Dout28R {
        Dout28R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO29. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout29(&self) -> Dout29R {
        Dout29R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO30. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout30(&self) -> Dout30R {
        Dout30R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO31. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout31(&self) -> Dout31R {
        Dout31R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO28. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout28(&mut self) -> Dout28W<GpoDout7Spec> {
        Dout28W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO29. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout29(&mut self) -> Dout29W<GpoDout7Spec> {
        Dout29W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO30. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout30(&mut self) -> Dout30W<GpoDout7Spec> {
        Dout30W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO31. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout31(&mut self) -> Dout31W<GpoDout7Spec> {
        Dout31W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout7Spec;
impl crate::RegisterSpec for GpoDout7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout7::R`](R) reader structure"]
impl crate::Readable for GpoDout7Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout7::W`](W) writer structure"]
impl crate::Writable for GpoDout7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout7 to value 0"]
impl crate::Resettable for GpoDout7Spec {
    const RESET_VALUE: u32 = 0;
}
