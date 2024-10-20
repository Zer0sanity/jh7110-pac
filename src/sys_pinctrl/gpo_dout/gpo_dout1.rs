#[doc = "Register `gpo_dout1` reader"]
pub type R = crate::R<GpoDout1Spec>;
#[doc = "Register `gpo_dout1` writer"]
pub type W = crate::W<GpoDout1Spec>;
#[doc = "Field `dout4` reader - The selected output signal for GPIO4. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout4R = crate::FieldReader;
#[doc = "Field `dout4` writer - The selected output signal for GPIO4. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout5` reader - The selected output signal for GPIO5. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout5R = crate::FieldReader;
#[doc = "Field `dout5` writer - The selected output signal for GPIO5. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout6` reader - The selected output signal for GPIO6. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout6R = crate::FieldReader;
#[doc = "Field `dout6` writer - The selected output signal for GPIO6. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout7` reader - The selected output signal for GPIO7. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout7R = crate::FieldReader;
#[doc = "Field `dout7` writer - The selected output signal for GPIO7. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO4. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout4(&self) -> Dout4R {
        Dout4R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO5. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout5(&self) -> Dout5R {
        Dout5R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO6. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout6(&self) -> Dout6R {
        Dout6R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO7. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout7(&self) -> Dout7R {
        Dout7R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO4. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout4(&mut self) -> Dout4W<GpoDout1Spec> {
        Dout4W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO5. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout5(&mut self) -> Dout5W<GpoDout1Spec> {
        Dout5W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO6. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout6(&mut self) -> Dout6W<GpoDout1Spec> {
        Dout6W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO7. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout7(&mut self) -> Dout7W<GpoDout1Spec> {
        Dout7W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout1Spec;
impl crate::RegisterSpec for GpoDout1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout1::R`](R) reader structure"]
impl crate::Readable for GpoDout1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout1::W`](W) writer structure"]
impl crate::Writable for GpoDout1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout1 to value 0x1400"]
impl crate::Resettable for GpoDout1Spec {
    const RESET_VALUE: u32 = 0x1400;
}
