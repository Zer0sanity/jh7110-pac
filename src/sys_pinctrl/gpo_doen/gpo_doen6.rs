#[doc = "Register `gpo_doen6` reader"]
pub type R = crate::R<GpoDoen6Spec>;
#[doc = "Register `gpo_doen6` writer"]
pub type W = crate::W<GpoDoen6Spec>;
#[doc = "Field `doen24` reader - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen24R = crate::FieldReader;
#[doc = "Field `doen24` writer - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen24W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen25` reader - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen25R = crate::FieldReader;
#[doc = "Field `doen25` writer - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen25W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen26` reader - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen26R = crate::FieldReader;
#[doc = "Field `doen26` writer - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen26W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen27` reader - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen27R = crate::FieldReader;
#[doc = "Field `doen27` writer - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen27W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen24(&self) -> Doen24R {
        Doen24R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen25(&self) -> Doen25R {
        Doen25R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen26(&self) -> Doen26R {
        Doen26R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen27(&self) -> Doen27R {
        Doen27R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO24. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen24(&mut self) -> Doen24W<GpoDoen6Spec> {
        Doen24W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO25. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen25(&mut self) -> Doen25W<GpoDoen6Spec> {
        Doen25W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO26. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen26(&mut self) -> Doen26W<GpoDoen6Spec> {
        Doen26W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO27. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen27(&mut self) -> Doen27W<GpoDoen6Spec> {
        Doen27W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen6Spec;
impl crate::RegisterSpec for GpoDoen6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen6::R`](R) reader structure"]
impl crate::Readable for GpoDoen6Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen6::W`](W) writer structure"]
impl crate::Writable for GpoDoen6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen6 to value 0"]
impl crate::Resettable for GpoDoen6Spec {
    const RESET_VALUE: u32 = 0;
}
