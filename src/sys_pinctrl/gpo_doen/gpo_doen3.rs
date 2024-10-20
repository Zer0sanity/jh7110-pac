#[doc = "Register `gpo_doen3` reader"]
pub type R = crate::R<GpoDoen3Spec>;
#[doc = "Register `gpo_doen3` writer"]
pub type W = crate::W<GpoDoen3Spec>;
#[doc = "Field `doen12` reader - The selected OEN signal for GPIO12. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen12R = crate::FieldReader;
#[doc = "Field `doen12` writer - The selected OEN signal for GPIO12. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen12W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen13` reader - The selected OEN signal for GPIO13. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen13R = crate::FieldReader;
#[doc = "Field `doen13` writer - The selected OEN signal for GPIO13. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen13W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen14` reader - The selected OEN signal for GPIO14. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen14R = crate::FieldReader;
#[doc = "Field `doen14` writer - The selected OEN signal for GPIO14. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen14W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen15` reader - The selected OEN signal for GPIO15. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen15R = crate::FieldReader;
#[doc = "Field `doen15` writer - The selected OEN signal for GPIO15. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen15W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO12. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen12(&self) -> Doen12R {
        Doen12R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO13. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen13(&self) -> Doen13R {
        Doen13R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO14. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen14(&self) -> Doen14R {
        Doen14R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO15. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen15(&self) -> Doen15R {
        Doen15R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO12. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen12(&mut self) -> Doen12W<GpoDoen3Spec> {
        Doen12W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO13. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen13(&mut self) -> Doen13W<GpoDoen3Spec> {
        Doen13W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO14. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen14(&mut self) -> Doen14W<GpoDoen3Spec> {
        Doen14W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO15. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen15(&mut self) -> Doen15W<GpoDoen3Spec> {
        Doen15W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen3Spec;
impl crate::RegisterSpec for GpoDoen3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen3::R`](R) reader structure"]
impl crate::Readable for GpoDoen3Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen3::W`](W) writer structure"]
impl crate::Writable for GpoDoen3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen3 to value 0x0101"]
impl crate::Resettable for GpoDoen3Spec {
    const RESET_VALUE: u32 = 0x0101;
}
