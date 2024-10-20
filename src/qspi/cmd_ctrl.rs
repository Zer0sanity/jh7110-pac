#[doc = "Register `cmd_ctrl` reader"]
pub type R = crate::R<CmdCtrlSpec>;
#[doc = "Register `cmd_ctrl` writer"]
pub type W = crate::W<CmdCtrlSpec>;
#[doc = "Field `execute` reader - Execute-in-Place (XIP)"]
pub type ExecuteR = crate::BitReader;
#[doc = "Field `execute` writer - Execute-in-Place (XIP)"]
pub type ExecuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `in_progress` reader - Command in progress"]
pub type InProgressR = crate::BitReader;
#[doc = "Field `in_progress` writer - Command in progress"]
pub type InProgressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - Dummy command"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - Dummy command"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `wr_bytes` reader - Write bytes"]
pub type WrBytesR = crate::FieldReader;
#[doc = "Field `wr_bytes` writer - Write bytes"]
pub type WrBytesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `wr_en` reader - Write enable"]
pub type WrEnR = crate::BitReader;
#[doc = "Field `wr_en` writer - Write enable"]
pub type WrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `add_bytes` reader - Add command bytes"]
pub type AddBytesR = crate::FieldReader;
#[doc = "Field `add_bytes` writer - Add command bytes"]
pub type AddBytesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `addr_en` reader - Address enable"]
pub type AddrEnR = crate::BitReader;
#[doc = "Field `addr_en` writer - Address enable"]
pub type AddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rd_bytes` reader - Read bytes"]
pub type RdBytesR = crate::FieldReader;
#[doc = "Field `rd_bytes` writer - Read bytes"]
pub type RdBytesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rd_en` reader - Read enable"]
pub type RdEnR = crate::BitReader;
#[doc = "Field `rd_en` writer - Read enable"]
pub type RdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `opcode` reader - Command opcode"]
pub type OpcodeR = crate::FieldReader;
#[doc = "Field `opcode` writer - Command opcode"]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Execute-in-Place (XIP)"]
    #[inline(always)]
    pub fn execute(&self) -> ExecuteR {
        ExecuteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command in progress"]
    #[inline(always)]
    pub fn in_progress(&self) -> InProgressR {
        InProgressR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Dummy command"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - Write bytes"]
    #[inline(always)]
    pub fn wr_bytes(&self) -> WrBytesR {
        WrBytesR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Write enable"]
    #[inline(always)]
    pub fn wr_en(&self) -> WrEnR {
        WrEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Add command bytes"]
    #[inline(always)]
    pub fn add_bytes(&self) -> AddBytesR {
        AddBytesR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Address enable"]
    #[inline(always)]
    pub fn addr_en(&self) -> AddrEnR {
        AddrEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Read bytes"]
    #[inline(always)]
    pub fn rd_bytes(&self) -> RdBytesR {
        RdBytesR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Read enable"]
    #[inline(always)]
    pub fn rd_en(&self) -> RdEnR {
        RdEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Command opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Execute-in-Place (XIP)"]
    #[inline(always)]
    #[must_use]
    pub fn execute(&mut self) -> ExecuteW<CmdCtrlSpec> {
        ExecuteW::new(self, 0)
    }
    #[doc = "Bit 1 - Command in progress"]
    #[inline(always)]
    #[must_use]
    pub fn in_progress(&mut self) -> InProgressW<CmdCtrlSpec> {
        InProgressW::new(self, 1)
    }
    #[doc = "Bits 7:11 - Dummy command"]
    #[inline(always)]
    #[must_use]
    pub fn dummy(&mut self) -> DummyW<CmdCtrlSpec> {
        DummyW::new(self, 7)
    }
    #[doc = "Bits 12:14 - Write bytes"]
    #[inline(always)]
    #[must_use]
    pub fn wr_bytes(&mut self) -> WrBytesW<CmdCtrlSpec> {
        WrBytesW::new(self, 12)
    }
    #[doc = "Bit 15 - Write enable"]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WrEnW<CmdCtrlSpec> {
        WrEnW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Add command bytes"]
    #[inline(always)]
    #[must_use]
    pub fn add_bytes(&mut self) -> AddBytesW<CmdCtrlSpec> {
        AddBytesW::new(self, 16)
    }
    #[doc = "Bit 19 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn addr_en(&mut self) -> AddrEnW<CmdCtrlSpec> {
        AddrEnW::new(self, 19)
    }
    #[doc = "Bits 20:22 - Read bytes"]
    #[inline(always)]
    #[must_use]
    pub fn rd_bytes(&mut self) -> RdBytesW<CmdCtrlSpec> {
        RdBytesW::new(self, 20)
    }
    #[doc = "Bit 23 - Read enable"]
    #[inline(always)]
    #[must_use]
    pub fn rd_en(&mut self) -> RdEnW<CmdCtrlSpec> {
        RdEnW::new(self, 23)
    }
    #[doc = "Bits 24:31 - Command opcode"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OpcodeW<CmdCtrlSpec> {
        OpcodeW::new(self, 24)
    }
}
#[doc = "Cadence QSPI Command Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdCtrlSpec;
impl crate::RegisterSpec for CmdCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_ctrl::R`](R) reader structure"]
impl crate::Readable for CmdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_ctrl::W`](W) writer structure"]
impl crate::Writable for CmdCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd_ctrl to value 0"]
impl crate::Resettable for CmdCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
