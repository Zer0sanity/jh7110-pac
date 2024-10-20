#[doc = "Register `csr[%s]` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `csr[%s]` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Field `csr` reader - "]
pub type CsrR = crate::FieldReader<u32>;
#[doc = "Field `csr` writer - "]
pub type CsrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CsrW<CsrSpec> {
        CsrW::new(self, 0)
    }
}
#[doc = "DDR Memory Control CSR register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets csr[%s]
to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}
