#[doc = "Register `gpo_dout9` reader"]
pub type R = crate::R<GpoDout9Spec>;
#[doc = "Register `gpo_dout9` writer"]
pub type W = crate::W<GpoDout9Spec>;
#[doc = "Field `dout36` reader - The selected output signal for GPIO36. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout36R = crate::FieldReader;
#[doc = "Field `dout36` writer - The selected output signal for GPIO36. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout36W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout37` reader - The selected output signal for GPIO37. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout37R = crate::FieldReader;
#[doc = "Field `dout37` writer - The selected output signal for GPIO37. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout37W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout38` reader - The selected output signal for GPIO38. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout38R = crate::FieldReader;
#[doc = "Field `dout38` writer - The selected output signal for GPIO38. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout38W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dout39` reader - The selected output signal for GPIO39. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout39R = crate::FieldReader;
#[doc = "Field `dout39` writer - The selected output signal for GPIO39. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
pub type Dout39W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - The selected output signal for GPIO36. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout36(&self) -> Dout36R {
        Dout36R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO37. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout37(&self) -> Dout37R {
        Dout37R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO38. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout38(&self) -> Dout38R {
        Dout38R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO39. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    pub fn dout39(&self) -> Dout39R {
        Dout39R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - The selected output signal for GPIO36. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout36(&mut self) -> Dout36W<GpoDout9Spec> {
        Dout36W::new(self, 0)
    }
    #[doc = "Bits 8:14 - The selected output signal for GPIO37. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout37(&mut self) -> Dout37W<GpoDout9Spec> {
        Dout37W::new(self, 8)
    }
    #[doc = "Bits 16:22 - The selected output signal for GPIO38. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout38(&mut self) -> Dout38W<GpoDout9Spec> {
        Dout38W::new(self, 16)
    }
    #[doc = "Bits 24:30 - The selected output signal for GPIO39. The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
    #[inline(always)]
    #[must_use]
    pub fn dout39(&mut self) -> Dout39W<GpoDout9Spec> {
        Dout39W::new(self, 24)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpoDout9Spec;
impl crate::RegisterSpec for GpoDout9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpo_dout9::R`](R) reader structure"]
impl crate::Readable for GpoDout9Spec {}
#[doc = "`write(|w| ..)` method takes [`gpo_dout9::W`](W) writer structure"]
impl crate::Writable for GpoDout9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpo_dout9 to value 0x5453_0f0e"]
impl crate::Resettable for GpoDout9Spec {
    const RESET_VALUE: u32 = 0x5453_0f0e;
}
