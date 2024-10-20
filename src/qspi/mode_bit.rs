#[doc = "Register `mode_bit` reader"]
pub type R = crate::R<ModeBitSpec>;
#[doc = "Register `mode_bit` writer"]
pub type W = crate::W<ModeBitSpec>;
#[doc = "Field `mode` reader - "]
pub type ModeR = crate::FieldReader<u32>;
#[doc = "Field `mode` writer - "]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<ModeBitSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Mode Bit(s)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_bit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_bit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeBitSpec;
impl crate::RegisterSpec for ModeBitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_bit::R`](R) reader structure"]
impl crate::Readable for ModeBitSpec {}
#[doc = "`write(|w| ..)` method takes [`mode_bit::W`](W) writer structure"]
impl crate::Writable for ModeBitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mode_bit to value 0"]
impl crate::Resettable for ModeBitSpec {
    const RESET_VALUE: u32 = 0;
}
