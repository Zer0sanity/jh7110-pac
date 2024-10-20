#[doc = "Register `gpo_doen7` reader"]
pub type R = crate::R<GpoDoen7Spec>;
#[doc = "Register `gpo_doen7` writer"]
pub type W = crate::W<GpoDoen7Spec>;
#[doc = "Field `doen28` reader - The selected OEN signal for GPIO28. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen28R = crate::FieldReader;
#[doc = "Field `doen28` writer - The selected OEN signal for GPIO28. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen28W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen29` reader - The selected OEN signal for GPIO29. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen29R = crate::FieldReader;
#[doc = "Field `doen29` writer - The selected OEN signal for GPIO29. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen29W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen30` reader - The selected OEN signal for GPIO30. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen30R = crate::FieldReader;
#[doc = "Field `doen30` writer - The selected OEN signal for GPIO30. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen30W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen31` reader - The selected OEN signal for GPIO31. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen31R = crate::FieldReader;
#[doc = "Field `doen31` writer - The selected OEN signal for GPIO31. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen31W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO28. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen28(&self) -> Doen28R {
        Doen28R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO29. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen29(&self) -> Doen29R {
        Doen29R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO30. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen30(&self) -> Doen30R {
        Doen30R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO31. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen31(&self) -> Doen31R {
        Doen31R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO28. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen28(&mut self) -> Doen28W<GpoDoen7Spec> {
        Doen28W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO29. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen29(&mut self) -> Doen29W<GpoDoen7Spec> {
        Doen29W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO30. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen30(&mut self) -> Doen30W<GpoDoen7Spec> {
        Doen30W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO31. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen31(&mut self) -> Doen31W<GpoDoen7Spec> {
        Doen31W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen7Spec;
impl crate::RegisterSpec for GpoDoen7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen7::R`](R) reader structure"]
impl crate::Readable for GpoDoen7Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen7::W`](W) writer structure"]
impl crate::Writable for GpoDoen7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen7 to value 0"]
impl crate::Resettable for GpoDoen7Spec {
    const RESET_VALUE: u32 = 0;
}
