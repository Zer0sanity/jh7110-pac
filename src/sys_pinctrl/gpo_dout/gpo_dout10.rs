#[doc = "Register `gpo_dout10` reader"]
pub type R = crate::R<GpoDout10Spec>;
#[doc = "Register `gpo_dout10` writer"]
pub type W = crate::W<GpoDout10Spec>;
#[doc = "Field `dout40` reader - The selected output signal for GPIO40. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout40R = crate::FieldReader;
#[doc = "Field `dout40` writer - The selected output signal for GPIO40. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout40W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout41` reader - The selected output signal for GPIO41. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout41R = crate::FieldReader;
#[doc = "Field `dout41` writer - The selected output signal for GPIO41. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout41W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout42` reader - The selected output signal for GPIO42. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout42R = crate::FieldReader;
#[doc = "Field `dout42` writer - The selected output signal for GPIO42. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout42W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout43` reader - The selected output signal for GPIO43. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout43R = crate::FieldReader;
#[doc = "Field `dout43` writer - The selected output signal for GPIO43. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout43W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO40. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout40(&self) -> Dout40R {
        Dout40R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO41. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout41(&self) -> Dout41R {
        Dout41R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO42. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout42(&self) -> Dout42R {
        Dout42R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO43. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout43(&self) -> Dout43R {
        Dout43R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO40. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout40(&mut self) -> Dout40W<GpoDout10Spec> {
        Dout40W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO41. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout41(&mut self) -> Dout41W<GpoDout10Spec> {
        Dout41W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO42. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout42(&mut self) -> Dout42W<GpoDout10Spec> {
        Dout42W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO43. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout43(&mut self) -> Dout43W<GpoDout10Spec> {
        Dout43W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout10Spec;
impl crate::RegisterSpec for GpoDout10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout10::R`](R) reader structure"]
impl crate::Readable for GpoDout10Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout10::W`](W) writer structure"]
impl crate::Writable for GpoDout10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout10 to value 0x004e_4f00"]
impl crate::Resettable for GpoDout10Spec {
    const RESET_VALUE: u32 = 0x004e_4f00;
}
