#[doc = "Register `fmux_gpo_dout` reader"]
pub type R = crate::R<FmuxGpoDoutSpec>;
#[doc = "Register `fmux_gpo_dout` writer"]
pub type W = crate::W<FmuxGpoDoutSpec>;
#[doc = "Field `gpo_dout0` reader - The selected DOUT signal for GPIO0. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDout0R = crate::FieldReader;
#[doc = "Field `gpo_dout0` writer - The selected DOUT signal for GPIO0. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDout0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpo_dout1` reader - The selected DOUT signal for GPIO1. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDout1R = crate::FieldReader;
#[doc = "Field `gpo_dout1` writer - The selected DOUT signal for GPIO1. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDout1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpo_dout2` reader - The selected DOUT signal for GPIO2. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDout2R = crate::FieldReader;
#[doc = "Field `gpo_dout2` writer - The selected DOUT signal for GPIO2. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDout2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `gpo_dout3` reader - The selected DOUT signal for GPIO3. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDout3R = crate::FieldReader;
#[doc = "Field `gpo_dout3` writer - The selected DOUT signal for GPIO3. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
pub type GpoDout3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - The selected DOUT signal for GPIO0. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_dout0(&self) -> GpoDout0R {
        GpoDout0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - The selected DOUT signal for GPIO1. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_dout1(&self) -> GpoDout1R {
        GpoDout1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The selected DOUT signal for GPIO2. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_dout2(&self) -> GpoDout2R {
        GpoDout2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - The selected DOUT signal for GPIO3. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    pub fn gpo_dout3(&self) -> GpoDout3R {
        GpoDout3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The selected DOUT signal for GPIO0. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_dout0(&mut self) -> GpoDout0W<FmuxGpoDoutSpec> {
        GpoDout0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - The selected DOUT signal for GPIO1. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_dout1(&mut self) -> GpoDout1W<FmuxGpoDoutSpec> {
        GpoDout1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - The selected DOUT signal for GPIO2. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_dout2(&mut self) -> GpoDout2W<FmuxGpoDoutSpec> {
        GpoDout2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - The selected DOUT signal for GPIO3. The register value indicates the selected GPIO output signal list 0-9. See Table 2-42: GPIO OEN List for AON_IOMUX for more information."]
    #[inline(always)]
    #[must_use]
    pub fn gpo_dout3(&mut self) -> GpoDout3W<FmuxGpoDoutSpec> {
        GpoDout3W::new(self, 24)
    }
}
#[doc = "The register can be used to configure the selected (Digital Output) DOUT signal for GPIO0 - GPIO3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_gpo_dout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_gpo_dout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmuxGpoDoutSpec;
impl crate::RegisterSpec for FmuxGpoDoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmux_gpo_dout::R`](R) reader structure"]
impl crate::Readable for FmuxGpoDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`fmux_gpo_dout::W`](W) writer structure"]
impl crate::Writable for FmuxGpoDoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fmux_gpo_dout to value 0"]
impl crate::Resettable for FmuxGpoDoutSpec {
    const RESET_VALUE: u32 = 0;
}
