#[doc = "Register `clr_tx_over` reader"]
pub type R = crate::R<ClrTxOverSpec>;
#[doc = "Register `clr_tx_over` writer"]
pub type W = crate::W<ClrTxOverSpec>;
#[doc = "Field `clr_tx_over` reader - "]
pub type ClrTxOverR = crate::FieldReader<u32>;
#[doc = "Field `clr_tx_over` writer - "]
pub type ClrTxOverW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_tx_over(&self) -> ClrTxOverR {
        ClrTxOverR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_tx_over(&mut self) -> ClrTxOverW<ClrTxOverSpec> {
        ClrTxOverW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear TX Overrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_tx_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_tx_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrTxOverSpec;
impl crate::RegisterSpec for ClrTxOverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_tx_over::R`](R) reader structure"]
impl crate::Readable for ClrTxOverSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_tx_over::W`](W) writer structure"]
impl crate::Writable for ClrTxOverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_tx_over to value 0"]
impl crate::Resettable for ClrTxOverSpec {
    const RESET_VALUE: u32 = 0;
}
