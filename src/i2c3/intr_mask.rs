#[doc = "Register `intr_mask` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `intr_mask` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `rx_under` reader - RX FIFO Underrun"]
pub type RxUnderR = crate::BitReader;
#[doc = "Field `rx_under` writer - RX FIFO Underrun"]
pub type RxUnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_over` reader - RX FIFO Overrun"]
pub type RxOverR = crate::BitReader;
#[doc = "Field `rx_over` writer - RX FIFO Overrun"]
pub type RxOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_full` reader - RX FIFO Full"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `rx_full` writer - RX FIFO Full"]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_over` reader - TX FIFO Overrun"]
pub type TxOverR = crate::BitReader;
#[doc = "Field `tx_over` writer - TX FIFO Overrun"]
pub type TxOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_empty` reader - TX FIFO Empty"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `tx_empty` writer - TX FIFO Empty"]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_req` reader - Read Request"]
pub type RdReqR = crate::BitReader;
#[doc = "Field `rd_req` writer - Read Request"]
pub type RdReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_abrt` reader - TX Abort"]
pub type TxAbrtR = crate::BitReader;
#[doc = "Field `tx_abrt` writer - TX Abort"]
pub type TxAbrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_done` reader - RX Done"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `rx_done` writer - RX Done"]
pub type RxDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `activity` reader - Activity"]
pub type ActivityR = crate::BitReader;
#[doc = "Field `activity` writer - Activity"]
pub type ActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_det` reader - Stop DET"]
pub type StopDetR = crate::BitReader;
#[doc = "Field `stop_det` writer - Stop DET"]
pub type StopDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `start_det` reader - Start DET"]
pub type StartDetR = crate::BitReader;
#[doc = "Field `start_det` writer - Start DET"]
pub type StartDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gen_call` reader - General Call"]
pub type GenCallR = crate::BitReader;
#[doc = "Field `gen_call` writer - General Call"]
pub type GenCallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `restart_det` reader - Restart DET"]
pub type RestartDetR = crate::BitReader;
#[doc = "Field `restart_det` writer - Restart DET"]
pub type RestartDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mst_on_hold` reader - Master on Hold"]
pub type MstOnHoldR = crate::BitReader;
#[doc = "Field `mst_on_hold` writer - Master on Hold"]
pub type MstOnHoldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX FIFO Underrun"]
    #[inline(always)]
    pub fn rx_under(&self) -> RxUnderR {
        RxUnderR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Overrun"]
    #[inline(always)]
    pub fn rx_over(&self) -> RxOverR {
        RxOverR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Full"]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO Overrun"]
    #[inline(always)]
    pub fn tx_over(&self) -> TxOverR {
        TxOverR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO Empty"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read Request"]
    #[inline(always)]
    pub fn rd_req(&self) -> RdReqR {
        RdReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX Abort"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TxAbrtR {
        TxAbrtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Done"]
    #[inline(always)]
    pub fn rx_done(&self) -> RxDoneR {
        RxDoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Activity"]
    #[inline(always)]
    pub fn activity(&self) -> ActivityR {
        ActivityR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop DET"]
    #[inline(always)]
    pub fn stop_det(&self) -> StopDetR {
        StopDetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start DET"]
    #[inline(always)]
    pub fn start_det(&self) -> StartDetR {
        StartDetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - General Call"]
    #[inline(always)]
    pub fn gen_call(&self) -> GenCallR {
        GenCallR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Restart DET"]
    #[inline(always)]
    pub fn restart_det(&self) -> RestartDetR {
        RestartDetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master on Hold"]
    #[inline(always)]
    pub fn mst_on_hold(&self) -> MstOnHoldR {
        MstOnHoldR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_under(&mut self) -> RxUnderW<IntrMaskSpec> {
        RxUnderW::new(self, 0)
    }
    #[doc = "Bit 1 - RX FIFO Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn rx_over(&mut self) -> RxOverW<IntrMaskSpec> {
        RxOverW::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn rx_full(&mut self) -> RxFullW<IntrMaskSpec> {
        RxFullW::new(self, 2)
    }
    #[doc = "Bit 3 - TX FIFO Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn tx_over(&mut self) -> TxOverW<IntrMaskSpec> {
        TxOverW::new(self, 3)
    }
    #[doc = "Bit 4 - TX FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<IntrMaskSpec> {
        TxEmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - Read Request"]
    #[inline(always)]
    #[must_use]
    pub fn rd_req(&mut self) -> RdReqW<IntrMaskSpec> {
        RdReqW::new(self, 5)
    }
    #[doc = "Bit 6 - TX Abort"]
    #[inline(always)]
    #[must_use]
    pub fn tx_abrt(&mut self) -> TxAbrtW<IntrMaskSpec> {
        TxAbrtW::new(self, 6)
    }
    #[doc = "Bit 7 - RX Done"]
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RxDoneW<IntrMaskSpec> {
        RxDoneW::new(self, 7)
    }
    #[doc = "Bit 8 - Activity"]
    #[inline(always)]
    #[must_use]
    pub fn activity(&mut self) -> ActivityW<IntrMaskSpec> {
        ActivityW::new(self, 8)
    }
    #[doc = "Bit 9 - Stop DET"]
    #[inline(always)]
    #[must_use]
    pub fn stop_det(&mut self) -> StopDetW<IntrMaskSpec> {
        StopDetW::new(self, 9)
    }
    #[doc = "Bit 10 - Start DET"]
    #[inline(always)]
    #[must_use]
    pub fn start_det(&mut self) -> StartDetW<IntrMaskSpec> {
        StartDetW::new(self, 10)
    }
    #[doc = "Bit 11 - General Call"]
    #[inline(always)]
    #[must_use]
    pub fn gen_call(&mut self) -> GenCallW<IntrMaskSpec> {
        GenCallW::new(self, 11)
    }
    #[doc = "Bit 12 - Restart DET"]
    #[inline(always)]
    #[must_use]
    pub fn restart_det(&mut self) -> RestartDetW<IntrMaskSpec> {
        RestartDetW::new(self, 12)
    }
    #[doc = "Bit 13 - Master on Hold"]
    #[inline(always)]
    #[must_use]
    pub fn mst_on_hold(&mut self) -> MstOnHoldW<IntrMaskSpec> {
        MstOnHoldW::new(self, 13)
    }
}
#[doc = "DesignWare I2C Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets intr_mask to value 0"]
impl crate::Resettable for IntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
