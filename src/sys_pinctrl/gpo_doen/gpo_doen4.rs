#[doc = "Register `gpo_doen4` reader"]
pub type R = crate::R<GpoDoen4Spec>;
#[doc = "Register `gpo_doen4` writer"]
pub type W = crate::W<GpoDoen4Spec>;
#[doc = "Field `doen16` reader - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen16R = crate::FieldReader;
#[doc = "Field `doen16` writer - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen16W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen17` reader - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen17R = crate::FieldReader;
#[doc = "Field `doen17` writer - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen17W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen18` reader - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen18R = crate::FieldReader;
#[doc = "Field `doen18` writer - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen18W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen19` reader - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen19R = crate::FieldReader;
#[doc = "Field `doen19` writer - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen19W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen16(&self) -> Doen16R {
        Doen16R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen17(&self) -> Doen17R {
        Doen17R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen18(&self) -> Doen18R {
        Doen18R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen19(&self) -> Doen19R {
        Doen19R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO16. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen16(&mut self) -> Doen16W<GpoDoen4Spec> {
        Doen16W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO17. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen17(&mut self) -> Doen17W<GpoDoen4Spec> {
        Doen17W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO18. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen18(&mut self) -> Doen18W<GpoDoen4Spec> {
        Doen18W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO19. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen19(&mut self) -> Doen19W<GpoDoen4Spec> {
        Doen19W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen4Spec;
impl crate::RegisterSpec for GpoDoen4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen4::R`](R) reader structure"]
impl crate::Readable for GpoDoen4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen4::W`](W) writer structure"]
impl crate::Writable for GpoDoen4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen4 to value 0x0100_0000"]
impl crate::Resettable for GpoDoen4Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
