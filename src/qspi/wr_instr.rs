#[doc = "Register `wr_instr` reader"]
pub type R = crate::R<WrInstrSpec>;
#[doc = "Register `wr_instr` writer"]
pub type W = crate::W<WrInstrSpec>;
#[doc = "Field `opcode` reader - Instruction Opcode"]
pub type OpcodeR = crate::FieldReader;
#[doc = "Field `opcode` writer - Instruction Opcode"]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `type_addr` reader - Type of Address"]
pub type TypeAddrR = crate::FieldReader;
#[doc = "Field `type_addr` writer - Type of Address"]
pub type TypeAddrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `type_data` reader - "]
pub type TypeDataR = crate::FieldReader;
#[doc = "Field `type_data` writer - "]
pub type TypeDataW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Instruction Opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new((self.bits & 0xff) as u8)
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
}
impl W {
    #[doc = "Bits 0:7 - Instruction Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn opcode(&mut self) -> OpcodeW<WrInstrSpec> {
        OpcodeW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Type of Address"]
    #[inline(always)]
    #[must_use]
    pub fn type_addr(&mut self) -> TypeAddrW<WrInstrSpec> {
        TypeAddrW::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn type_data(&mut self) -> TypeDataW<WrInstrSpec> {
        TypeDataW::new(self, 16)
    }
}
#[doc = "Cadence QSPI Write Instruction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_instr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_instr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrInstrSpec;
impl crate::RegisterSpec for WrInstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_instr::R`](R) reader structure"]
impl crate::Readable for WrInstrSpec {}
#[doc = "`write(|w| ..)` method takes [`wr_instr::W`](W) writer structure"]
impl crate::Writable for WrInstrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wr_instr to value 0"]
impl crate::Resettable for WrInstrSpec {
    const RESET_VALUE: u32 = 0;
}
