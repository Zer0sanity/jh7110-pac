#[doc = "Register `wlen[%s]` reader"]
pub type R = crate::R<WlenSpec>;
#[doc = "Register `wlen[%s]` writer"]
pub type W = crate::W<WlenSpec>;
#[doc = "Field `wlen` reader - SHA WLEN"]
pub type WlenR = crate::FieldReader<u32>;
#[doc = "Field `wlen` writer - SHA WLEN"]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA WLEN"]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA WLEN"]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WlenW<WlenSpec> {
        WlenW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto SHA WLEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WlenSpec;
impl crate::RegisterSpec for WlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wlen::R`](R) reader structure"]
impl crate::Readable for WlenSpec {}
#[doc = "`write(|w| ..)` method takes [`wlen::W`](W) writer structure"]
impl crate::Writable for WlenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wlen[%s]
to value 0"]
impl crate::Resettable for WlenSpec {
    const RESET_VALUE: u32 = 0;
}
