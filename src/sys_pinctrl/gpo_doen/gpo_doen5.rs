#[doc = "Register `gpo_doen5` reader"]
pub type R = crate::R<GpoDoen5Spec>;
#[doc = "Register `gpo_doen5` writer"]
pub type W = crate::W<GpoDoen5Spec>;
#[doc = "Field `doen20` reader - The selected OEN signal for GPIO20. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen20R = crate::FieldReader;
#[doc = "Field `doen20` writer - The selected OEN signal for GPIO20. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen20W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen21` reader - The selected OEN signal for GPIO21. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen21R = crate::FieldReader;
#[doc = "Field `doen21` writer - The selected OEN signal for GPIO21. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen21W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen22` reader - The selected OEN signal for GPIO22. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen22R = crate::FieldReader;
#[doc = "Field `doen22` writer - The selected OEN signal for GPIO22. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen22W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen23` reader - The selected OEN signal for GPIO23. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen23R = crate::FieldReader;
#[doc = "Field `doen23` writer - The selected OEN signal for GPIO23. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen23W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO20. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen20(&self) -> Doen20R {
        Doen20R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO21. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen21(&self) -> Doen21R {
        Doen21R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO22. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen22(&self) -> Doen22R {
        Doen22R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO23. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen23(&self) -> Doen23R {
        Doen23R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO20. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen20(&mut self) -> Doen20W<GpoDoen5Spec> {
        Doen20W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO21. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen21(&mut self) -> Doen21W<GpoDoen5Spec> {
        Doen21W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO22. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen22(&mut self) -> Doen22W<GpoDoen5Spec> {
        Doen22W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO23. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen23(&mut self) -> Doen23W<GpoDoen5Spec> {
        Doen23W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen5Spec;
impl crate::RegisterSpec for GpoDoen5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen5::R`](R) reader structure"]
impl crate::Readable for GpoDoen5Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen5::W`](W) writer structure"]
impl crate::Writable for GpoDoen5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen5 to value 0"]
impl crate::Resettable for GpoDoen5Spec {
    const RESET_VALUE: u32 = 0;
}
