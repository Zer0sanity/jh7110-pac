#[doc = "Register `indirect_wr_watermark` reader"]
pub type R = crate::R<IndirectWrWatermarkSpec>;
#[doc = "Register `indirect_wr_watermark` writer"]
pub type W = crate::W<IndirectWrWatermarkSpec>;
#[doc = "Field `watermark` reader - "]
pub type WatermarkR = crate::FieldReader<u32>;
#[doc = "Field `watermark` writer - "]
pub type WatermarkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn watermark(&self) -> WatermarkR {
        WatermarkR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn watermark(&mut self) -> WatermarkW<IndirectWrWatermarkSpec> {
        WatermarkW::new(self, 0)
    }
}
#[doc = "Cadence QSPI Indirect Write Watermark\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_watermark::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_watermark::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectWrWatermarkSpec;
impl crate::RegisterSpec for IndirectWrWatermarkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_wr_watermark::R`](R) reader structure"]
impl crate::Readable for IndirectWrWatermarkSpec {}
#[doc = "`write(|w| ..)` method takes [`indirect_wr_watermark::W`](W) writer structure"]
impl crate::Writable for IndirectWrWatermarkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets indirect_wr_watermark to value 0"]
impl crate::Resettable for IndirectWrWatermarkSpec {
    const RESET_VALUE: u32 = 0;
}
