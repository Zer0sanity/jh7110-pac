#[doc = "Register `gpo_doen10` reader"]
pub type R = crate::R<GpoDoen10Spec>;
#[doc = "Register `gpo_doen10` writer"]
pub type W = crate::W<GpoDoen10Spec>;
#[doc = "Field `doen40` reader - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen40R = crate::FieldReader;
#[doc = "Field `doen40` writer - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen40W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen41` reader - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen41R = crate::FieldReader;
#[doc = "Field `doen41` writer - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen41W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen42` reader - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen42R = crate::FieldReader;
#[doc = "Field `doen42` writer - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen42W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen43` reader - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen43R = crate::FieldReader;
#[doc = "Field `doen43` writer - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen43W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen40(&self) -> Doen40R {
        Doen40R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen41(&self) -> Doen41R {
        Doen41R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen42(&self) -> Doen42R {
        Doen42R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen43(&self) -> Doen43R {
        Doen43R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO40. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen40(&mut self) -> Doen40W<GpoDoen10Spec> {
        Doen40W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO41. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen41(&mut self) -> Doen41W<GpoDoen10Spec> {
        Doen41W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO42. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen42(&mut self) -> Doen42W<GpoDoen10Spec> {
        Doen42W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO43. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen43(&mut self) -> Doen43W<GpoDoen10Spec> {
        Doen43W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen10Spec;
impl crate::RegisterSpec for GpoDoen10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen10::R`](R) reader structure"]
impl crate::Readable for GpoDoen10Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen10::W`](W) writer structure"]
impl crate::Writable for GpoDoen10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen10 to value 0x0100_0001"]
impl crate::Resettable for GpoDoen10Spec {
    const RESET_VALUE: u32 = 0x0100_0001;
}
