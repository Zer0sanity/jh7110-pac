#[doc = "Register `gpo_dout8` reader"]
pub type R = crate::R<GpoDout8Spec>;
#[doc = "Register `gpo_dout8` writer"]
pub type W = crate::W<GpoDout8Spec>;
#[doc = "Field `dout32` reader - The selected output signal for GPIO32. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout32R = crate::FieldReader;
#[doc = "Field `dout32` writer - The selected output signal for GPIO32. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout32W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout33` reader - The selected output signal for GPIO33. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout33R = crate::FieldReader;
#[doc = "Field `dout33` writer - The selected output signal for GPIO33. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout33W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout34` reader - The selected output signal for GPIO34. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout34R = crate::FieldReader;
#[doc = "Field `dout34` writer - The selected output signal for GPIO34. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout34W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout35` reader - The selected output signal for GPIO35. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout35R = crate::FieldReader;
#[doc = "Field `dout35` writer - The selected output signal for GPIO35. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout35W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO32. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout32(&self) -> Dout32R {
        Dout32R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO33. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout33(&self) -> Dout33R {
        Dout33R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO34. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout34(&self) -> Dout34R {
        Dout34R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO35. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout35(&self) -> Dout35R {
        Dout35R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO32. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout32(&mut self) -> Dout32W<GpoDout8Spec> {
        Dout32W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO33. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout33(&mut self) -> Dout33W<GpoDout8Spec> {
        Dout33W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO34. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout34(&mut self) -> Dout34W<GpoDout8Spec> {
        Dout34W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO35. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout35(&mut self) -> Dout35W<GpoDout8Spec> {
        Dout35W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout8Spec;
impl crate::RegisterSpec for GpoDout8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout8::R`](R) reader structure"]
impl crate::Readable for GpoDout8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout8::W`](W) writer structure"]
impl crate::Writable for GpoDout8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout8 to value 0x0d00_0000"]
impl crate::Resettable for GpoDout8Spec {
    const RESET_VALUE: u32 = 0x0d00_0000;
}
