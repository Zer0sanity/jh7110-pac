#[doc = "Register `signal_enable` reader"]
pub type R = crate::R<SignalEnableSpec>;
#[doc = "Register `signal_enable` writer"]
pub type W = crate::W<SignalEnableSpec>;
#[doc = "Field `blk_tr_done` reader - Channel Block Transfer Done - 0: disabled, 1: enabled"]
pub type BlkTrDoneR = crate::BitReader;
#[doc = "Field `blk_tr_done` writer - Channel Block Transfer Done - 0: disabled, 1: enabled"]
pub type BlkTrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_tr_done` reader - Channel DMA Transfer Done - 0: disabled, 1: enabled"]
pub type DmaTrDoneR = crate::BitReader;
#[doc = "Field `dma_tr_done` writer - Channel DMA Transfer Done - 0: disabled, 1: enabled"]
pub type DmaTrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `transcomp(_src,_dst)` reader - Channel Transfer Complete - 0: disabled, 1: enabled"]
pub type TranscompR = crate::BitReader;
#[doc = "Field `transcomp(_src,_dst)` writer - Channel Transfer Complete - 0: disabled, 1: enabled"]
pub type TranscompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dec_err(_src,_dst)` reader - Channel Decoding Error - 0: disabled, 1: enabled"]
pub type DecErrR = crate::BitReader;
#[doc = "Field `dec_err(_src,_dst)` writer - Channel Decoding Error - 0: disabled, 1: enabled"]
pub type DecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slv_err(_src,_dst)` reader - Channel Slave Error - 0: disabled, 1: enabled"]
pub type SlvErrR = crate::BitReader;
#[doc = "Field `slv_err(_src,_dst)` writer - Channel Slave Error - 0: disabled, 1: enabled"]
pub type SlvErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lli_dec_err(_rd,_wr)` reader - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
pub type LliDecErrR = crate::BitReader;
#[doc = "Field `lli_dec_err(_rd,_wr)` writer - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
pub type LliDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lli_slv_err(_rd,_wr)` reader - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
pub type LliSlvErrR = crate::BitReader;
#[doc = "Field `lli_slv_err(_rd,_wr)` writer - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
pub type LliSlvErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `shadow_or_invalid_lli_err` reader - Channel Shadow Register or Linked List Item Invalid Error - 0: disabled, 1: enabled"]
pub type ShadowOrInvalidLliErrR = crate::BitReader;
#[doc = "Field `shadow_or_invalid_lli_err` writer - Channel Shadow Register or Linked List Item Invalid Error - 0: disabled, 1: enabled"]
pub type ShadowOrInvalidLliErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_multiblktype_err` reader - Channel Slave Interface Multi Block Type Error - 0: disabled, 1: enabled"]
pub type SlvifMultiblktypeErrR = crate::BitReader;
#[doc = "Field `slvif_multiblktype_err` writer - Channel Slave Interface Multi Block Type Error - 0: disabled, 1: enabled"]
pub type SlvifMultiblktypeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_dec_err` reader - Channel Slave Interface Decoding Error - 0: disabled, 1: enabled"]
pub type SlvifDecErrR = crate::BitReader;
#[doc = "Field `slvif_dec_err` writer - Channel Slave Interface Decoding Error - 0: disabled, 1: enabled"]
pub type SlvifDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wr2ro_err` reader - Channel Slave Interface Write to Read-only Error - 0: disabled, 1: enabled"]
pub type SlvifWr2roErrR = crate::BitReader;
#[doc = "Field `slvif_wr2ro_err` writer - Channel Slave Interface Write to Read-only Error - 0: disabled, 1: enabled"]
pub type SlvifWr2roErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_r2wro_err` reader - Channel Slave Interface Read to Write-only Error - 0: disabled, 1: enabled"]
pub type SlvifR2wroErrR = crate::BitReader;
#[doc = "Field `slvif_r2wro_err` writer - Channel Slave Interface Read to Write-only Error - 0: disabled, 1: enabled"]
pub type SlvifR2wroErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wronchen_err` reader - Channel Slave Interface Write On Channel Enabled Error - 0: disabled, 1: enabled"]
pub type SlvifWronchenErrR = crate::BitReader;
#[doc = "Field `slvif_wronchen_err` writer - Channel Slave Interface Write On Channel Enabled Error - 0: disabled, 1: enabled"]
pub type SlvifWronchenErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_shadow_wron_valid_err` reader - Channel Slave Interface Shadow Register Write On Valid Error - 0: disabled, 1: enabled"]
pub type SlvifShadowWronValidErrR = crate::BitReader;
#[doc = "Field `slvif_shadow_wron_valid_err` writer - Channel Slave Interface Shadow Register Write On Valid Error - 0: disabled, 1: enabled"]
pub type SlvifShadowWronValidErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wron_hold_err` reader - Channel Slave Interface Write On Hold Error - 0: disabled, 1: enabled"]
pub type SlvifWronHoldErrR = crate::BitReader;
#[doc = "Field `slvif_wron_hold_err` writer - Channel Slave Interface Write On Hold Error - 0: disabled, 1: enabled"]
pub type SlvifWronHoldErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_lock_cleared` reader - Channel Lock Cleared - 0: disabled, 1: enabled"]
pub type ChLockClearedR = crate::BitReader;
#[doc = "Field `ch_lock_cleared` writer - Channel Lock Cleared - 0: disabled, 1: enabled"]
pub type ChLockClearedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_src_suspended` reader - Channel Source Suspended - 0: disabled, 1: enabled"]
pub type ChSrcSuspendedR = crate::BitReader;
#[doc = "Field `ch_src_suspended` writer - Channel Source Suspended - 0: disabled, 1: enabled"]
pub type ChSrcSuspendedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_suspended` reader - Channel Suspended - 0: disabled, 1: enabled"]
pub type ChSuspendedR = crate::BitReader;
#[doc = "Field `ch_suspended` writer - Channel Suspended - 0: disabled, 1: enabled"]
pub type ChSuspendedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_disabled` reader - Channel Disabled - 0: disabled, 1: enabled"]
pub type ChDisabledR = crate::BitReader;
#[doc = "Field `ch_disabled` writer - Channel Disabled - 0: disabled, 1: enabled"]
pub type ChDisabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_aborted` reader - Channel Aborted - 0: disabled, 1: enabled"]
pub type ChAbortedR = crate::BitReader;
#[doc = "Field `ch_aborted` writer - Channel Aborted - 0: disabled, 1: enabled"]
pub type ChAbortedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Block Transfer Done - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn blk_tr_done(&self) -> BlkTrDoneR {
        BlkTrDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel DMA Transfer Done - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn dma_tr_done(&self) -> DmaTrDoneR {
        DmaTrDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `transcomp_src` field"]
    #[inline(always)]
    pub fn transcomp(&self, n: u8) -> TranscompR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TranscompR::new(((self.bits >> (n + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn transcomp_iter(&self) -> impl Iterator<Item = TranscompR> + '_ {
        (0..2).map(move |n| TranscompR::new(((self.bits >> (n + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn transcomp_src(&self) -> TranscompR {
        TranscompR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn transcomp_dst(&self) -> TranscompR {
        TranscompR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Channel Decoding Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dec_err_src` field"]
    #[inline(always)]
    pub fn dec_err(&self, n: u8) -> DecErrR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DecErrR::new(((self.bits >> (n + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn dec_err_iter(&self) -> impl Iterator<Item = DecErrR> + '_ {
        (0..2).map(move |n| DecErrR::new(((self.bits >> (n + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - Channel Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn dec_err_src(&self) -> DecErrR {
        DecErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn dec_err_dst(&self) -> DecErrR {
        DecErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Channel Slave Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slv_err_src` field"]
    #[inline(always)]
    pub fn slv_err(&self, n: u8) -> SlvErrR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SlvErrR::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slv_err_iter(&self) -> impl Iterator<Item = SlvErrR> + '_ {
        (0..2).map(move |n| SlvErrR::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    #[doc = "Bit 7 - Channel Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slv_err_src(&self) -> SlvErrR {
        SlvErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slv_err_dst(&self) -> SlvErrR {
        SlvErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lli_dec_err_rd` field"]
    #[inline(always)]
    pub fn lli_dec_err(&self, n: u8) -> LliDecErrR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        LliDecErrR::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn lli_dec_err_iter(&self) -> impl Iterator<Item = LliDecErrR> + '_ {
        (0..2).map(move |n| LliDecErrR::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn lli_dec_err_rd(&self) -> LliDecErrR {
        LliDecErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn lli_dec_err_wr(&self) -> LliDecErrR {
        LliDecErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lli_slv_err_rd` field"]
    #[inline(always)]
    pub fn lli_slv_err(&self, n: u8) -> LliSlvErrR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        LliSlvErrR::new(((self.bits >> (n + 11)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn lli_slv_err_iter(&self) -> impl Iterator<Item = LliSlvErrR> + '_ {
        (0..2).map(move |n| LliSlvErrR::new(((self.bits >> (n + 11)) & 1) != 0))
    }
    #[doc = "Bit 11 - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn lli_slv_err_rd(&self) -> LliSlvErrR {
        LliSlvErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn lli_slv_err_wr(&self) -> LliSlvErrR {
        LliSlvErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel Shadow Register or Linked List Item Invalid Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn shadow_or_invalid_lli_err(&self) -> ShadowOrInvalidLliErrR {
        ShadowOrInvalidLliErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel Slave Interface Multi Block Type Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slvif_multiblktype_err(&self) -> SlvifMultiblktypeErrR {
        SlvifMultiblktypeErrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel Slave Interface Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slvif_dec_err(&self) -> SlvifDecErrR {
        SlvifDecErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel Slave Interface Write to Read-only Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slvif_wr2ro_err(&self) -> SlvifWr2roErrR {
        SlvifWr2roErrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel Slave Interface Read to Write-only Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slvif_r2wro_err(&self) -> SlvifR2wroErrR {
        SlvifR2wroErrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel Slave Interface Write On Channel Enabled Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slvif_wronchen_err(&self) -> SlvifWronchenErrR {
        SlvifWronchenErrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel Slave Interface Shadow Register Write On Valid Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slvif_shadow_wron_valid_err(&self) -> SlvifShadowWronValidErrR {
        SlvifShadowWronValidErrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel Slave Interface Write On Hold Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn slvif_wron_hold_err(&self) -> SlvifWronHoldErrR {
        SlvifWronHoldErrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel Lock Cleared - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn ch_lock_cleared(&self) -> ChLockClearedR {
        ChLockClearedR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel Source Suspended - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn ch_src_suspended(&self) -> ChSrcSuspendedR {
        ChSrcSuspendedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel Suspended - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn ch_suspended(&self) -> ChSuspendedR {
        ChSuspendedR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel Disabled - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn ch_disabled(&self) -> ChDisabledR {
        ChDisabledR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel Aborted - 0: disabled, 1: enabled"]
    #[inline(always)]
    pub fn ch_aborted(&self) -> ChAbortedR {
        ChAbortedR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Block Transfer Done - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn blk_tr_done(&mut self) -> BlkTrDoneW<SignalEnableSpec> {
        BlkTrDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel DMA Transfer Done - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tr_done(&mut self) -> DmaTrDoneW<SignalEnableSpec> {
        DmaTrDoneW::new(self, 1)
    }
    #[doc = "Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `transcomp_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn transcomp(&mut self, n: u8) -> TranscompW<SignalEnableSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TranscompW::new(self, n + 3)
    }
    #[doc = "Bit 3 - Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn transcomp_src(&mut self) -> TranscompW<SignalEnableSpec> {
        TranscompW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn transcomp_dst(&mut self) -> TranscompW<SignalEnableSpec> {
        TranscompW::new(self, 4)
    }
    #[doc = "Channel Decoding Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dec_err_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn dec_err(&mut self, n: u8) -> DecErrW<SignalEnableSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DecErrW::new(self, n + 5)
    }
    #[doc = "Bit 5 - Channel Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dec_err_src(&mut self) -> DecErrW<SignalEnableSpec> {
        DecErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dec_err_dst(&mut self) -> DecErrW<SignalEnableSpec> {
        DecErrW::new(self, 6)
    }
    #[doc = "Channel Slave Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slv_err_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn slv_err(&mut self, n: u8) -> SlvErrW<SignalEnableSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SlvErrW::new(self, n + 7)
    }
    #[doc = "Bit 7 - Channel Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slv_err_src(&mut self) -> SlvErrW<SignalEnableSpec> {
        SlvErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slv_err_dst(&mut self) -> SlvErrW<SignalEnableSpec> {
        SlvErrW::new(self, 8)
    }
    #[doc = "Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lli_dec_err_rd` field"]
    #[inline(always)]
    #[must_use]
    pub fn lli_dec_err(&mut self, n: u8) -> LliDecErrW<SignalEnableSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        LliDecErrW::new(self, n + 9)
    }
    #[doc = "Bit 9 - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lli_dec_err_rd(&mut self) -> LliDecErrW<SignalEnableSpec> {
        LliDecErrW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lli_dec_err_wr(&mut self) -> LliDecErrW<SignalEnableSpec> {
        LliDecErrW::new(self, 10)
    }
    #[doc = "Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lli_slv_err_rd` field"]
    #[inline(always)]
    #[must_use]
    pub fn lli_slv_err(&mut self, n: u8) -> LliSlvErrW<SignalEnableSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        LliSlvErrW::new(self, n + 11)
    }
    #[doc = "Bit 11 - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lli_slv_err_rd(&mut self) -> LliSlvErrW<SignalEnableSpec> {
        LliSlvErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lli_slv_err_wr(&mut self) -> LliSlvErrW<SignalEnableSpec> {
        LliSlvErrW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel Shadow Register or Linked List Item Invalid Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn shadow_or_invalid_lli_err(&mut self) -> ShadowOrInvalidLliErrW<SignalEnableSpec> {
        ShadowOrInvalidLliErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel Slave Interface Multi Block Type Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_multiblktype_err(&mut self) -> SlvifMultiblktypeErrW<SignalEnableSpec> {
        SlvifMultiblktypeErrW::new(self, 14)
    }
    #[doc = "Bit 16 - Channel Slave Interface Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_dec_err(&mut self) -> SlvifDecErrW<SignalEnableSpec> {
        SlvifDecErrW::new(self, 16)
    }
    #[doc = "Bit 17 - Channel Slave Interface Write to Read-only Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wr2ro_err(&mut self) -> SlvifWr2roErrW<SignalEnableSpec> {
        SlvifWr2roErrW::new(self, 17)
    }
    #[doc = "Bit 18 - Channel Slave Interface Read to Write-only Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_r2wro_err(&mut self) -> SlvifR2wroErrW<SignalEnableSpec> {
        SlvifR2wroErrW::new(self, 18)
    }
    #[doc = "Bit 19 - Channel Slave Interface Write On Channel Enabled Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wronchen_err(&mut self) -> SlvifWronchenErrW<SignalEnableSpec> {
        SlvifWronchenErrW::new(self, 19)
    }
    #[doc = "Bit 20 - Channel Slave Interface Shadow Register Write On Valid Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_shadow_wron_valid_err(&mut self) -> SlvifShadowWronValidErrW<SignalEnableSpec> {
        SlvifShadowWronValidErrW::new(self, 20)
    }
    #[doc = "Bit 21 - Channel Slave Interface Write On Hold Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wron_hold_err(&mut self) -> SlvifWronHoldErrW<SignalEnableSpec> {
        SlvifWronHoldErrW::new(self, 21)
    }
    #[doc = "Bit 27 - Channel Lock Cleared - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_lock_cleared(&mut self) -> ChLockClearedW<SignalEnableSpec> {
        ChLockClearedW::new(self, 27)
    }
    #[doc = "Bit 28 - Channel Source Suspended - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_src_suspended(&mut self) -> ChSrcSuspendedW<SignalEnableSpec> {
        ChSrcSuspendedW::new(self, 28)
    }
    #[doc = "Bit 29 - Channel Suspended - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_suspended(&mut self) -> ChSuspendedW<SignalEnableSpec> {
        ChSuspendedW::new(self, 29)
    }
    #[doc = "Bit 30 - Channel Disabled - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_disabled(&mut self) -> ChDisabledW<SignalEnableSpec> {
        ChDisabledW::new(self, 30)
    }
    #[doc = "Bit 31 - Channel Aborted - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_aborted(&mut self) -> ChAbortedW<SignalEnableSpec> {
        ChAbortedW::new(self, 31)
    }
}
#[doc = "Channel Interrupt Signal Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`signal_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`signal_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SignalEnableSpec;
impl crate::RegisterSpec for SignalEnableSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`signal_enable::R`](R) reader structure"]
impl crate::Readable for SignalEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`signal_enable::W`](W) writer structure"]
impl crate::Writable for SignalEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets signal_enable to value 0xffff_ffff_ffff_ffff"]
impl crate::Resettable for SignalEnableSpec {
    const RESET_VALUE: u64 = 0xffff_ffff_ffff_ffff;
}
