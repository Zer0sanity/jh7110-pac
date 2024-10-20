#[doc = "Register `tdm_fifo[%s]` reader"]
pub type R = crate::R<TdmFifoSpec>;
#[doc = "Register `tdm_fifo[%s]` writer"]
pub type W = crate::W<TdmFifoSpec>;
#[doc = "Field `fifo` reader - TDM FIFO"]
pub type FifoR = crate::FieldReader<u32>;
#[doc = "Field `fifo` writer - TDM FIFO"]
pub type FifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TDM FIFO"]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TDM FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn fifo(&mut self) -> FifoW<TdmFifoSpec> {
        FifoW::new(self, 0)
    }
}
#[doc = "TDM FIFO registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdm_fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdm_fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdmFifoSpec;
impl crate::RegisterSpec for TdmFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdm_fifo::R`](R) reader structure"]
impl crate::Readable for TdmFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`tdm_fifo::W`](W) writer structure"]
impl crate::Writable for TdmFifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tdm_fifo[%s]
to value 0"]
impl crate::Resettable for TdmFifoSpec {
    const RESET_VALUE: u32 = 0;
}
