#[doc = "Register `intclear` writer"]
pub type W = crate::W<IntclearSpec>;
#[doc = "Field `slv_if(_dec_err,_wr2_ro_err,_rd2_wo_err,_wron_hold_err,_rsvd0,_rsvd1,_rsvd2,_rsvd3,_undef_reg_dec_err)` writer - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
pub type SlvIfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slv_if_dec_err` field"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if(&mut self, n: u8) -> SlvIfW<IntclearSpec> {
        #[allow(clippy::no_effect)]
        [(); 9][n as usize];
        SlvIfW::new(self, n)
    }
    #[doc = "Bit 0 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_dec_err(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_wr2_ro_err(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rd2_wo_err(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_wron_hold_err(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rsvd0(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rsvd1(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rsvd2(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rsvd3(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 7)
    }
    #[doc = "Bit 8 - DMAC Channel Interrupt Clear Slave Interface - 0: no-op, 1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_undef_reg_dec_err(&mut self) -> SlvIfW<IntclearSpec> {
        SlvIfW::new(self, 8)
    }
}
#[doc = "DMAC Interrupt Clear register contains the DMAC interrupt clear settings.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclearSpec;
impl crate::RegisterSpec for IntclearSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`intclear::W`](W) writer structure"]
impl crate::Writable for IntclearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets intclear to value 0"]
impl crate::Resettable for IntclearSpec {
    const RESET_VALUE: u64 = 0;
}
