#[doc = "Register `gpo_doen8` reader"]
pub type R = crate::R<GpoDoen8Spec>;
#[doc = "Register `gpo_doen8` writer"]
pub type W = crate::W<GpoDoen8Spec>;
#[doc = "Field `doen32` reader - The selected OEN signal for GPIO32. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen32R = crate::FieldReader;
#[doc = "Field `doen32` writer - The selected OEN signal for GPIO32. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen32W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen33` reader - The selected OEN signal for GPIO33. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen33R = crate::FieldReader;
#[doc = "Field `doen33` writer - The selected OEN signal for GPIO33. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen33W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen34` reader - The selected OEN signal for GPIO34. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen34R = crate::FieldReader;
#[doc = "Field `doen34` writer - The selected OEN signal for GPIO34. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen34W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen35` reader - The selected OEN signal for GPIO35. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen35R = crate::FieldReader;
#[doc = "Field `doen35` writer - The selected OEN signal for GPIO35. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen35W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO32. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen32(&self) -> Doen32R {
        Doen32R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO33. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen33(&self) -> Doen33R {
        Doen33R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO34. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen34(&self) -> Doen34R {
        Doen34R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO35. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen35(&self) -> Doen35R {
        Doen35R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO32. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen32(&mut self) -> Doen32W<GpoDoen8Spec> {
        Doen32W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO33. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen33(&mut self) -> Doen33W<GpoDoen8Spec> {
        Doen33W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO34. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen34(&mut self) -> Doen34W<GpoDoen8Spec> {
        Doen34W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO35. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen35(&mut self) -> Doen35W<GpoDoen8Spec> {
        Doen35W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen8Spec;
impl crate::RegisterSpec for GpoDoen8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen8::R`](R) reader structure"]
impl crate::Readable for GpoDoen8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen8::W`](W) writer structure"]
impl crate::Writable for GpoDoen8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen8 to value 0"]
impl crate::Resettable for GpoDoen8Spec {
    const RESET_VALUE: u32 = 0;
}
