#[doc = "Register `gpo_dout6` reader"]
pub type R = crate::R<GpoDout6Spec>;
#[doc = "Register `gpo_dout6` writer"]
pub type W = crate::W<GpoDout6Spec>;
#[doc = "Field `dout24` reader - The selected output signal for GPIO24. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout24R = crate::FieldReader;
#[doc = "Field `dout24` writer - The selected output signal for GPIO24. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout24W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout25` reader - The selected output signal for GPIO25. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout25R = crate::FieldReader;
#[doc = "Field `dout25` writer - The selected output signal for GPIO25. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout26` reader - The selected output signal for GPIO26. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout26R = crate::FieldReader;
#[doc = "Field `dout26` writer - The selected output signal for GPIO26. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout26W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout27` reader - The selected output signal for GPIO27. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout27R = crate::FieldReader;
#[doc = "Field `dout27` writer - The selected output signal for GPIO27. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout27W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO24. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout24(&self) -> Dout24R {
        Dout24R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO25. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout25(&self) -> Dout25R {
        Dout25R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO26. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout26(&self) -> Dout26R {
        Dout26R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO27. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout27(&self) -> Dout27R {
        Dout27R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO24. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout24(&mut self) -> Dout24W<GpoDout6Spec> {
        Dout24W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO25. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout25(&mut self) -> Dout25W<GpoDout6Spec> {
        Dout25W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO26. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout26(&mut self) -> Dout26W<GpoDout6Spec> {
        Dout26W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO27. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout27(&mut self) -> Dout27W<GpoDout6Spec> {
        Dout27W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout6Spec;
impl crate::RegisterSpec for GpoDout6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout6::R`](R) reader structure"]
impl crate::Readable for GpoDout6Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout6::W`](W) writer structure"]
impl crate::Writable for GpoDout6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout6 to value 0"]
impl crate::Resettable for GpoDout6Spec {
    const RESET_VALUE: u32 = 0;
}
