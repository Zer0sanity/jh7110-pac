#[doc = "Register `intstatus` reader"]
pub type R = crate::R<IntstatusSpec>;
#[doc = "Field `slv_if(_dec_err,_wr2_ro_err,_rd2_wo_err,_wron_hold_err,_rsvd0,_rsvd1,_rsvd2,_rsvd3,_undef_reg_dec_err)` reader - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
pub type SlvIfR = crate::BitReader;
impl R {
    #[doc = "DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slv_if_dec_err` field"]
    #[inline(always)]
    pub fn slv_if(&self, n: u8) -> SlvIfR {
        #[allow(clippy::no_effect)]
        [(); 9][n as usize];
        SlvIfR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_iter(&self) -> impl Iterator<Item = SlvIfR> + '_ {
        (0..9).map(move |n| SlvIfR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_dec_err(&self) -> SlvIfR {
        SlvIfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_wr2_ro_err(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_rd2_wo_err(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_wron_hold_err(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_rsvd0(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_rsvd1(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_rsvd2(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_rsvd3(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAC Channel Interrupt Status Slave Interface - 0: no interrupt, 1: interrupt active"]
    #[inline(always)]
    pub fn slv_if_undef_reg_dec_err(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DMAC Interrupt Status register contains the DMAC interrupt status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatusSpec;
impl crate::RegisterSpec for IntstatusSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for IntstatusSpec {}
#[doc = "`reset()` method sets intstatus to value 0"]
impl crate::Resettable for IntstatusSpec {
    const RESET_VALUE: u64 = 0;
}
