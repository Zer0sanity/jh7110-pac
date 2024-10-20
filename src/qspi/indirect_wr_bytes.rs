#[doc = "Register `indirect_wr_bytes` reader"]
pub type R = crate::R<IndirectWrBytesSpec>;
#[doc = "Register `indirect_wr_bytes` writer"]
pub type W = crate::W<IndirectWrBytesSpec>;
#[doc = "Field `bytes` reader - "]
pub type BytesR = crate::FieldReader<u32>;
#[doc = "Field `bytes` writer - "]
pub type BytesW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bytes(&self) -> BytesR {
        BytesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bytes(&mut self) -> BytesW<IndirectWrBytesSpec> {
        BytesW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Indirect Write Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_bytes::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_bytes::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectWrBytesSpec;
impl crate::RegisterSpec for IndirectWrBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_wr_bytes::R`](R) reader structure"]
impl crate::Readable for IndirectWrBytesSpec {}
#[doc = "`write(|w| ..)` method takes [`indirect_wr_bytes::W`](W) writer structure"]
impl crate::Writable for IndirectWrBytesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets indirect_wr_bytes to value 0"]
impl crate::Resettable for IndirectWrBytesSpec {
    const RESET_VALUE: u32 = 0;
}
