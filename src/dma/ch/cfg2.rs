#[doc = "Register `cfg2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `cfg2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `multblk_type(_src,_dst)` reader - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
pub type MultblkTypeR = crate::FieldReader;
#[doc = "Field `multblk_type(_src,_dst)` writer - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
pub type MultblkTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `per(_src,_dst)` reader - Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
pub type PerR = crate::FieldReader;
#[doc = "Field `per(_src,_dst)` writer - Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `tt_fc` reader - Transfer Type and Flow Control - 0: tt=mem-to-mem, fc=dw_axi_dmac, 1: tt=mem-to-per, fc=dw_axi_dmac, 2: tt=per-to-mem, fc=dw_axi_dmac, 3: tt=per-to-per, fc=dw_axi_dmac, 4: tt=per-to-mem, fc=source-peripheral, 5: tt=per-to-per, fc=source-peripheral, 6: tt=mem-to-per, fc=destination-peripheral, 7: tt=per-to-per, fc=destination-peripheral"]
pub type TtFcR = crate::FieldReader;
#[doc = "Field `tt_fc` writer - Transfer Type and Flow Control - 0: tt=mem-to-mem, fc=dw_axi_dmac, 1: tt=mem-to-per, fc=dw_axi_dmac, 2: tt=per-to-mem, fc=dw_axi_dmac, 3: tt=per-to-per, fc=dw_axi_dmac, 4: tt=per-to-mem, fc=source-peripheral, 5: tt=per-to-per, fc=source-peripheral, 6: tt=mem-to-per, fc=destination-peripheral, 7: tt=per-to-per, fc=destination-peripheral"]
pub type TtFcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `hs_sel(_src,_dst)` reader - Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
pub type HsSelR = crate::BitReader;
#[doc = "Field `hs_sel(_src,_dst)` writer - Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
pub type HsSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwhs_pol(_src,_dst)` reader - Hardware Handshaking Polarity - 0: active high, 1: active low"]
pub type HwhsPolR = crate::BitReader;
#[doc = "Field `hwhs_pol(_src,_dst)` writer - Hardware Handshaking Polarity - 0: active high, 1: active low"]
pub type HwhsPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_prior` reader - Channel priority - 0: lowest, NUM_CHAN - 1: highest. **NOTE** a value outside this range leads to undefined behavior."]
pub type ChPriorR = crate::FieldReader;
#[doc = "Field `ch_prior` writer - Channel priority - 0: lowest, NUM_CHAN - 1: highest. **NOTE** a value outside this range leads to undefined behavior."]
pub type ChPriorW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `lock_ch` reader - Lock Channel - 0: no lock, 1: lock."]
pub type LockChR = crate::BitReader;
#[doc = "Field `lock_ch` writer - Lock Channel - 0: no lock, 1: lock."]
pub type LockChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_lock_lvl` reader - Channel Lock Level - 0: entire transfer, 1: current block."]
pub type ChLockLvlR = crate::BitReader;
#[doc = "Field `ch_lock_lvl` writer - Channel Lock Level - 0: entire transfer, 1: current block."]
pub type ChLockLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `osr_lmt(_src,_dst)` reader - Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
pub type OsrLmtR = crate::FieldReader;
#[doc = "Field `osr_lmt(_src,_dst)` writer - Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
pub type OsrLmtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `multblk_type_src` field"]
    #[inline(always)]
    pub fn multblk_type(&self, n: u8) -> MultblkTypeR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MultblkTypeR::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[inline(always)]
    pub fn multblk_type_iter(&self) -> impl Iterator<Item = MultblkTypeR> + '_ {
        (0..2).map(move |n| MultblkTypeR::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[inline(always)]
    pub fn multblk_type_src(&self) -> MultblkTypeR {
        MultblkTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[inline(always)]
    pub fn multblk_type_dst(&self) -> MultblkTypeR {
        MultblkTypeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `per_src` field"]
    #[inline(always)]
    pub fn per(&self, n: u8) -> PerR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PerR::new(((self.bits >> (n * 7 + 4)) & 0x7f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
    #[inline(always)]
    pub fn per_iter(&self) -> impl Iterator<Item = PerR> + '_ {
        (0..2).map(move |n| PerR::new(((self.bits >> (n * 7 + 4)) & 0x7f) as u8))
    }
    #[doc = "Bits 4:10 - Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
    #[inline(always)]
    pub fn per_src(&self) -> PerR {
        PerR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:17 - Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
    #[inline(always)]
    pub fn per_dst(&self) -> PerR {
        PerR::new(((self.bits >> 11) & 0x7f) as u8)
    }
    #[doc = "Bits 32:34 - Transfer Type and Flow Control - 0: tt=mem-to-mem, fc=dw_axi_dmac, 1: tt=mem-to-per, fc=dw_axi_dmac, 2: tt=per-to-mem, fc=dw_axi_dmac, 3: tt=per-to-per, fc=dw_axi_dmac, 4: tt=per-to-mem, fc=source-peripheral, 5: tt=per-to-per, fc=source-peripheral, 6: tt=mem-to-per, fc=destination-peripheral, 7: tt=per-to-per, fc=destination-peripheral"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TtFcR {
        TtFcR::new(((self.bits >> 32) & 7) as u8)
    }
    #[doc = "Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `hs_sel_src` field"]
    #[inline(always)]
    pub fn hs_sel(&self, n: u8) -> HsSelR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        HsSelR::new(((self.bits >> (n + 35)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[inline(always)]
    pub fn hs_sel_iter(&self) -> impl Iterator<Item = HsSelR> + '_ {
        (0..2).map(move |n| HsSelR::new(((self.bits >> (n + 35)) & 1) != 0))
    }
    #[doc = "Bit 35 - Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HsSelR {
        HsSelR::new(((self.bits >> 35) & 1) != 0)
    }
    #[doc = "Bit 36 - Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HsSelR {
        HsSelR::new(((self.bits >> 36) & 1) != 0)
    }
    #[doc = "Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `hwhs_pol_src` field"]
    #[inline(always)]
    pub fn hwhs_pol(&self, n: u8) -> HwhsPolR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        HwhsPolR::new(((self.bits >> (n + 37)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[inline(always)]
    pub fn hwhs_pol_iter(&self) -> impl Iterator<Item = HwhsPolR> + '_ {
        (0..2).map(move |n| HwhsPolR::new(((self.bits >> (n + 37)) & 1) != 0))
    }
    #[doc = "Bit 37 - Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[inline(always)]
    pub fn hwhs_pol_src(&self) -> HwhsPolR {
        HwhsPolR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Bit 38 - Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[inline(always)]
    pub fn hwhs_pol_dst(&self) -> HwhsPolR {
        HwhsPolR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bits 47:51 - Channel priority - 0: lowest, NUM_CHAN - 1: highest. **NOTE** a value outside this range leads to undefined behavior."]
    #[inline(always)]
    pub fn ch_prior(&self) -> ChPriorR {
        ChPriorR::new(((self.bits >> 47) & 0x1f) as u8)
    }
    #[doc = "Bit 52 - Lock Channel - 0: no lock, 1: lock."]
    #[inline(always)]
    pub fn lock_ch(&self) -> LockChR {
        LockChR::new(((self.bits >> 52) & 1) != 0)
    }
    #[doc = "Bit 53 - Channel Lock Level - 0: entire transfer, 1: current block."]
    #[inline(always)]
    pub fn ch_lock_lvl(&self) -> ChLockLvlR {
        ChLockLvlR::new(((self.bits >> 53) & 1) != 0)
    }
    #[doc = "Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `osr_lmt_src` field"]
    #[inline(always)]
    pub fn osr_lmt(&self, n: u8) -> OsrLmtR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OsrLmtR::new(((self.bits >> (n * 4 + 55)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[inline(always)]
    pub fn osr_lmt_iter(&self) -> impl Iterator<Item = OsrLmtR> + '_ {
        (0..2).map(move |n| OsrLmtR::new(((self.bits >> (n * 4 + 55)) & 0x0f) as u8))
    }
    #[doc = "Bits 55:58 - Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[inline(always)]
    pub fn osr_lmt_src(&self) -> OsrLmtR {
        OsrLmtR::new(((self.bits >> 55) & 0x0f) as u8)
    }
    #[doc = "Bits 59:62 - Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[inline(always)]
    pub fn osr_lmt_dst(&self) -> OsrLmtR {
        OsrLmtR::new(((self.bits >> 59) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `multblk_type_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn multblk_type(&mut self, n: u8) -> MultblkTypeW<Cfg2Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MultblkTypeW::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[inline(always)]
    #[must_use]
    pub fn multblk_type_src(&mut self) -> MultblkTypeW<Cfg2Spec> {
        MultblkTypeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[inline(always)]
    #[must_use]
    pub fn multblk_type_dst(&mut self) -> MultblkTypeW<Cfg2Spec> {
        MultblkTypeW::new(self, 2)
    }
    #[doc = "Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `per_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self, n: u8) -> PerW<Cfg2Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PerW::new(self, n * 7 + 4)
    }
    #[doc = "Bits 4:10 - Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
    #[inline(always)]
    #[must_use]
    pub fn per_src(&mut self) -> PerW<Cfg2Spec> {
        PerW::new(self, 4)
    }
    #[doc = "Bits 11:17 - Assigns a hardware handshaking interface - **NOTE** only one peripheral should be assigned to the same interface."]
    #[inline(always)]
    #[must_use]
    pub fn per_dst(&mut self) -> PerW<Cfg2Spec> {
        PerW::new(self, 11)
    }
    #[doc = "Bits 32:34 - Transfer Type and Flow Control - 0: tt=mem-to-mem, fc=dw_axi_dmac, 1: tt=mem-to-per, fc=dw_axi_dmac, 2: tt=per-to-mem, fc=dw_axi_dmac, 3: tt=per-to-per, fc=dw_axi_dmac, 4: tt=per-to-mem, fc=source-peripheral, 5: tt=per-to-per, fc=source-peripheral, 6: tt=mem-to-per, fc=destination-peripheral, 7: tt=per-to-per, fc=destination-peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn tt_fc(&mut self) -> TtFcW<Cfg2Spec> {
        TtFcW::new(self, 32)
    }
    #[doc = "Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `hs_sel_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel(&mut self, n: u8) -> HsSelW<Cfg2Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        HsSelW::new(self, n + 35)
    }
    #[doc = "Bit 35 - Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_src(&mut self) -> HsSelW<Cfg2Spec> {
        HsSelW::new(self, 35)
    }
    #[doc = "Bit 36 - Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_dst(&mut self) -> HsSelW<Cfg2Spec> {
        HsSelW::new(self, 36)
    }
    #[doc = "Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `hwhs_pol_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn hwhs_pol(&mut self, n: u8) -> HwhsPolW<Cfg2Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        HwhsPolW::new(self, n + 37)
    }
    #[doc = "Bit 37 - Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[inline(always)]
    #[must_use]
    pub fn hwhs_pol_src(&mut self) -> HwhsPolW<Cfg2Spec> {
        HwhsPolW::new(self, 37)
    }
    #[doc = "Bit 38 - Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[inline(always)]
    #[must_use]
    pub fn hwhs_pol_dst(&mut self) -> HwhsPolW<Cfg2Spec> {
        HwhsPolW::new(self, 38)
    }
    #[doc = "Bits 47:51 - Channel priority - 0: lowest, NUM_CHAN - 1: highest. **NOTE** a value outside this range leads to undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ch_prior(&mut self) -> ChPriorW<Cfg2Spec> {
        ChPriorW::new(self, 47)
    }
    #[doc = "Bit 52 - Lock Channel - 0: no lock, 1: lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch(&mut self) -> LockChW<Cfg2Spec> {
        LockChW::new(self, 52)
    }
    #[doc = "Bit 53 - Channel Lock Level - 0: entire transfer, 1: current block."]
    #[inline(always)]
    #[must_use]
    pub fn ch_lock_lvl(&mut self) -> ChLockLvlW<Cfg2Spec> {
        ChLockLvlW::new(self, 53)
    }
    #[doc = "Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `osr_lmt_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn osr_lmt(&mut self, n: u8) -> OsrLmtW<Cfg2Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OsrLmtW::new(self, n * 4 + 55)
    }
    #[doc = "Bits 55:58 - Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[inline(always)]
    #[must_use]
    pub fn osr_lmt_src(&mut self) -> OsrLmtW<Cfg2Spec> {
        OsrLmtW::new(self, 55)
    }
    #[doc = "Bits 59:62 - Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[inline(always)]
    #[must_use]
    pub fn osr_lmt_dst(&mut self) -> OsrLmtW<Cfg2Spec> {
        OsrLmtW::new(self, 59)
    }
}
#[doc = "DMAC Channel Configuration 2 register (only exists for DMAX_NUM_CHANNELS > 8).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets cfg2 to value 0"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u64 = 0;
}
