#[doc = "Register `gpo_doen12` reader"]
pub type R = crate::R<GpoDoen12Spec>;
#[doc = "Register `gpo_doen12` writer"]
pub type W = crate::W<GpoDoen12Spec>;
#[doc = "Field `doen48` reader - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen48R = crate::FieldReader;
#[doc = "Field `doen48` writer - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen48W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen49` reader - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen49R = crate::FieldReader;
#[doc = "Field `doen49` writer - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen49W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen50` reader - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen50R = crate::FieldReader;
#[doc = "Field `doen50` writer - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen50W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen51` reader - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen51R = crate::FieldReader;
#[doc = "Field `doen51` writer - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen51W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen48(&self) -> Doen48R {
        Doen48R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen49(&self) -> Doen49R {
        Doen49R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen50(&self) -> Doen50R {
        Doen50R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen51(&self) -> Doen51R {
        Doen51R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO48. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen48(&mut self) -> Doen48W<GpoDoen12Spec> {
        Doen48W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO49. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen49(&mut self) -> Doen49W<GpoDoen12Spec> {
        Doen49W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO50. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen50(&mut self) -> Doen50W<GpoDoen12Spec> {
        Doen50W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO51. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen51(&mut self) -> Doen51W<GpoDoen12Spec> {
        Doen51W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen12Spec;
impl crate::RegisterSpec for GpoDoen12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen12::R`](R) reader structure"]
impl crate::Readable for GpoDoen12Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen12::W`](W) writer structure"]
impl crate::Writable for GpoDoen12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen12 to value 0x0e01_0d0d"]
impl crate::Resettable for GpoDoen12Spec {
    const RESET_VALUE: u32 = 0x0e01_0d0d;
}
