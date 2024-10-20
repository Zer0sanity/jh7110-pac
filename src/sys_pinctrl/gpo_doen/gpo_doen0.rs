#[doc = "Register `gpo_doen0` reader"]
pub type R = crate::R<GpoDoen0Spec>;
#[doc = "Register `gpo_doen0` writer"]
pub type W = crate::W<GpoDoen0Spec>;
#[doc = "Field `doen0` reader - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen0R = crate::FieldReader;
#[doc = "Field `doen0` writer - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen1` reader - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen1R = crate::FieldReader;
#[doc = "Field `doen1` writer - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen2` reader - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen2R = crate::FieldReader;
#[doc = "Field `doen2` writer - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `doen3` reader - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen3R = crate::FieldReader;
#[doc = "Field `doen3` writer - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
pub type Doen3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen0(&self) -> Doen0R {
        Doen0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen1(&self) -> Doen1R {
        Doen1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen2(&self) -> Doen2R {
        Doen2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn doen3(&self) -> Doen3R {
        Doen3R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen0(&mut self) -> Doen0W<GpoDoen0Spec> {
        Doen0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen1(&mut self) -> Doen1W<GpoDoen0Spec> {
        Doen1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen2(&mut self) -> Doen2W<GpoDoen0Spec> {
        Doen2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn doen3(&mut self) -> Doen3W<GpoDoen0Spec> {
        Doen3W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDoen0Spec;
impl crate::RegisterSpec for GpoDoen0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_doen0::R`](R) reader structure"]
impl crate::Readable for GpoDoen0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_doen0::W`](W) writer structure"]
impl crate::Writable for GpoDoen0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_doen0 to value 0x0801_0101"]
impl crate::Resettable for GpoDoen0Spec {
    const RESET_VALUE: u32 = 0x0801_0101;
}
