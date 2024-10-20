#[doc = "Register `gpo_doen14` reader"]
pub type R = crate::R<GpoDoen14Spec>;
#[doc = "Register `gpo_doen14` writer"]
pub type W = crate::W<GpoDoen14Spec>;
#[doc = "Field `doen56` reader - The selected OEN signal for GPIO56. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen56R = crate::FieldReader;
#[doc = "Field `doen56` writer - The selected OEN signal for GPIO56. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen56W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen57` reader - The selected OEN signal for GPIO57. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen57R = crate::FieldReader;
#[doc = "Field `doen57` writer - The selected OEN signal for GPIO57. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen57W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen58` reader - The selected OEN signal for GPIO58. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen58R = crate::FieldReader;
#[doc = "Field `doen58` writer - The selected OEN signal for GPIO58. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen58W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen59` reader - The selected OEN signal for GPIO59. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen59R = crate::FieldReader;
#[doc = "Field `doen59` writer - The selected OEN signal for GPIO59. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen59W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO56. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen56(&self) -> Doen56R {
        Doen56R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO57. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen57(&self) -> Doen57R {
        Doen57R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO58. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen58(&self) -> Doen58R {
        Doen58R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO59. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen59(&self) -> Doen59R {
        Doen59R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO56. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen56(&mut self) -> Doen56W<GpoDoen14Spec> {
        Doen56W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO57. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen57(&mut self) -> Doen57W<GpoDoen14Spec> {
        Doen57W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO58. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen58(&mut self) -> Doen58W<GpoDoen14Spec> {
        Doen58W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO59. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen59(&mut self) -> Doen59W<GpoDoen14Spec> {
        Doen59W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen14Spec;
impl crate::RegisterSpec for GpoDoen14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen14::R`](R) reader structure"]
impl crate::Readable for GpoDoen14Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen14::W`](W) writer structure"]
impl crate::Writable for GpoDoen14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen14 to value 0x2501_2424"]
impl crate::Resettable for GpoDoen14Spec {
    const RESET_VALUE: u32 = 0x2501_2424;
}
