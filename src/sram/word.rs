#[doc = "Register `word[%s]` reader"]
pub type R = crate::R<WordSpec>;
#[doc = "Register `word[%s]` writer"]
pub type W = crate::W<WordSpec>;
#[doc = "Field `word` reader - SiFive U74(MC) SRAM (L2 LIM) word"]
pub type WordR = crate::FieldReader<u32>;
#[doc = "Field `word` writer - SiFive U74(MC) SRAM (L2 LIM) word"]
pub type WordW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SiFive U74(MC) SRAM (L2 LIM) word"]
    #[inline(always)]
    pub fn word(&self) -> WordR {
        WordR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SiFive U74(MC) SRAM (L2 LIM) word"]
    #[inline(always)]
    #[must_use]
    pub fn word(&mut self) -> WordW<WordSpec> {
        WordW::new(self, 0)
    }
}
#[doc = "SiFive U74(MC) SRAM (L2 LIM) word\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WordSpec;
impl crate::RegisterSpec for WordSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`word::R`](R) reader structure"]
impl crate::Readable for WordSpec {}
#[doc = "`write(|w| ..)` method takes [`word::W`](W) writer structure"]
impl crate::Writable for WordSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets word[%s]
to value 0"]
impl crate::Resettable for WordSpec {
    const RESET_VALUE: u32 = 0;
}
