#[doc = "Register `caar` reader"]
pub type R = crate::R<CaarSpec>;
#[doc = "Register `caar` writer"]
pub type W = crate::W<CaarSpec>;
#[doc = "Field `caar` reader - Crypto CAAR"]
pub type CaarR = crate::FieldReader<u32>;
#[doc = "Field `caar` writer - Crypto CAAR"]
pub type CaarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Crypto CAAR"]
    #[inline(always)]
    pub fn caar(&self) -> CaarR {
        CaarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Crypto CAAR"]
    #[inline(always)]
    #[must_use]
    pub fn caar(&mut self) -> CaarW<CaarSpec> {
        CaarW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto CAAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaarSpec;
impl crate::RegisterSpec for CaarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caar::R`](R) reader structure"]
impl crate::Readable for CaarSpec {}
#[doc = "`write(|w| ..)` method takes [`caar::W`](W) writer structure"]
impl crate::Writable for CaarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets caar to value 0"]
impl crate::Resettable for CaarSpec {
    const RESET_VALUE: u32 = 0;
}
