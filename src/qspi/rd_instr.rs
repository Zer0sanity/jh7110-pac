#[doc = "Register `rd_instr` reader"]
pub type R = crate::R<RdInstrSpec>;
#[doc = "Register `rd_instr` writer"]
pub type W = crate::W<RdInstrSpec>;
#[doc = "Field `opcode` reader - Instruction Opcode"]
pub type OpcodeR = crate::FieldReader;
#[doc = "Field `opcode` writer - Instruction Opcode"]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `type_instr` reader - Type of Instruction"]
pub type TypeInstrR = crate::FieldReader;
#[doc = "Field `type_instr` writer - Type of Instruction"]
pub type TypeInstrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `type_addr` reader - Type of Address"]
pub type TypeAddrR = crate::FieldReader;
#[doc = "Field `type_addr` writer - Type of Address"]
pub type TypeAddrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `type_data` reader - "]
pub type TypeDataR = crate::FieldReader;
#[doc = "Field `type_data` writer - "]
pub type TypeDataW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mode_en` reader - Mode enable"]
pub type ModeEnR = crate::BitReader;
#[doc = "Field `mode_en` writer - Mode enable"]
pub type ModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy` reader - Send dummy signal to stall the device"]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - Send dummy signal to stall the device"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Instruction Opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Type of Instruction"]
    #[inline(always)]
    pub fn type_instr(&self) -> TypeInstrR {
        TypeInstrR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Type of Address"]
    #[inline(always)]
    pub fn type_addr(&self) -> TypeAddrR {
        TypeAddrR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn type_data(&self) -> TypeDataR {
        TypeDataR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Mode enable"]
    #[inline(always)]
    pub fn mode_en(&self) -> ModeEnR {
        ModeEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Send dummy signal to stall the device"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OpcodeW<RdInstrSpec> {
        OpcodeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Type of Instruction"]
    #[inline(always)]
    #[must_use]
    pub fn type_instr(&mut self) -> TypeInstrW<RdInstrSpec> {
        TypeInstrW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Type of Address"]
    #[inline(always)]
    #[must_use]
    pub fn type_addr(&mut self) -> TypeAddrW<RdInstrSpec> {
        TypeAddrW::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn type_data(&mut self) -> TypeDataW<RdInstrSpec> {
        TypeDataW::new(self, 16)
    }
    #[doc = "Bit 20 - Mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mode_en(&mut self) -> ModeEnW<RdInstrSpec> {
        ModeEnW::new(self, 20)
    }
    #[doc = "Bits 24:28 - Send dummy signal to stall the device"]
    #[inline(always)]
    #[must_use]
    pub fn dummy(&mut self) -> DummyW<RdInstrSpec> {
        DummyW::new(self, 24)
    }
}
#[doc = "Cadence QSPI Read Instruction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_instr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_instr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdInstrSpec;
impl crate::RegisterSpec for RdInstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_instr::R`](R) reader structure"]
impl crate::Readable for RdInstrSpec {}
#[doc = "`write(|w| ..)` method takes [`rd_instr::W`](W) writer structure"]
impl crate::Writable for RdInstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rd_instr to value 0"]
impl crate::Resettable for RdInstrSpec {
    const RESET_VALUE: u32 = 0;
}
