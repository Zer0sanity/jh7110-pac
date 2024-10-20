#[doc = "Register `gpo_dout12` reader"]
pub type R = crate::R<GpoDout12Spec>;
#[doc = "Register `gpo_dout12` writer"]
pub type W = crate::W<GpoDout12Spec>;
#[doc = "Field `dout48` reader - The selected output signal for GPIO48. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout48R = crate::FieldReader;
#[doc = "Field `dout48` writer - The selected output signal for GPIO48. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout48W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout49` reader - The selected output signal for GPIO49. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout49R = crate::FieldReader;
#[doc = "Field `dout49` writer - The selected output signal for GPIO49. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout49W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout50` reader - The selected output signal for GPIO50. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout50R = crate::FieldReader;
#[doc = "Field `dout50` writer - The selected output signal for GPIO50. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout50W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout51` reader - The selected output signal for GPIO51. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout51R = crate::FieldReader;
#[doc = "Field `dout51` writer - The selected output signal for GPIO51. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout51W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO48. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout48(&self) -> Dout48R {
        Dout48R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO49. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout49(&self) -> Dout49R {
        Dout49R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO50. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout50(&self) -> Dout50R {
        Dout50R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO51. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout51(&self) -> Dout51R {
        Dout51R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO48. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout48(&mut self) -> Dout48W<GpoDout12Spec> {
        Dout48W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO49. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout49(&mut self) -> Dout49W<GpoDout12Spec> {
        Dout49W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO50. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout50(&mut self) -> Dout50W<GpoDout12Spec> {
        Dout50W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO51. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout51(&mut self) -> Dout51W<GpoDout12Spec> {
        Dout51W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout12Spec;
impl crate::RegisterSpec for GpoDout12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout12::R`](R) reader structure"]
impl crate::Readable for GpoDout12Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout12::W`](W) writer structure"]
impl crate::Writable for GpoDout12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout12 to value 0x2000_1e1f"]
impl crate::Resettable for GpoDout12Spec {
    const RESET_VALUE: u32 = 0x2000_1e1f;
}
