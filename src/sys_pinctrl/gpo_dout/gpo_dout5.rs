#[doc = "Register `gpo_dout5` reader"]
pub type R = crate::R<GpoDout5Spec>;
#[doc = "Register `gpo_dout5` writer"]
pub type W = crate::W<GpoDout5Spec>;
#[doc = "Field `dout20` reader - The selected output signal for GPIO20. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout20R = crate::FieldReader;
#[doc = "Field `dout20` writer - The selected output signal for GPIO20. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout20W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout21` reader - The selected output signal for GPIO21. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout21R = crate::FieldReader;
#[doc = "Field `dout21` writer - The selected output signal for GPIO21. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout21W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout22` reader - The selected output signal for GPIO22. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout22R = crate::FieldReader;
#[doc = "Field `dout22` writer - The selected output signal for GPIO22. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout22W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout23` reader - The selected output signal for GPIO23. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout23R = crate::FieldReader;
#[doc = "Field `dout23` writer - The selected output signal for GPIO23. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout23W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO20. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout20(&self) -> Dout20R {
        Dout20R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO21. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout21(&self) -> Dout21R {
        Dout21R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO22. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout22(&self) -> Dout22R {
        Dout22R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO23. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout23(&self) -> Dout23R {
        Dout23R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO20. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout20(&mut self) -> Dout20W<GpoDout5Spec> {
        Dout20W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO21. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout21(&mut self) -> Dout21W<GpoDout5Spec> {
        Dout21W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO22. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout22(&mut self) -> Dout22W<GpoDout5Spec> {
        Dout22W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO23. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout23(&mut self) -> Dout23W<GpoDout5Spec> {
        Dout23W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout5Spec;
impl crate::RegisterSpec for GpoDout5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout5::R`](R) reader structure"]
impl crate::Readable for GpoDout5Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout5::W`](W) writer structure"]
impl crate::Writable for GpoDout5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout5 to value 0x0055_0000"]
impl crate::Resettable for GpoDout5Spec {
    const RESET_VALUE: u32 = 0x0055_0000;
}
