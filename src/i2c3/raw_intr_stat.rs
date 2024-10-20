#[doc = "Register `raw_intr_stat` reader"]
pub type R = crate::R<RawIntrStatSpec>;
#[doc = "Field `rx_under` reader - RX FIFO Underrun"]
pub type RxUnderR = crate::BitReader;
#[doc = "Field `rx_over` reader - RX FIFO Overrun"]
pub type RxOverR = crate::BitReader;
#[doc = "Field `rx_full` reader - RX FIFO Full"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `tx_over` reader - TX FIFO Overrun"]
pub type TxOverR = crate::BitReader;
#[doc = "Field `tx_empty` reader - TX FIFO Empty"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `rd_req` reader - Read Request"]
pub type RdReqR = crate::BitReader;
#[doc = "Field `tx_abrt` reader - TX Abort"]
pub type TxAbrtR = crate::BitReader;
#[doc = "Field `rx_done` reader - RX Done"]
pub type RxDoneR = crate::BitReader;
#[doc = "Field `activity` reader - Activity"]
pub type ActivityR = crate::BitReader;
#[doc = "Field `stop_det` reader - Stop DET"]
pub type StopDetR = crate::BitReader;
#[doc = "Field `start_det` reader - Start DET"]
pub type StartDetR = crate::BitReader;
#[doc = "Field `gen_call` reader - General Call"]
pub type GenCallR = crate::BitReader;
#[doc = "Field `restart_det` reader - Restart DET"]
pub type RestartDetR = crate::BitReader;
#[doc = "Field `mst_on_hold` reader - Master on Hold"]
pub type MstOnHoldR = crate::BitReader;
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
#[doc = "DesignWare I2C Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_intr_stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawIntrStatSpec;
impl crate::RegisterSpec for RawIntrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_intr_stat::R`](R) reader structure"]
impl crate::Readable for RawIntrStatSpec {}
#[doc = "`reset()` method sets raw_intr_stat to value 0"]
impl crate::Resettable for RawIntrStatSpec {
    const RESET_VALUE: u32 = 0;
}
