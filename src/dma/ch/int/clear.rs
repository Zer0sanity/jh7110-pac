#[doc = "Register `clear` writer"]
pub type W = crate::W<ClearSpec>;
#[doc = "Field `blk_tr_done` writer - Channel Block Transfer Done - 0: disabled, 1: enabled"]
pub type BlkTrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_tr_done` writer - Channel DMA Transfer Done - 0: disabled, 1: enabled"]
pub type DmaTrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `transcomp(_src,_dst)` writer - Channel Transfer Complete - 0: disabled, 1: enabled"]
pub type TranscompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dec_err(_src,_dst)` writer - Channel Decoding Error - 0: disabled, 1: enabled"]
pub type DecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slv_err(_src,_dst)` writer - Channel Slave Error - 0: disabled, 1: enabled"]
pub type SlvErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lli_dec_err(_rd,_wr)` writer - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
pub type LliDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lli_slv_err(_rd,_wr)` writer - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
pub type LliSlvErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `shadow_or_invalid_lli_err` writer - Channel Shadow Register or Linked List Item Invalid Error - 0: disabled, 1: enabled"]
pub type ShadowOrInvalidLliErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_multiblktype_err` writer - Channel Slave Interface Multi Block Type Error - 0: disabled, 1: enabled"]
pub type SlvifMultiblktypeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_dec_err` writer - Channel Slave Interface Decoding Error - 0: disabled, 1: enabled"]
pub type SlvifDecErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wr2ro_err` writer - Channel Slave Interface Write to Read-only Error - 0: disabled, 1: enabled"]
pub type SlvifWr2roErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_r2wro_err` writer - Channel Slave Interface Read to Write-only Error - 0: disabled, 1: enabled"]
pub type SlvifR2wroErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wronchen_err` writer - Channel Slave Interface Write On Channel Enabled Error - 0: disabled, 1: enabled"]
pub type SlvifWronchenErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_shadow_wron_valid_err` writer - Channel Slave Interface Shadow Register Write On Valid Error - 0: disabled, 1: enabled"]
pub type SlvifShadowWronValidErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slvif_wron_hold_err` writer - Channel Slave Interface Write On Hold Error - 0: disabled, 1: enabled"]
pub type SlvifWronHoldErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_lock_cleared` writer - Channel Lock Cleared - 0: disabled, 1: enabled"]
pub type ChLockClearedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_src_suspended` writer - Channel Source Suspended - 0: disabled, 1: enabled"]
pub type ChSrcSuspendedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_suspended` writer - Channel Suspended - 0: disabled, 1: enabled"]
pub type ChSuspendedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_disabled` writer - Channel Disabled - 0: disabled, 1: enabled"]
pub type ChDisabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_aborted` writer - Channel Aborted - 0: disabled, 1: enabled"]
pub type ChAbortedW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel Block Transfer Done - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn blk_tr_done(&mut self) -> BlkTrDoneW<ClearSpec> {
        BlkTrDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel DMA Transfer Done - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dma_tr_done(&mut self) -> DmaTrDoneW<ClearSpec> {
        DmaTrDoneW::new(self, 1)
    }
    #[doc = "Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `transcomp_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn transcomp(&mut self, n: u8) -> TranscompW<ClearSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TranscompW::new(self, n + 3)
    }
    #[doc = "Bit 3 - Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn transcomp_src(&mut self) -> TranscompW<ClearSpec> {
        TranscompW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Transfer Complete - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn transcomp_dst(&mut self) -> TranscompW<ClearSpec> {
        TranscompW::new(self, 4)
    }
    #[doc = "Channel Decoding Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dec_err_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn dec_err(&mut self, n: u8) -> DecErrW<ClearSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DecErrW::new(self, n + 5)
    }
    #[doc = "Bit 5 - Channel Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dec_err_src(&mut self) -> DecErrW<ClearSpec> {
        DecErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dec_err_dst(&mut self) -> DecErrW<ClearSpec> {
        DecErrW::new(self, 6)
    }
    #[doc = "Channel Slave Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slv_err_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn slv_err(&mut self, n: u8) -> SlvErrW<ClearSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SlvErrW::new(self, n + 7)
    }
    #[doc = "Bit 7 - Channel Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slv_err_src(&mut self) -> SlvErrW<ClearSpec> {
        SlvErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slv_err_dst(&mut self) -> SlvErrW<ClearSpec> {
        SlvErrW::new(self, 8)
    }
    #[doc = "Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lli_dec_err_rd` field"]
    #[inline(always)]
    #[must_use]
    pub fn lli_dec_err(&mut self, n: u8) -> LliDecErrW<ClearSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        LliDecErrW::new(self, n + 9)
    }
    #[doc = "Bit 9 - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lli_dec_err_rd(&mut self) -> LliDecErrW<ClearSpec> {
        LliDecErrW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel Linked List Item Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lli_dec_err_wr(&mut self) -> LliDecErrW<ClearSpec> {
        LliDecErrW::new(self, 10)
    }
    #[doc = "Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lli_slv_err_rd` field"]
    #[inline(always)]
    #[must_use]
    pub fn lli_slv_err(&mut self, n: u8) -> LliSlvErrW<ClearSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        LliSlvErrW::new(self, n + 11)
    }
    #[doc = "Bit 11 - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lli_slv_err_rd(&mut self) -> LliSlvErrW<ClearSpec> {
        LliSlvErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel Linked List Item Slave Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn lli_slv_err_wr(&mut self) -> LliSlvErrW<ClearSpec> {
        LliSlvErrW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel Shadow Register or Linked List Item Invalid Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn shadow_or_invalid_lli_err(&mut self) -> ShadowOrInvalidLliErrW<ClearSpec> {
        ShadowOrInvalidLliErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel Slave Interface Multi Block Type Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_multiblktype_err(&mut self) -> SlvifMultiblktypeErrW<ClearSpec> {
        SlvifMultiblktypeErrW::new(self, 14)
    }
    #[doc = "Bit 16 - Channel Slave Interface Decoding Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_dec_err(&mut self) -> SlvifDecErrW<ClearSpec> {
        SlvifDecErrW::new(self, 16)
    }
    #[doc = "Bit 17 - Channel Slave Interface Write to Read-only Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wr2ro_err(&mut self) -> SlvifWr2roErrW<ClearSpec> {
        SlvifWr2roErrW::new(self, 17)
    }
    #[doc = "Bit 18 - Channel Slave Interface Read to Write-only Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_r2wro_err(&mut self) -> SlvifR2wroErrW<ClearSpec> {
        SlvifR2wroErrW::new(self, 18)
    }
    #[doc = "Bit 19 - Channel Slave Interface Write On Channel Enabled Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wronchen_err(&mut self) -> SlvifWronchenErrW<ClearSpec> {
        SlvifWronchenErrW::new(self, 19)
    }
    #[doc = "Bit 20 - Channel Slave Interface Shadow Register Write On Valid Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_shadow_wron_valid_err(&mut self) -> SlvifShadowWronValidErrW<ClearSpec> {
        SlvifShadowWronValidErrW::new(self, 20)
    }
    #[doc = "Bit 21 - Channel Slave Interface Write On Hold Error - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn slvif_wron_hold_err(&mut self) -> SlvifWronHoldErrW<ClearSpec> {
        SlvifWronHoldErrW::new(self, 21)
    }
    #[doc = "Bit 27 - Channel Lock Cleared - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_lock_cleared(&mut self) -> ChLockClearedW<ClearSpec> {
        ChLockClearedW::new(self, 27)
    }
    #[doc = "Bit 28 - Channel Source Suspended - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_src_suspended(&mut self) -> ChSrcSuspendedW<ClearSpec> {
        ChSrcSuspendedW::new(self, 28)
    }
    #[doc = "Bit 29 - Channel Suspended - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_suspended(&mut self) -> ChSuspendedW<ClearSpec> {
        ChSuspendedW::new(self, 29)
    }
    #[doc = "Bit 30 - Channel Disabled - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_disabled(&mut self) -> ChDisabledW<ClearSpec> {
        ChDisabledW::new(self, 30)
    }
    #[doc = "Bit 31 - Channel Aborted - 0: disabled, 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ch_aborted(&mut self) -> ChAbortedW<ClearSpec> {
        ChAbortedW::new(self, 31)
    }
}
#[doc = "Channel Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearSpec;
impl crate::RegisterSpec for ClearSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for ClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets clear to value 0xffff_ffff_ffff_ffff"]
impl crate::Resettable for ClearSpec {
    const RESET_VALUE: u64 = 0xffff_ffff_ffff_ffff;
}
