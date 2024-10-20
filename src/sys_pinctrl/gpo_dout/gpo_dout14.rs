#[doc = "Register `gpo_dout14` reader"]
pub type R = crate::R<GpoDout14Spec>;
#[doc = "Register `gpo_dout14` writer"]
pub type W = crate::W<GpoDout14Spec>;
#[doc = "Field `dout56` reader - The selected output signal for GPIO56. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout56R = crate::FieldReader;
#[doc = "Field `dout56` writer - The selected output signal for GPIO56. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout56W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout57` reader - The selected output signal for GPIO57. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout57R = crate::FieldReader;
#[doc = "Field `dout57` writer - The selected output signal for GPIO57. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout57W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout58` reader - The selected output signal for GPIO58. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout58R = crate::FieldReader;
#[doc = "Field `dout58` writer - The selected output signal for GPIO58. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout58W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout59` reader - The selected output signal for GPIO59. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout59R = crate::FieldReader;
#[doc = "Field `dout59` writer - The selected output signal for GPIO59. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout59W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO56. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout56(&self) -> Dout56R {
        Dout56R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO57. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout57(&self) -> Dout57R {
        Dout57R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO58. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout58(&self) -> Dout58R {
        Dout58R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO59. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout59(&self) -> Dout59R {
        Dout59R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO56. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout56(&mut self) -> Dout56W<GpoDout14Spec> {
        Dout56W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO57. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout57(&mut self) -> Dout57W<GpoDout14Spec> {
        Dout57W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO58. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout58(&mut self) -> Dout58W<GpoDout14Spec> {
        Dout58W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO59. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout59(&mut self) -> Dout59W<GpoDout14Spec> {
        Dout59W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout14Spec;
impl crate::RegisterSpec for GpoDout14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout14::R`](R) reader structure"]
impl crate::Readable for GpoDout14Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout14::W`](W) writer structure"]
impl crate::Writable for GpoDout14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout14 to value 0x5800_5657"]
impl crate::Resettable for GpoDout14Spec {
    const RESET_VALUE: u32 = 0x5800_5657;
}
