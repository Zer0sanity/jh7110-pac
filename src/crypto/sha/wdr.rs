#[doc = "Register `wdr` reader"]
pub type R = crate::R<WdrSpec>;
#[doc = "Register `wdr` writer"]
pub type W = crate::W<WdrSpec>;
#[doc = "Field `wdr` reader - SHA WDR"]
pub type WdrR = crate::FieldReader<u32>;
#[doc = "Field `wdr` writer - SHA WDR"]
pub type WdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA WDR"]
    #[inline(always)]
    pub fn wdr(&self) -> WdrR {
        WdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA WDR"]
    #[inline(always)]
    #[must_use]
    pub fn wdr(&mut self) -> WdrW<WdrSpec> {
        WdrW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto SHA WDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrSpec;
impl crate::RegisterSpec for WdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr::R`](R) reader structure"]
impl crate::Readable for WdrSpec {}
#[doc = "`write(|w| ..)` method takes [`wdr::W`](W) writer structure"]
impl crate::Writable for WdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wdr to value 0"]
impl crate::Resettable for WdrSpec {
    const RESET_VALUE: u32 = 0;
}
