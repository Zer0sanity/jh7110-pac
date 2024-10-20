#[doc = "Register `gpo_doen9` reader"]
pub type R = crate::R<GpoDoen9Spec>;
#[doc = "Register `gpo_doen9` writer"]
pub type W = crate::W<GpoDoen9Spec>;
#[doc = "Field `doen36` reader - The selected OEN signal for GPIO36. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen36R = crate::FieldReader;
#[doc = "Field `doen36` writer - The selected OEN signal for GPIO36. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen36W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen37` reader - The selected OEN signal for GPIO37. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen37R = crate::FieldReader;
#[doc = "Field `doen37` writer - The selected OEN signal for GPIO37. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen37W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen38` reader - The selected OEN signal for GPIO38. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen38R = crate::FieldReader;
#[doc = "Field `doen38` writer - The selected OEN signal for GPIO38. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen38W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen39` reader - The selected OEN signal for GPIO39. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen39R = crate::FieldReader;
#[doc = "Field `doen39` writer - The selected OEN signal for GPIO39. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen39W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO36. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen36(&self) -> Doen36R {
        Doen36R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO37. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen37(&self) -> Doen37R {
        Doen37R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO38. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen38(&self) -> Doen38R {
        Doen38R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO39. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen39(&self) -> Doen39R {
        Doen39R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO36. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen36(&mut self) -> Doen36W<GpoDoen9Spec> {
        Doen36W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO37. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen37(&mut self) -> Doen37W<GpoDoen9Spec> {
        Doen37W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO38. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen38(&mut self) -> Doen38W<GpoDoen9Spec> {
        Doen38W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO39. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen39(&mut self) -> Doen39W<GpoDoen9Spec> {
        Doen39W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen9Spec;
impl crate::RegisterSpec for GpoDoen9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen9::R`](R) reader structure"]
impl crate::Readable for GpoDoen9Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen9::W`](W) writer structure"]
impl crate::Writable for GpoDoen9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen9 to value 0x2322_0605"]
impl crate::Resettable for GpoDoen9Spec {
    const RESET_VALUE: u32 = 0x2322_0605;
}
