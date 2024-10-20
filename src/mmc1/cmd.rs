#[doc = "Register `cmd` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `cmd` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `indx` reader - MMC command index"]
pub type IndxR = crate::FieldReader;
#[doc = "Field `indx` writer - MMC command index"]
pub type IndxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `resp(_exp,_long,_crc)` reader - MMC command response"]
pub type RespR = crate::BitReader;
#[doc = "Field `resp(_exp,_long,_crc)` writer - MMC command response"]
pub type RespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dat(_exp,_wr)` reader - MMC command data"]
pub type DatR = crate::BitReader;
#[doc = "Field `dat(_exp,_wr)` writer - MMC command data"]
pub type DatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `strm_mode` reader - MMC command stream mode"]
pub type StrmModeR = crate::BitReader;
#[doc = "Field `strm_mode` writer - MMC command stream mode"]
pub type StrmModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `send_stop` reader - MMC command send stop"]
pub type SendStopR = crate::BitReader;
#[doc = "Field `send_stop` writer - MMC command send stop"]
pub type SendStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prv_data_wait` reader - MMC command private data wait"]
pub type PrvDataWaitR = crate::BitReader;
#[doc = "Field `prv_data_wait` writer - MMC command private data wait"]
pub type PrvDataWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop` reader - MMC command stop"]
pub type StopR = crate::BitReader;
#[doc = "Field `stop` writer - MMC command stop"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `init` reader - MMC command init"]
pub type InitR = crate::BitReader;
#[doc = "Field `init` writer - MMC command init"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `upd_clk` reader - MMC command update clock"]
pub type UpdClkR = crate::BitReader;
#[doc = "Field `upd_clk` writer - MMC command update clock"]
pub type UpdClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ceata_rd` reader - MMC command CEATA read"]
pub type CeataRdR = crate::BitReader;
#[doc = "Field `ceata_rd` writer - MMC command CEATA read"]
pub type CeataRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ccs_exp` reader - MMC command CCS EXP"]
pub type CcsExpR = crate::BitReader;
#[doc = "Field `ccs_exp` writer - MMC command CCS EXP"]
pub type CcsExpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `volt_switch` reader - MMC command volt switch"]
pub type VoltSwitchR = crate::BitReader;
#[doc = "Field `volt_switch` writer - MMC command volt switch"]
pub type VoltSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `use_hold_reg` reader - MMC command use hold register"]
pub type UseHoldRegR = crate::BitReader;
#[doc = "Field `use_hold_reg` writer - MMC command use hold register"]
pub type UseHoldRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `start` reader - MMC command start"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - MMC command start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - MMC command index"]
    #[inline(always)]
    pub fn indx(&self) -> IndxR {
        IndxR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "MMC command response"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `resp_exp` field"]
    #[inline(always)]
    pub fn resp(&self, n: u8) -> RespR {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        RespR::new(((self.bits >> (n + 6)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC command response"]
    #[inline(always)]
    pub fn resp_iter(&self) -> impl Iterator<Item = RespR> + '_ {
        (0..3).map(move |n| RespR::new(((self.bits >> (n + 6)) & 1) != 0))
    }
    #[doc = "Bit 6 - MMC command response"]
    #[inline(always)]
    pub fn resp_exp(&self) -> RespR {
        RespR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC command response"]
    #[inline(always)]
    pub fn resp_long(&self) -> RespR {
        RespR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC command response"]
    #[inline(always)]
    pub fn resp_crc(&self) -> RespR {
        RespR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "MMC command data"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dat_exp` field"]
    #[inline(always)]
    pub fn dat(&self, n: u8) -> DatR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DatR::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC command data"]
    #[inline(always)]
    pub fn dat_iter(&self) -> impl Iterator<Item = DatR> + '_ {
        (0..2).map(move |n| DatR::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - MMC command data"]
    #[inline(always)]
    pub fn dat_exp(&self) -> DatR {
        DatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC command data"]
    #[inline(always)]
    pub fn dat_wr(&self) -> DatR {
        DatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC command stream mode"]
    #[inline(always)]
    pub fn strm_mode(&self) -> StrmModeR {
        StrmModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC command send stop"]
    #[inline(always)]
    pub fn send_stop(&self) -> SendStopR {
        SendStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC command private data wait"]
    #[inline(always)]
    pub fn prv_data_wait(&self) -> PrvDataWaitR {
        PrvDataWaitR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC command stop"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC command init"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC command update clock"]
    #[inline(always)]
    pub fn upd_clk(&self) -> UpdClkR {
        UpdClkR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC command CEATA read"]
    #[inline(always)]
    pub fn ceata_rd(&self) -> CeataRdR {
        CeataRdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC command CCS EXP"]
    #[inline(always)]
    pub fn ccs_exp(&self) -> CcsExpR {
        CcsExpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC command volt switch"]
    #[inline(always)]
    pub fn volt_switch(&self) -> VoltSwitchR {
        VoltSwitchR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC command use hold register"]
    #[inline(always)]
    pub fn use_hold_reg(&self) -> UseHoldRegR {
        UseHoldRegR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC command start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - MMC command index"]
    #[inline(always)]
    #[must_use]
    pub fn indx(&mut self) -> IndxW<CmdSpec> {
        IndxW::new(self, 0)
    }
    #[doc = "MMC command response"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `resp_exp` field"]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self, n: u8) -> RespW<CmdSpec> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        RespW::new(self, n + 6)
    }
    #[doc = "Bit 6 - MMC command response"]
    #[inline(always)]
    #[must_use]
    pub fn resp_exp(&mut self) -> RespW<CmdSpec> {
        RespW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC command response"]
    #[inline(always)]
    #[must_use]
    pub fn resp_long(&mut self) -> RespW<CmdSpec> {
        RespW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC command response"]
    #[inline(always)]
    #[must_use]
    pub fn resp_crc(&mut self) -> RespW<CmdSpec> {
        RespW::new(self, 8)
    }
    #[doc = "MMC command data"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dat_exp` field"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self, n: u8) -> DatW<CmdSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DatW::new(self, n + 9)
    }
    #[doc = "Bit 9 - MMC command data"]
    #[inline(always)]
    #[must_use]
    pub fn dat_exp(&mut self) -> DatW<CmdSpec> {
        DatW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC command data"]
    #[inline(always)]
    #[must_use]
    pub fn dat_wr(&mut self) -> DatW<CmdSpec> {
        DatW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC command stream mode"]
    #[inline(always)]
    #[must_use]
    pub fn strm_mode(&mut self) -> StrmModeW<CmdSpec> {
        StrmModeW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC command send stop"]
    #[inline(always)]
    #[must_use]
    pub fn send_stop(&mut self) -> SendStopW<CmdSpec> {
        SendStopW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC command private data wait"]
    #[inline(always)]
    #[must_use]
    pub fn prv_data_wait(&mut self) -> PrvDataWaitW<CmdSpec> {
        PrvDataWaitW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC command stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<CmdSpec> {
        StopW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC command init"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<CmdSpec> {
        InitW::new(self, 15)
    }
    #[doc = "Bit 21 - MMC command update clock"]
    #[inline(always)]
    #[must_use]
    pub fn upd_clk(&mut self) -> UpdClkW<CmdSpec> {
        UpdClkW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC command CEATA read"]
    #[inline(always)]
    #[must_use]
    pub fn ceata_rd(&mut self) -> CeataRdW<CmdSpec> {
        CeataRdW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC command CCS EXP"]
    #[inline(always)]
    #[must_use]
    pub fn ccs_exp(&mut self) -> CcsExpW<CmdSpec> {
        CcsExpW::new(self, 23)
    }
    #[doc = "Bit 28 - MMC command volt switch"]
    #[inline(always)]
    #[must_use]
    pub fn volt_switch(&mut self) -> VoltSwitchW<CmdSpec> {
        VoltSwitchW::new(self, 28)
    }
    #[doc = "Bit 29 - MMC command use hold register"]
    #[inline(always)]
    #[must_use]
    pub fn use_hold_reg(&mut self) -> UseHoldRegW<CmdSpec> {
        UseHoldRegW::new(self, 29)
    }
    #[doc = "Bit 31 - MMC command start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CmdSpec> {
        StartW::new(self, 31)
    }
}
#[doc = "MMC command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
