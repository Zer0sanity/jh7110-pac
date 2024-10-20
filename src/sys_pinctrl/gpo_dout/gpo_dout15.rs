#[doc = "Register `gpo_dout15` reader"]
pub type R = crate::R<GpoDout15Spec>;
#[doc = "Register `gpo_dout15` writer"]
pub type W = crate::W<GpoDout15Spec>;
#[doc = "Field `dout60` reader - The selected output signal for GPIO60. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout60R = crate::FieldReader;
#[doc = "Field `dout60` writer - The selected output signal for GPIO60. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout60W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout61` reader - The selected output signal for GPIO61. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout61R = crate::FieldReader;
#[doc = "Field `dout61` writer - The selected output signal for GPIO61. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout61W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout62` reader - The selected output signal for GPIO62. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout62R = crate::FieldReader;
#[doc = "Field `dout62` writer - The selected output signal for GPIO62. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout62W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout63` reader - The selected output signal for GPIO63. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout63R = crate::FieldReader;
#[doc = "Field `dout63` writer - The selected output signal for GPIO63. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout63W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO60. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout60(&self) -> Dout60R {
        Dout60R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO61. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout61(&self) -> Dout61R {
        Dout61R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO62. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout62(&self) -> Dout62R {
        Dout62R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO63. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout63(&self) -> Dout63R {
        Dout63R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO60. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout60(&mut self) -> Dout60W<GpoDout15Spec> {
        Dout60W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO61. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout61(&mut self) -> Dout61W<GpoDout15Spec> {
        Dout61W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO62. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout62(&mut self) -> Dout62W<GpoDout15Spec> {
        Dout62W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO63. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout63(&mut self) -> Dout63W<GpoDout15Spec> {
        Dout63W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout15Spec;
impl crate::RegisterSpec for GpoDout15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout15::R`](R) reader structure"]
impl crate::Readable for GpoDout15Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout15::W`](W) writer structure"]
impl crate::Writable for GpoDout15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout15 to value 0x5f00_5d5e"]
impl crate::Resettable for GpoDout15Spec {
    const RESET_VALUE: u32 = 0x5f00_5d5e;
}
