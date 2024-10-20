#[doc = "Register `gpo_dout2` reader"]
pub type R = crate::R<GpoDout2Spec>;
#[doc = "Register `gpo_dout2` writer"]
pub type W = crate::W<GpoDout2Spec>;
#[doc = "Field `dout8` reader - The selected output signal for GPIO8. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout8R = crate::FieldReader;
#[doc = "Field `dout8` writer - The selected output signal for GPIO8. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout8W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout9` reader - The selected output signal for GPIO9. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout9R = crate::FieldReader;
#[doc = "Field `dout9` writer - The selected output signal for GPIO9. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout10` reader - The selected output signal for GPIO10. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout10R = crate::FieldReader;
#[doc = "Field `dout10` writer - The selected output signal for GPIO10. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout10W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout11` reader - The selected output signal for GPIO11. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout11R = crate::FieldReader;
#[doc = "Field `dout11` writer - The selected output signal for GPIO11. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout11W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO8. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout8(&self) -> Dout8R {
        Dout8R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO9. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout9(&self) -> Dout9R {
        Dout9R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO10. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout10(&self) -> Dout10R {
        Dout10R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO11. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout11(&self) -> Dout11R {
        Dout11R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO8. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout8(&mut self) -> Dout8W<GpoDout2Spec> {
        Dout8W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO9. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout9(&mut self) -> Dout9W<GpoDout2Spec> {
        Dout9W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO10. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout10(&mut self) -> Dout10W<GpoDout2Spec> {
        Dout10W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO11. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout11(&mut self) -> Dout11W<GpoDout2Spec> {
        Dout11W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout2Spec;
impl crate::RegisterSpec for GpoDout2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout2::R`](R) reader structure"]
impl crate::Readable for GpoDout2Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout2::W`](W) writer structure"]
impl crate::Writable for GpoDout2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout2 to value 0x1500_0000"]
impl crate::Resettable for GpoDout2Spec {
    const RESET_VALUE: u32 = 0x1500_0000;
}
