#[doc = "Register `low` reader"]
pub type R = crate::R<LowSpec>;
#[doc = "Register `low` writer"]
pub type W = crate::W<LowSpec>;
#[doc = "Field `addr` reader - Hardware Address Low"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `addr` writer - Hardware Address Low"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hardware Address Low"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hardware Address Low"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<LowSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Hardware Address Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowSpec;
impl crate::RegisterSpec for LowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`low::R`](R) reader structure"]
impl crate::Readable for LowSpec {}
#[doc = "`write(|w| ..)` method takes [`low::W`](W) writer structure"]
impl crate::Writable for LowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets low to value 0"]
impl crate::Resettable for LowSpec {
    const RESET_VALUE: u32 = 0;
}
