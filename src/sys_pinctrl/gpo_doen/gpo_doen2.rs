#[doc = "Register `gpo_doen2` reader"]
pub type R = crate::R<GpoDoen2Spec>;
#[doc = "Register `gpo_doen2` writer"]
pub type W = crate::W<GpoDoen2Spec>;
#[doc = "Field `doen8` reader - The selected OEN signal for GPIO8. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen8R = crate::FieldReader;
#[doc = "Field `doen8` writer - The selected OEN signal for GPIO8. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen8W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen9` reader - The selected OEN signal for GPIO9. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen9R = crate::FieldReader;
#[doc = "Field `doen9` writer - The selected OEN signal for GPIO9. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen9W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen10` reader - The selected OEN signal for GPIO10. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen10R = crate::FieldReader;
#[doc = "Field `doen10` writer - The selected OEN signal for GPIO10. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen10W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen11` reader - The selected OEN signal for GPIO11. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen11R = crate::FieldReader;
#[doc = "Field `doen11` writer - The selected OEN signal for GPIO11. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen11W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO8. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen8(&self) -> Doen8R {
        Doen8R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO9. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen9(&self) -> Doen9R {
        Doen9R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO10. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen10(&self) -> Doen10R {
        Doen10R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO11. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen11(&self) -> Doen11R {
        Doen11R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO8. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen8(&mut self) -> Doen8W<GpoDoen2Spec> {
        Doen8W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO9. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen9(&mut self) -> Doen9W<GpoDoen2Spec> {
        Doen9W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO10. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen10(&mut self) -> Doen10W<GpoDoen2Spec> {
        Doen10W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO11. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen11(&mut self) -> Doen11W<GpoDoen2Spec> {
        Doen11W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen2Spec;
impl crate::RegisterSpec for GpoDoen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen2::R`](R) reader structure"]
impl crate::Readable for GpoDoen2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen2::W`](W) writer structure"]
impl crate::Writable for GpoDoen2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen2 to value 0x0701_0100"]
impl crate::Resettable for GpoDoen2Spec {
    const RESET_VALUE: u32 = 0x0701_0100;
}
