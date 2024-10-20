#[doc = "Register `fmux_gpo_doen` reader"]
pub type R = crate::R<FmuxGpoDoenSpec>;
#[doc = "Register `fmux_gpo_doen` writer"]
pub type W = crate::W<FmuxGpoDoenSpec>;
#[doc = "Field `gpo_doen0` reader - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDoen0R = crate::FieldReader;
#[doc = "Field `gpo_doen0` writer - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDoen0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpo_doen1` reader - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDoen1R = crate::FieldReader;
#[doc = "Field `gpo_doen1` writer - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDoen1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpo_doen2` reader - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDoen2R = crate::FieldReader;
#[doc = "Field `gpo_doen2` writer - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDoen2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gpo_doen3` reader - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDoen3R = crate::FieldReader;
#[doc = "Field `gpo_doen3` writer - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDoen3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_doen0(&self) -> GpoDoen0R {
        GpoDoen0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_doen1(&self) -> GpoDoen1R {
        GpoDoen1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_doen2(&self) -> GpoDoen2R {
        GpoDoen2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_doen3(&self) -> GpoDoen3R {
        GpoDoen3R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The selected OEN signal for GPIO0. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_doen0(&mut self) -> GpoDoen0W<FmuxGpoDoenSpec> {
        GpoDoen0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - The selected OEN signal for GPIO1. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_doen1(&mut self) -> GpoDoen1W<FmuxGpoDoenSpec> {
        GpoDoen1W::new(self, 8)
    }
    #[doc = "Bits 16:18 - The selected OEN signal for GPIO2. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_doen2(&mut self) -> GpoDoen2W<FmuxGpoDoenSpec> {
        GpoDoen2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - The selected OEN signal for GPIO3. The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-5. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_doen3(&mut self) -> GpoDoen3W<FmuxGpoDoenSpec> {
        GpoDoen3W::new(self, 24)
    }
}
#[doc = "The register can be used to configure the selected (Output Enable) OEN signal for GPIO0 - GPIO3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_gpo_doen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_gpo_doen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmuxGpoDoenSpec;
impl crate::RegisterSpec for FmuxGpoDoenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmux_gpo_doen::R`](R) reader structure"]
impl crate::Readable for FmuxGpoDoenSpec {}
#[doc = "`write(|w| ..)` method takes [`fmux_gpo_doen::W`](W) writer structure"]
impl crate::Writable for FmuxGpoDoenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fmux_gpo_doen to value 0x0101_0101"]
impl crate::Resettable for FmuxGpoDoenSpec {
    const RESET_VALUE: u32 = 0x0101_0101;
}
