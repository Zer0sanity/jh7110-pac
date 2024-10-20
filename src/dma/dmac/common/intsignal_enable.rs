#[doc = "Register `intsignal_enable` reader"]
pub type R = crate::R<IntsignalEnableSpec>;
#[doc = "Register `intsignal_enable` writer"]
pub type W = crate::W<IntsignalEnableSpec>;
#[doc = "Field `slv_if(_dec_err,_wr2_ro_err,_rd2_wo_err,_wron_hold_err,_rsvd0,_rsvd1,_rsvd2,_rsvd3,_undef_reg_dec_err)` reader - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
pub type SlvIfR = crate::BitReader;
#[doc = "Field `slv_if(_dec_err,_wr2_ro_err,_rd2_wo_err,_wron_hold_err,_rsvd0,_rsvd1,_rsvd2,_rsvd3,_undef_reg_dec_err)` writer - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
pub type SlvIfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slv_if_dec_err` field"]
    #[inline(always)]
    pub fn slv_if(&self, n: u8) -> SlvIfR {
        #[allow(clippy::no_effect)]
        [(); 9][n as usize];
        SlvIfR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_iter(&self) -> impl Iterator<Item = SlvIfR> + '_ {
        (0..9).map(move |n| SlvIfR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_dec_err(&self) -> SlvIfR {
        SlvIfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_wr2_ro_err(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_rd2_wo_err(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_wron_hold_err(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_rsvd0(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_rsvd1(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_rsvd2(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_rsvd3(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    pub fn slv_if_undef_reg_dec_err(&self) -> SlvIfR {
        SlvIfR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slv_if_dec_err` field"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if(&mut self, n: u8) -> SlvIfW<IntsignalEnableSpec> {
        #[allow(clippy::no_effect)]
        [(); 9][n as usize];
        SlvIfW::new(self, n)
    }
    #[doc = "Bit 0 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_dec_err(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_wr2_ro_err(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rd2_wo_err(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_wron_hold_err(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rsvd0(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rsvd1(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rsvd2(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_rsvd3(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 7)
    }
    #[doc = "Bit 8 - DMAC Channel Interrupt Signal Enable Slave Interface - 0: disable interrupt signal, 1: enable interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn slv_if_undef_reg_dec_err(&mut self) -> SlvIfW<IntsignalEnableSpec> {
        SlvIfW::new(self, 8)
    }
}
#[doc = "DMAC Interrupt Signal Enable register contains the DMAC interrupt signal enable settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsignal_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsignal_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntsignalEnableSpec;
impl crate::RegisterSpec for IntsignalEnableSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intsignal_enable::R`](R) reader structure"]
impl crate::Readable for IntsignalEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`intsignal_enable::W`](W) writer structure"]
impl crate::Writable for IntsignalEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets intsignal_enable to value 0"]
impl crate::Resettable for IntsignalEnableSpec {
    const RESET_VALUE: u64 = 0;
}
