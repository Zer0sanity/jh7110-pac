#[doc = "Register `tx_abrt_source` reader"]
pub type R = crate::R<TxAbrtSourceSpec>;
#[doc = "Field `b7_addr_noack` reader - "]
pub type B7AddrNoackR = crate::BitReader;
#[doc = "Field `b10_addr1_noack` reader - "]
pub type B10Addr1NoackR = crate::BitReader;
#[doc = "Field `b10_addr2_noack` reader - "]
pub type B10Addr2NoackR = crate::BitReader;
#[doc = "Field `txdata_noack` reader - "]
pub type TxdataNoackR = crate::BitReader;
#[doc = "Field `gcall_noack` reader - "]
pub type GcallNoackR = crate::BitReader;
#[doc = "Field `gcall_read` reader - "]
pub type GcallReadR = crate::BitReader;
#[doc = "Field `sbyte_ackdet` reader - "]
pub type SbyteAckdetR = crate::BitReader;
#[doc = "Field `sbyte_norstrt` reader - "]
pub type SbyteNorstrtR = crate::BitReader;
#[doc = "Field `b10_rd_norstrt` reader - "]
pub type B10RdNorstrtR = crate::BitReader;
#[doc = "Field `master_dis` reader - "]
pub type MasterDisR = crate::BitReader;
#[doc = "Field `arb_lost` reader - "]
pub type ArbLostR = crate::BitReader;
#[doc = "Field `slave_flush_txfifo` reader - "]
pub type SlaveFlushTxfifoR = crate::BitReader;
#[doc = "Field `slave_arblost` reader - "]
pub type SlaveArblostR = crate::BitReader;
#[doc = "Field `slave_rd_intx` reader - "]
pub type SlaveRdIntxR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn b7_addr_noack(&self) -> B7AddrNoackR {
        B7AddrNoackR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn b10_addr1_noack(&self) -> B10Addr1NoackR {
        B10Addr1NoackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn b10_addr2_noack(&self) -> B10Addr2NoackR {
        B10Addr2NoackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn txdata_noack(&self) -> TxdataNoackR {
        TxdataNoackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gcall_noack(&self) -> GcallNoackR {
        GcallNoackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gcall_read(&self) -> GcallReadR {
        GcallReadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sbyte_ackdet(&self) -> SbyteAckdetR {
        SbyteAckdetR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sbyte_norstrt(&self) -> SbyteNorstrtR {
        SbyteNorstrtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn b10_rd_norstrt(&self) -> B10RdNorstrtR {
        B10RdNorstrtR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn master_dis(&self) -> MasterDisR {
        MasterDisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ArbLostR {
        ArbLostR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slave_flush_txfifo(&self) -> SlaveFlushTxfifoR {
        SlaveFlushTxfifoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slave_arblost(&self) -> SlaveArblostR {
        SlaveArblostR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slave_rd_intx(&self) -> SlaveRdIntxR {
        SlaveRdIntxR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "DesignWare I2C TX Abort Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_abrt_source::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxAbrtSourceSpec;
impl crate::RegisterSpec for TxAbrtSourceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_abrt_source::R`](R) reader structure"]
impl crate::Readable for TxAbrtSourceSpec {}
#[doc = "`reset()` method sets tx_abrt_source to value 0"]
impl crate::Resettable for TxAbrtSourceSpec {
    const RESET_VALUE: u32 = 0;
}
