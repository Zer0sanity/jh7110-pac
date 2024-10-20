#[doc = "Register `gpo_doen1` reader"]
pub type R = crate::R<GpoDoen1Spec>;
#[doc = "Register `gpo_doen1` writer"]
pub type W = crate::W<GpoDoen1Spec>;
#[doc = "Field `doen4` reader - The selected OEN signal for GPIO4. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen4R = crate::FieldReader;
#[doc = "Field `doen4` writer - The selected OEN signal for GPIO4. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen5` reader - The selected OEN signal for GPIO5. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen5R = crate::FieldReader;
#[doc = "Field `doen5` writer - The selected OEN signal for GPIO5. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen5W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen6` reader - The selected OEN signal for GPIO6. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen6R = crate::FieldReader;
#[doc = "Field `doen6` writer - The selected OEN signal for GPIO6. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen6W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen7` reader - The selected OEN signal for GPIO7. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen7R = crate::FieldReader;
#[doc = "Field `doen7` writer - The selected OEN signal for GPIO7. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen7W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO4. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen4(&self) -> Doen4R {
        Doen4R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO5. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen5(&self) -> Doen5R {
        Doen5R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO6. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen6(&self) -> Doen6R {
        Doen6R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO7. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen7(&self) -> Doen7R {
        Doen7R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO4. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen4(&mut self) -> Doen4W<GpoDoen1Spec> {
        Doen4W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO5. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen5(&mut self) -> Doen5W<GpoDoen1Spec> {
        Doen5W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO6. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen6(&mut self) -> Doen6W<GpoDoen1Spec> {
        Doen6W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO7. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen7(&mut self) -> Doen7W<GpoDoen1Spec> {
        Doen7W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen1Spec;
impl crate::RegisterSpec for GpoDoen1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen1::R`](R) reader structure"]
impl crate::Readable for GpoDoen1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen1::W`](W) writer structure"]
impl crate::Writable for GpoDoen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen1 to value 0x0001_0001"]
impl crate::Resettable for GpoDoen1Spec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
