#[doc = "Register `cfg` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `cfg` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `multblk_type(_src,_dst)` reader - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
pub type MultblkTypeR = crate::FieldReader;
#[doc = "Field `multblk_type(_src,_dst)` writer - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
pub type MultblkTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
#[doc = "Field `per(_src,_dst)` reader - Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
pub type PerR = crate::FieldReader;
#[doc = "Field `per(_src,_dst)` writer - Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
pub type PerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ch_prior` reader - Channel priority - 0: lowest, NUM_CHAN - 1: highest. **NOTE** a value outside this range leads to undefined behavior."]
pub type ChPriorR = crate::FieldReader;
#[doc = "Field `ch_prior` writer - Channel priority - 0: lowest, NUM_CHAN - 1: highest. **NOTE** a value outside this range leads to undefined behavior."]
pub type ChPriorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    #[doc = "Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `per_src` field"]
    #[inline(always)]
    pub fn per(&self, n: u8) -> PerR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PerR::new(((self.bits >> (n * 5 + 39)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
    #[inline(always)]
    pub fn per_iter(&self) -> impl Iterator<Item = PerR> + '_ {
        (0..2).map(move |n| PerR::new(((self.bits >> (n * 5 + 39)) & 0x1f) as u8))
    }
    #[doc = "Bits 39:43 - Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
    #[inline(always)]
    pub fn per_src(&self) -> PerR {
        PerR::new(((self.bits >> 39) & 0x1f) as u8)
    }
    #[doc = "Bits 44:48 - Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
    #[inline(always)]
    pub fn per_dst(&self) -> PerR {
        PerR::new(((self.bits >> 44) & 0x1f) as u8)
    }
    #[doc = "Bits 49:51 - Channel priority - 0: lowest, NUM_CHAN - 1: highest. **NOTE** a value outside this range leads to undefined behavior."]
    #[inline(always)]
    pub fn ch_prior(&self) -> ChPriorR {
        ChPriorR::new(((self.bits >> 49) & 7) as u8)
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
    pub fn multblk_type(&mut self, n: u8) -> MultblkTypeW<CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MultblkTypeW::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[inline(always)]
    #[must_use]
    pub fn multblk_type_src(&mut self) -> MultblkTypeW<CfgSpec> {
        MultblkTypeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Source Multi Block Transfer Type - 0b00: Contiguous, 0b01: Reload, 0b10: Shadow Register, 0b11: Linked List"]
    #[inline(always)]
    #[must_use]
    pub fn multblk_type_dst(&mut self) -> MultblkTypeW<CfgSpec> {
        MultblkTypeW::new(self, 2)
    }
    #[doc = "Bits 32:34 - Transfer Type and Flow Control - 0: tt=mem-to-mem, fc=dw_axi_dmac, 1: tt=mem-to-per, fc=dw_axi_dmac, 2: tt=per-to-mem, fc=dw_axi_dmac, 3: tt=per-to-per, fc=dw_axi_dmac, 4: tt=per-to-mem, fc=source-peripheral, 5: tt=per-to-per, fc=source-peripheral, 6: tt=mem-to-per, fc=destination-peripheral, 7: tt=per-to-per, fc=destination-peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn tt_fc(&mut self) -> TtFcW<CfgSpec> {
        TtFcW::new(self, 32)
    }
    #[doc = "Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `hs_sel_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel(&mut self, n: u8) -> HsSelW<CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        HsSelW::new(self, n + 35)
    }
    #[doc = "Bit 35 - Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_src(&mut self) -> HsSelW<CfgSpec> {
        HsSelW::new(self, 35)
    }
    #[doc = "Bit 36 - Source Software or Hardware Handshaking Select - 0: hardware handshake, 1: software handshake"]
    #[inline(always)]
    #[must_use]
    pub fn hs_sel_dst(&mut self) -> HsSelW<CfgSpec> {
        HsSelW::new(self, 36)
    }
    #[doc = "Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `hwhs_pol_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn hwhs_pol(&mut self, n: u8) -> HwhsPolW<CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        HwhsPolW::new(self, n + 37)
    }
    #[doc = "Bit 37 - Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[inline(always)]
    #[must_use]
    pub fn hwhs_pol_src(&mut self) -> HwhsPolW<CfgSpec> {
        HwhsPolW::new(self, 37)
    }
    #[doc = "Bit 38 - Hardware Handshaking Polarity - 0: active high, 1: active low"]
    #[inline(always)]
    #[must_use]
    pub fn hwhs_pol_dst(&mut self) -> HwhsPolW<CfgSpec> {
        HwhsPolW::new(self, 38)
    }
    #[doc = "Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `per_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self, n: u8) -> PerW<CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        PerW::new(self, n * 5 + 39)
    }
    #[doc = "Bits 39:43 - Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
    #[inline(always)]
    #[must_use]
    pub fn per_src(&mut self) -> PerW<CfgSpec> {
        PerW::new(self, 39)
    }
    #[doc = "Bits 44:48 - Assigns a hardware handshaking interface - **NOTE** for proper operation, only one peripheral should be assigned to the same handshaking interface."]
    #[inline(always)]
    #[must_use]
    pub fn per_dst(&mut self) -> PerW<CfgSpec> {
        PerW::new(self, 44)
    }
    #[doc = "Bits 49:51 - Channel priority - 0: lowest, NUM_CHAN - 1: highest. **NOTE** a value outside this range leads to undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ch_prior(&mut self) -> ChPriorW<CfgSpec> {
        ChPriorW::new(self, 49)
    }
    #[doc = "Bit 52 - Lock Channel - 0: no lock, 1: lock."]
    #[inline(always)]
    #[must_use]
    pub fn lock_ch(&mut self) -> LockChW<CfgSpec> {
        LockChW::new(self, 52)
    }
    #[doc = "Bit 53 - Channel Lock Level - 0: entire transfer, 1: current block."]
    #[inline(always)]
    #[must_use]
    pub fn ch_lock_lvl(&mut self) -> ChLockLvlW<CfgSpec> {
        ChLockLvlW::new(self, 53)
    }
    #[doc = "Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `osr_lmt_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn osr_lmt(&mut self, n: u8) -> OsrLmtW<CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        OsrLmtW::new(self, n * 4 + 55)
    }
    #[doc = "Bits 55:58 - Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[inline(always)]
    #[must_use]
    pub fn osr_lmt_src(&mut self) -> OsrLmtW<CfgSpec> {
        OsrLmtW::new(self, 55)
    }
    #[doc = "Bits 59:62 - Outstanding Request Limit. **NOTE** Maximum outstanding request limit is 16."]
    #[inline(always)]
    #[must_use]
    pub fn osr_lmt_dst(&mut self) -> OsrLmtW<CfgSpec> {
        OsrLmtW::new(self, 59)
    }
}
#[doc = "DMAC Channel Configuration register (only exists for DMAX_NUM_CHANNELS &lt;= 8).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets cfg to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u64 = 0;
}
