#[doc = "Register `gpo_doen11` reader"]
pub type R = crate::R<GpoDoen11Spec>;
#[doc = "Register `gpo_doen11` writer"]
pub type W = crate::W<GpoDoen11Spec>;
#[doc = "Field `doen44` reader - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen44R = crate::FieldReader;
#[doc = "Field `doen44` writer - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen44W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen45` reader - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen45R = crate::FieldReader;
#[doc = "Field `doen45` writer - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen45W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen46` reader - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen46R = crate::FieldReader;
#[doc = "Field `doen46` writer - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen46W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen47` reader - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen47R = crate::FieldReader;
#[doc = "Field `doen47` writer - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen47W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen44(&self) -> Doen44R {
        Doen44R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen45(&self) -> Doen45R {
        Doen45R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen46(&self) -> Doen46R {
        Doen46R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen47(&self) -> Doen47R {
        Doen47R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO44. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen44(&mut self) -> Doen44W<GpoDoen11Spec> {
        Doen44W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO45. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen45(&mut self) -> Doen45W<GpoDoen11Spec> {
        Doen45W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO46. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen46(&mut self) -> Doen46W<GpoDoen11Spec> {
        Doen46W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO47. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen47(&mut self) -> Doen47W<GpoDoen11Spec> {
        Doen47W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen11Spec;
impl crate::RegisterSpec for GpoDoen11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen11::R`](R) reader structure"]
impl crate::Readable for GpoDoen11Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen11::W`](W) writer structure"]
impl crate::Writable for GpoDoen11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen11 to value 0x0100_0001"]
impl crate::Resettable for GpoDoen11Spec {
    const RESET_VALUE: u32 = 0x0100_0001;
}
