#[doc = "Register `clr_tx_abrt` reader"]
pub type R = crate::R<ClrTxAbrtSpec>;
#[doc = "Register `clr_tx_abrt` writer"]
pub type W = crate::W<ClrTxAbrtSpec>;
#[doc = "Field `clr_tx_abrt` reader - "]
pub type ClrTxAbrtR = crate::FieldReader<u32>;
#[doc = "Field `clr_tx_abrt` writer - "]
pub type ClrTxAbrtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_tx_abrt(&self) -> ClrTxAbrtR {
        ClrTxAbrtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_tx_abrt(&mut self) -> ClrTxAbrtW<ClrTxAbrtSpec> {
        ClrTxAbrtW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear TX Abort\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_tx_abrt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_tx_abrt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrTxAbrtSpec;
impl crate::RegisterSpec for ClrTxAbrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_tx_abrt::R`](R) reader structure"]
impl crate::Readable for ClrTxAbrtSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_tx_abrt::W`](W) writer structure"]
impl crate::Writable for ClrTxAbrtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_tx_abrt to value 0"]
impl crate::Resettable for ClrTxAbrtSpec {
    const RESET_VALUE: u32 = 0;
}
