#[doc = "Register `pending[%s]` reader"]
pub type R = crate::R<PendingSpec>;
#[doc = "Register `pending[%s]` writer"]
pub type W = crate::W<PendingSpec>;
#[doc = "Field `pending` reader - "]
pub type PendingR = crate::FieldReader<u32>;
#[doc = "Field `pending` writer - "]
pub type PendingW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn pending(&mut self) -> PendingW<PendingSpec> {
        PendingW::new(self, 0)
    }
}
#[doc = "RISC-V PLIC Pending: 32-bit register indicating if there is a pending interrupt. The bit index indicates the interrupt source, e.g. pending\\[0\\]\\[31\\]
is interrupt 31, pending\\[1\\]\\[0\\]
is interrupt 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PendingSpec;
impl crate::RegisterSpec for PendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending::R`](R) reader structure"]
impl crate::Readable for PendingSpec {}
#[doc = "`write(|w| ..)` method takes [`pending::W`](W) writer structure"]
impl crate::Writable for PendingSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pending[%s]
to value 0"]
impl crate::Resettable for PendingSpec {
    const RESET_VALUE: u32 = 0;
}
