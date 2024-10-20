#[doc = "Register `cacr` reader"]
pub type R = crate::R<CacrSpec>;
#[doc = "Register `cacr` writer"]
pub type W = crate::W<CacrSpec>;
#[doc = "Field `start` reader - Crypto Start"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - Crypto Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reset` reader - Crypto Reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `reset` writer - Crypto Reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ie` reader - Crypto Interrupt Enable"]
pub type IeR = crate::BitReader;
#[doc = "Field `ie` writer - Crypto Interrupt Enable"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo_mode` reader - Crypto FIFO Mode"]
pub type FifoModeR = crate::BitReader;
#[doc = "Field `fifo_mode` writer - Crypto FIFO Mode"]
pub type FifoModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `not_r2` reader - Crypto Not R2"]
pub type NotR2R = crate::BitReader;
#[doc = "Field `not_r2` writer - Crypto Not R2"]
pub type NotR2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ecc_sub` reader - Crypto ECC Sub"]
pub type EccSubR = crate::BitReader;
#[doc = "Field `ecc_sub` writer - Crypto ECC Sub"]
pub type EccSubW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pre_expf` reader - Crypto Pre EXPF"]
pub type PreExpfR = crate::BitReader;
#[doc = "Field `pre_expf` writer - Crypto Pre EXPF"]
pub type PreExpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmd` reader - Crypto Command - 0: PRE (R^2 mod N and N0'), 1: AAN ((A + A) mod N, ==> A), 2: AMEN (A ^ E mod N ==> A), 3: AAEN (A + E mod N ==> A), 4: ADEN (A - E mod N ==> A), 5: ARN (A * $ mod N ==> A), 6: AERN (A * E * R mod N ==> A), 7: AARN (A * A * R mod N ==> A), 8: ECC2P (ECC2P ==> A), 9: ECCPQ (ECCPQ ==> A)"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `cmd` writer - Crypto Command - 0: PRE (R^2 mod N and N0'), 1: AAN ((A + A) mod N, ==> A), 2: AMEN (A ^ E mod N ==> A), 3: AAEN (A + E mod N ==> A), 4: ADEN (A - E mod N ==> A), 5: ARN (A * $ mod N ==> A), 6: AERN (A * E * R mod N ==> A), 7: AARN (A * A * R mod N ==> A), 8: ECC2P (ECC2P ==> A), 9: ECCPQ (ECCPQ ==> A)"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ctrl_dummy` reader - Crypto Control Dummy"]
pub type CtrlDummyR = crate::BitReader;
#[doc = "Field `ctrl_dummy` writer - Crypto Control Dummy"]
pub type CtrlDummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctrl_false` reader - Crypto Control False"]
pub type CtrlFalseR = crate::BitReader;
#[doc = "Field `ctrl_false` writer - Crypto Control False"]
pub type CtrlFalseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cln_done` reader - Crypto CLN Done"]
pub type ClnDoneR = crate::BitReader;
#[doc = "Field `cln_done` writer - Crypto CLN Done"]
pub type ClnDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `opsize` reader - Crypto OPSIZE"]
pub type OpsizeR = crate::FieldReader;
#[doc = "Field `opsize` writer - Crypto OPSIZE"]
pub type OpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `exposize` reader - Crypto EXPOSIZE"]
pub type ExposizeR = crate::FieldReader;
#[doc = "Field `exposize` writer - Crypto EXPOSIZE"]
pub type ExposizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `bigendian` reader - Crypto Big Endian"]
pub type BigendianR = crate::BitReader;
#[doc = "Field `bigendian` writer - Crypto Big Endian"]
pub type BigendianW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Crypto Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Crypto Reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crypto Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Crypto FIFO Mode"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FifoModeR {
        FifoModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Crypto Not R2"]
    #[inline(always)]
    pub fn not_r2(&self) -> NotR2R {
        NotR2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Crypto ECC Sub"]
    #[inline(always)]
    pub fn ecc_sub(&self) -> EccSubR {
        EccSubR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Crypto Pre EXPF"]
    #[inline(always)]
    pub fn pre_expf(&self) -> PreExpfR {
        PreExpfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Crypto Command - 0: PRE (R^2 mod N and N0'), 1: AAN ((A + A) mod N, ==> A), 2: AMEN (A ^ E mod N ==> A), 3: AAEN (A + E mod N ==> A), 4: ADEN (A - E mod N ==> A), 5: ARN (A * $ mod N ==> A), 6: AERN (A * E * R mod N ==> A), 7: AARN (A * A * R mod N ==> A), 8: ECC2P (ECC2P ==> A), 9: ECCPQ (ECCPQ ==> A)"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Crypto Control Dummy"]
    #[inline(always)]
    pub fn ctrl_dummy(&self) -> CtrlDummyR {
        CtrlDummyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Crypto Control False"]
    #[inline(always)]
    pub fn ctrl_false(&self) -> CtrlFalseR {
        CtrlFalseR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Crypto CLN Done"]
    #[inline(always)]
    pub fn cln_done(&self) -> ClnDoneR {
        ClnDoneR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Crypto OPSIZE"]
    #[inline(always)]
    pub fn opsize(&self) -> OpsizeR {
        OpsizeR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Crypto EXPOSIZE"]
    #[inline(always)]
    pub fn exposize(&self) -> ExposizeR {
        ExposizeR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Crypto Big Endian"]
    #[inline(always)]
    pub fn bigendian(&self) -> BigendianR {
        BigendianR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Crypto Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CacrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Crypto Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CacrSpec> {
        ResetW::new(self, 1)
    }
    #[doc = "Bit 2 - Crypto Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<CacrSpec> {
        IeW::new(self, 2)
    }
    #[doc = "Bit 4 - Crypto FIFO Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_mode(&mut self) -> FifoModeW<CacrSpec> {
        FifoModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Crypto Not R2"]
    #[inline(always)]
    #[must_use]
    pub fn not_r2(&mut self) -> NotR2W<CacrSpec> {
        NotR2W::new(self, 5)
    }
    #[doc = "Bit 6 - Crypto ECC Sub"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_sub(&mut self) -> EccSubW<CacrSpec> {
        EccSubW::new(self, 6)
    }
    #[doc = "Bit 7 - Crypto Pre EXPF"]
    #[inline(always)]
    #[must_use]
    pub fn pre_expf(&mut self) -> PreExpfW<CacrSpec> {
        PreExpfW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Crypto Command - 0: PRE (R^2 mod N and N0'), 1: AAN ((A + A) mod N, ==> A), 2: AMEN (A ^ E mod N ==> A), 3: AAEN (A + E mod N ==> A), 4: ADEN (A - E mod N ==> A), 5: ARN (A * $ mod N ==> A), 6: AERN (A * E * R mod N ==> A), 7: AARN (A * A * R mod N ==> A), 8: ECC2P (ECC2P ==> A), 9: ECCPQ (ECCPQ ==> A)"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CacrSpec> {
        CmdW::new(self, 8)
    }
    #[doc = "Bit 13 - Crypto Control Dummy"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_dummy(&mut self) -> CtrlDummyW<CacrSpec> {
        CtrlDummyW::new(self, 13)
    }
    #[doc = "Bit 14 - Crypto Control False"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_false(&mut self) -> CtrlFalseW<CacrSpec> {
        CtrlFalseW::new(self, 14)
    }
    #[doc = "Bit 15 - Crypto CLN Done"]
    #[inline(always)]
    #[must_use]
    pub fn cln_done(&mut self) -> ClnDoneW<CacrSpec> {
        ClnDoneW::new(self, 15)
    }
    #[doc = "Bits 16:21 - Crypto OPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn opsize(&mut self) -> OpsizeW<CacrSpec> {
        OpsizeW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Crypto EXPOSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn exposize(&mut self) -> ExposizeW<CacrSpec> {
        ExposizeW::new(self, 24)
    }
    #[doc = "Bit 31 - Crypto Big Endian"]
    #[inline(always)]
    #[must_use]
    pub fn bigendian(&mut self) -> BigendianW<CacrSpec> {
        BigendianW::new(self, 31)
    }
}
#[doc = "JH7110 Crypto CA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacrSpec;
impl crate::RegisterSpec for CacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cacr::R`](R) reader structure"]
impl crate::Readable for CacrSpec {}
#[doc = "`write(|w| ..)` method takes [`cacr::W`](W) writer structure"]
impl crate::Writable for CacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cacr to value 0"]
impl crate::Resettable for CacrSpec {
    const RESET_VALUE: u32 = 0;
}
