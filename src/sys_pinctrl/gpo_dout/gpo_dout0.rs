#[doc = "Register `gpo_dout0` reader"]
pub type R = crate::R<GpoDout0Spec>;
#[doc = "Register `gpo_dout0` writer"]
pub type W = crate::W<GpoDout0Spec>;
#[doc = "Field `dout0` reader - The selected output signal for GPIO0. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout0R = crate::FieldReader;
#[doc = "Field `dout0` writer - The selected output signal for GPIO0. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout1` reader - The selected output signal for GPIO1. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout1R = crate::FieldReader;
#[doc = "Field `dout1` writer - The selected output signal for GPIO1. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout2` reader - The selected output signal for GPIO2. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout2R = crate::FieldReader;
#[doc = "Field `dout2` writer - The selected output signal for GPIO2. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout3` reader - The selected output signal for GPIO3. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout3R = crate::FieldReader;
#[doc = "Field `dout3` writer - The selected output signal for GPIO3. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO0. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout0(&self) -> Dout0R {
        Dout0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO1. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout1(&self) -> Dout1R {
        Dout1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO2. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout2(&self) -> Dout2R {
        Dout2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO3. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout3(&self) -> Dout3R {
        Dout3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO0. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout0(&mut self) -> Dout0W<GpoDout0Spec> {
        Dout0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO1. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout1(&mut self) -> Dout1W<GpoDout0Spec> {
        Dout1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO2. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout2(&mut self) -> Dout2W<GpoDout0Spec> {
        Dout2W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO3. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout3(&mut self) -> Dout3W<GpoDout0Spec> {
        Dout3W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout0Spec;
impl crate::RegisterSpec for GpoDout0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout0::R`](R) reader structure"]
impl crate::Readable for GpoDout0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout0::W`](W) writer structure"]
impl crate::Writable for GpoDout0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout0 to value 0x1600_0000"]
impl crate::Resettable for GpoDout0Spec {
    const RESET_VALUE: u32 = 0x1600_0000;
}
