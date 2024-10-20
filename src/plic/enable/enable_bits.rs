#[doc = "Register `enable_bits[%s]` reader"]
pub type R = crate::R<EnableBitsSpec>;
#[doc = "Register `enable_bits[%s]` writer"]
pub type W = crate::W<EnableBitsSpec>;
#[doc = "Field `enable` reader - "]
pub type EnableR = crate::FieldReader<u32>;
#[doc = "Field `enable` writer - "]
pub type EnableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<EnableBitsSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Interrupt source enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_bits::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_bits::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableBitsSpec;
impl crate::RegisterSpec for EnableBitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_bits::R`](R) reader structure"]
impl crate::Readable for EnableBitsSpec {}
#[doc = "`write(|w| ..)` method takes [`enable_bits::W`](W) writer structure"]
impl crate::Writable for EnableBitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets enable_bits[%s]
to value 0"]
impl crate::Resettable for EnableBitsSpec {
    const RESET_VALUE: u32 = 0;
}
