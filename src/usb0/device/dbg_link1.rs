#[doc = "Register `dbg_link1` reader"]
pub type R = crate::R<DbgLink1Spec>;
#[doc = "Register `dbg_link1` writer"]
pub type W = crate::W<DbgLink1Spec>;
#[doc = "Field `lfps_min_det_u1_exit` reader - LFPS_MIN_DET_U1_EXIT value - configures the minimum time required for decoding the received LFPS as an LFPS.U1_Exit."]
pub type LfpsMinDetU1ExitR = crate::FieldReader;
#[doc = "Field `lfps_min_det_u1_exit` writer - LFPS_MIN_DET_U1_EXIT value - configures the minimum time required for decoding the received LFPS as an LFPS.U1_Exit."]
pub type LfpsMinDetU1ExitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `lfps_gen_u1_exit` reader - LFPS_MIN_GEN_U1_EXIT value - configures the minimum time for phytxelecidle deassertion when LFPS.U1_Exit."]
pub type LfpsGenU1ExitR = crate::FieldReader;
#[doc = "Field `lfps_gen_u1_exit` writer - LFPS_MIN_GEN_U1_EXIT value - configures the minimum time for phytxelecidle deassertion when LFPS.U1_Exit."]
pub type LfpsGenU1ExitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "RXDET_BREAK_DIS value configures terminating the Far-end Receiver termination detection sequence.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdetBreakDis {
    #[doc = "0: It is possible that USBSS_DEV will terminate far-end receiver termination detection sequence."]
    Possible = 0,
    #[doc = "1: USBSS_DEV will not terminate far-end receiver termination detection sequence."]
    NotPossible = 1,
}
impl From<RxdetBreakDis> for bool {
    #[inline(always)]
    fn from(variant: RxdetBreakDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rxdet_break_dis` reader - RXDET_BREAK_DIS value configures terminating the Far-end Receiver termination detection sequence."]
pub type RxdetBreakDisR = crate::BitReader<RxdetBreakDis>;
impl RxdetBreakDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxdetBreakDis {
        match self.bits {
            false => RxdetBreakDis::Possible,
            true => RxdetBreakDis::NotPossible,
        }
    }
    #[doc = "It is possible that USBSS_DEV will terminate far-end receiver termination detection sequence."]
    #[inline(always)]
    pub fn is_possible(&self) -> bool {
        *self == RxdetBreakDis::Possible
    }
    #[doc = "USBSS_DEV will not terminate far-end receiver termination detection sequence."]
    #[inline(always)]
    pub fn is_not_possible(&self) -> bool {
        *self == RxdetBreakDis::NotPossible
    }
}
#[doc = "Field `rxdet_break_dis` writer - RXDET_BREAK_DIS value configures terminating the Far-end Receiver termination detection sequence."]
pub type RxdetBreakDisW<'a, REG> = crate::BitWriter<'a, REG, RxdetBreakDis>;
impl<'a, REG> RxdetBreakDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "It is possible that USBSS_DEV will terminate far-end receiver termination detection sequence."]
    #[inline(always)]
    pub fn possible(self) -> &'a mut crate::W<REG> {
        self.variant(RxdetBreakDis::Possible)
    }
    #[doc = "USBSS_DEV will not terminate far-end receiver termination detection sequence."]
    #[inline(always)]
    pub fn not_possible(self) -> &'a mut crate::W<REG> {
        self.variant(RxdetBreakDis::NotPossible)
    }
}
#[doc = "Field `lfps_gen_ping` reader - LFPS_GEN_PING value - configures the LFPS.Ping generation."]
pub type LfpsGenPingR = crate::FieldReader;
#[doc = "Field `lfps_gen_ping` writer - LFPS_GEN_PING value - configures the LFPS.Ping generation."]
pub type LfpsGenPingW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Set the LFPS_MIN_DET_U1_EXIT value - This bit is automatically cleared, writing `0` has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfpsMinDetU1ExitSet {
    #[doc = "1: Writes the LPFS_MIN_DET_U1_EXIT field value to the device."]
    Set = 1,
}
impl From<LfpsMinDetU1ExitSet> for bool {
    #[inline(always)]
    fn from(variant: LfpsMinDetU1ExitSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `lfps_min_det_u1_exit_set` reader - Set the LFPS_MIN_DET_U1_EXIT value - This bit is automatically cleared, writing `0` has no effect."]
pub type LfpsMinDetU1ExitSetR = crate::BitReader<LfpsMinDetU1ExitSet>;
impl LfpsMinDetU1ExitSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LfpsMinDetU1ExitSet> {
        match self.bits {
            true => Some(LfpsMinDetU1ExitSet::Set),
            _ => None,
        }
    }
    #[doc = "Writes the LPFS_MIN_DET_U1_EXIT field value to the device."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == LfpsMinDetU1ExitSet::Set
    }
}
#[doc = "Field `lfps_min_det_u1_exit_set` writer - Set the LFPS_MIN_DET_U1_EXIT value - This bit is automatically cleared, writing `0` has no effect."]
pub type LfpsMinDetU1ExitSetW<'a, REG> = crate::BitWriter<'a, REG, LfpsMinDetU1ExitSet>;
impl<'a, REG> LfpsMinDetU1ExitSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes the LPFS_MIN_DET_U1_EXIT field value to the device."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(LfpsMinDetU1ExitSet::Set)
    }
}
#[doc = "Set the LFPS_MIN_GEN_U1_EXIT value - This bit is automatically cleared, writing `0` has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfpsMinGenU1ExitSet {
    #[doc = "1: Writes the LPFS_MIN_GEN_U1_EXIT field value to the device."]
    Set = 1,
}
impl From<LfpsMinGenU1ExitSet> for bool {
    #[inline(always)]
    fn from(variant: LfpsMinGenU1ExitSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `lfps_min_gen_u1_exit_set` writer - Set the LFPS_MIN_GEN_U1_EXIT value - This bit is automatically cleared, writing `0` has no effect."]
pub type LfpsMinGenU1ExitSetW<'a, REG> = crate::BitWriter<'a, REG, LfpsMinGenU1ExitSet>;
impl<'a, REG> LfpsMinGenU1ExitSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes the LPFS_MIN_GEN_U1_EXIT field value to the device."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(LfpsMinGenU1ExitSet::Set)
    }
}
#[doc = "Set the RXDET_BREAK_DIS value - This bit is automatically cleared, writing `0` has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdetBreakDisSet {
    #[doc = "1: Writes the RXDET_BREAK_DIS field value to the device."]
    Set = 1,
}
impl From<RxdetBreakDisSet> for bool {
    #[inline(always)]
    fn from(variant: RxdetBreakDisSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rxdet_break_dis_set` writer - Set the RXDET_BREAK_DIS value - This bit is automatically cleared, writing `0` has no effect."]
pub type RxdetBreakDisSetW<'a, REG> = crate::BitWriter<'a, REG, RxdetBreakDisSet>;
impl<'a, REG> RxdetBreakDisSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes the RXDET_BREAK_DIS field value to the device."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(RxdetBreakDisSet::Set)
    }
}
#[doc = "Set the LFPS_GEN_PING value - This bit is automatically cleared, writing `0` has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfpsGenPingSet {
    #[doc = "1: Writes the LPFS_GEN_PING field value to the device."]
    Set = 1,
}
impl From<LfpsGenPingSet> for bool {
    #[inline(always)]
    fn from(variant: LfpsGenPingSet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `lfps_gen_ping_set` writer - Set the LFPS_GEN_PING value - This bit is automatically cleared, writing `0` has no effect."]
pub type LfpsGenPingSetW<'a, REG> = crate::BitWriter<'a, REG, LfpsGenPingSet>;
impl<'a, REG> LfpsGenPingSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes the LPFS_GEN_PING field value to the device."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(LfpsGenPingSet::Set)
    }
}
impl R {
    #[doc = "Bits 0:7 - LFPS_MIN_DET_U1_EXIT value - configures the minimum time required for decoding the received LFPS as an LFPS.U1_Exit."]
    #[inline(always)]
    pub fn lfps_min_det_u1_exit(&self) -> LfpsMinDetU1ExitR {
        LfpsMinDetU1ExitR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - LFPS_MIN_GEN_U1_EXIT value - configures the minimum time for phytxelecidle deassertion when LFPS.U1_Exit."]
    #[inline(always)]
    pub fn lfps_gen_u1_exit(&self) -> LfpsGenU1ExitR {
        LfpsGenU1ExitR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - RXDET_BREAK_DIS value configures terminating the Far-end Receiver termination detection sequence."]
    #[inline(always)]
    pub fn rxdet_break_dis(&self) -> RxdetBreakDisR {
        RxdetBreakDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - LFPS_GEN_PING value - configures the LFPS.Ping generation."]
    #[inline(always)]
    pub fn lfps_gen_ping(&self) -> LfpsGenPingR {
        LfpsGenPingR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Set the LFPS_MIN_DET_U1_EXIT value - This bit is automatically cleared, writing `0` has no effect."]
    #[inline(always)]
    pub fn lfps_min_det_u1_exit_set(&self) -> LfpsMinDetU1ExitSetR {
        LfpsMinDetU1ExitSetR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - LFPS_MIN_DET_U1_EXIT value - configures the minimum time required for decoding the received LFPS as an LFPS.U1_Exit."]
    #[inline(always)]
    #[must_use]
    pub fn lfps_min_det_u1_exit(&mut self) -> LfpsMinDetU1ExitW<DbgLink1Spec> {
        LfpsMinDetU1ExitW::new(self, 0)
    }
    #[doc = "Bits 8:15 - LFPS_MIN_GEN_U1_EXIT value - configures the minimum time for phytxelecidle deassertion when LFPS.U1_Exit."]
    #[inline(always)]
    #[must_use]
    pub fn lfps_gen_u1_exit(&mut self) -> LfpsGenU1ExitW<DbgLink1Spec> {
        LfpsGenU1ExitW::new(self, 8)
    }
    #[doc = "Bit 16 - RXDET_BREAK_DIS value configures terminating the Far-end Receiver termination detection sequence."]
    #[inline(always)]
    #[must_use]
    pub fn rxdet_break_dis(&mut self) -> RxdetBreakDisW<DbgLink1Spec> {
        RxdetBreakDisW::new(self, 16)
    }
    #[doc = "Bits 17:21 - LFPS_GEN_PING value - configures the LFPS.Ping generation."]
    #[inline(always)]
    #[must_use]
    pub fn lfps_gen_ping(&mut self) -> LfpsGenPingW<DbgLink1Spec> {
        LfpsGenPingW::new(self, 17)
    }
    #[doc = "Bit 24 - Set the LFPS_MIN_DET_U1_EXIT value - This bit is automatically cleared, writing `0` has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn lfps_min_det_u1_exit_set(&mut self) -> LfpsMinDetU1ExitSetW<DbgLink1Spec> {
        LfpsMinDetU1ExitSetW::new(self, 24)
    }
    #[doc = "Bit 25 - Set the LFPS_MIN_GEN_U1_EXIT value - This bit is automatically cleared, writing `0` has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn lfps_min_gen_u1_exit_set(&mut self) -> LfpsMinGenU1ExitSetW<DbgLink1Spec> {
        LfpsMinGenU1ExitSetW::new(self, 25)
    }
    #[doc = "Bit 26 - Set the RXDET_BREAK_DIS value - This bit is automatically cleared, writing `0` has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rxdet_break_dis_set(&mut self) -> RxdetBreakDisSetW<DbgLink1Spec> {
        RxdetBreakDisSetW::new(self, 26)
    }
    #[doc = "Bit 27 - Set the LFPS_GEN_PING value - This bit is automatically cleared, writing `0` has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn lfps_gen_ping_set(&mut self) -> LfpsGenPingSetW<DbgLink1Spec> {
        LfpsGenPingSetW::new(self, 27)
    }
}
#[doc = "Device debug link 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_link1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_link1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgLink1Spec;
impl crate::RegisterSpec for DbgLink1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_link1::R`](R) reader structure"]
impl crate::Readable for DbgLink1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_link1::W`](W) writer structure"]
impl crate::Writable for DbgLink1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dbg_link1 to value 0"]
impl crate::Resettable for DbgLink1Spec {
    const RESET_VALUE: u32 = 0;
}
