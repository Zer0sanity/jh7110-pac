#[doc = "Register `gpo_doen15` reader"]
pub type R = crate::R<GpoDoen15Spec>;
#[doc = "Register `gpo_doen15` writer"]
pub type W = crate::W<GpoDoen15Spec>;
#[doc = "Field `doen60` reader - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen60R = crate::FieldReader;
#[doc = "Field `doen60` writer - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen60W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen61` reader - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen61R = crate::FieldReader;
#[doc = "Field `doen61` writer - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen61W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen62` reader - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen62R = crate::FieldReader;
#[doc = "Field `doen62` writer - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen62W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen63` reader - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen63R = crate::FieldReader;
#[doc = "Field `doen63` writer - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen63W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen60(&self) -> Doen60R {
        Doen60R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen61(&self) -> Doen61R {
        Doen61R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen62(&self) -> Doen62R {
        Doen62R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen63(&self) -> Doen63R {
        Doen63R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO60. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen60(&mut self) -> Doen60W<GpoDoen15Spec> {
        Doen60W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO61. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen61(&mut self) -> Doen61W<GpoDoen15Spec> {
        Doen61W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO62. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen62(&mut self) -> Doen62W<GpoDoen15Spec> {
        Doen62W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO63. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen63(&mut self) -> Doen63W<GpoDoen15Spec> {
        Doen63W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen15Spec;
impl crate::RegisterSpec for GpoDoen15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen15::R`](R) reader structure"]
impl crate::Readable for GpoDoen15Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen15::W`](W) writer structure"]
impl crate::Writable for GpoDoen15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen15 to value 0x2901_2828"]
impl crate::Resettable for GpoDoen15Spec {
    const RESET_VALUE: u32 = 0x2901_2828;
}
