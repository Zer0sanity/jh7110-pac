#[doc = "Register `gpo_doen13` reader"]
pub type R = crate::R<GpoDoen13Spec>;
#[doc = "Register `gpo_doen13` writer"]
pub type W = crate::W<GpoDoen13Spec>;
#[doc = "Field `doen52` reader - The selected OEN signal for GPIO52. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen52R = crate::FieldReader;
#[doc = "Field `doen52` writer - The selected OEN signal for GPIO52. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen52W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen53` reader - The selected OEN signal for GPIO53. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen53R = crate::FieldReader;
#[doc = "Field `doen53` writer - The selected OEN signal for GPIO53. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen53W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen54` reader - The selected OEN signal for GPIO54. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen54R = crate::FieldReader;
#[doc = "Field `doen54` writer - The selected OEN signal for GPIO54. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen54W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen55` reader - The selected OEN signal for GPIO55. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen55R = crate::FieldReader;
#[doc = "Field `doen55` writer - The selected OEN signal for GPIO55. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen55W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO52. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen52(&self) -> Doen52R {
        Doen52R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO53. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen53(&self) -> Doen53R {
        Doen53R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO54. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen54(&self) -> Doen54R {
        Doen54R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO55. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen55(&self) -> Doen55R {
        Doen55R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO52. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen52(&mut self) -> Doen52W<GpoDoen13Spec> {
        Doen52W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO53. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen53(&mut self) -> Doen53W<GpoDoen13Spec> {
        Doen53W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO54. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen54(&mut self) -> Doen54W<GpoDoen13Spec> {
        Doen54W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO55. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen55(&mut self) -> Doen55W<GpoDoen13Spec> {
        Doen55W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen13Spec;
impl crate::RegisterSpec for GpoDoen13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen13::R`](R) reader structure"]
impl crate::Readable for GpoDoen13Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen13::W`](W) writer structure"]
impl crate::Writable for GpoDoen13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen13 to value 0x1d01_1c1c"]
impl crate::Resettable for GpoDoen13Spec {
    const RESET_VALUE: u32 = 0x1d01_1c1c;
}
