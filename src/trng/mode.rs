#[doc = "Register `mode` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `mode` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `r256` reader - 256-bit operation mode: 0 - 128-bit mode, 1 - 256-bit mode"]
pub type R256R = crate::BitReader;
#[doc = "Field `r256` writer - 256-bit operation mode: 0 - 128-bit mode, 1 - 256-bit mode"]
pub type R256W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 256-bit operation mode: 0 - 128-bit mode, 1 - 256-bit mode"]
    #[inline(always)]
    pub fn r256(&self) -> R256R {
        R256R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 256-bit operation mode: 0 - 128-bit mode, 1 - 256-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn r256(&mut self) -> R256W<ModeSpec> {
        R256W::new(self, 3)
    }
}
#[doc = "TRNG MODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mode to value 0"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0;
}
