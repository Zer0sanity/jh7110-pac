#[doc = "Register `config` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `config` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `enable` reader - Enable the QSPI controller"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Enable the QSPI controller"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enb_dir_acc_ctrl` reader - Enable direct access controller"]
pub type EnbDirAccCtrlR = crate::BitReader;
#[doc = "Field `enb_dir_acc_ctrl` writer - Enable direct access controller"]
pub type EnbDirAccCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `decode` reader - Enable the QSPI decoder"]
pub type DecodeR = crate::BitReader;
#[doc = "Field `decode` writer - Enable the QSPI decoder"]
pub type DecodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chipselect` reader - Chip select - CS0: 0b1110, CS1: 0b1101, CS2: 0b1011, CS3: 0b0111"]
pub type ChipselectR = crate::FieldReader;
#[doc = "Field `chipselect` writer - Chip select - CS0: 0b1110, CS1: 0b1101, CS2: 0b1011, CS3: 0b0111"]
pub type ChipselectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dma` reader - Enable Direct Memory Access"]
pub type DmaR = crate::BitReader;
#[doc = "Field `dma` writer - Enable Direct Memory Access"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `baud` reader - Set the QSPI BAUD rate divisor"]
pub type BaudR = crate::FieldReader;
#[doc = "Field `baud` writer - Set the QSPI BAUD rate divisor"]
pub type BaudW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dtr_proto` reader - Enable DTR Protocol"]
pub type DtrProtoR = crate::BitReader;
#[doc = "Field `dtr_proto` writer - Enable DTR Protocol"]
pub type DtrProtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dual_opcode` reader - Enable Dual Opcode Mode"]
pub type DualOpcodeR = crate::BitReader;
#[doc = "Field `dual_opcode` writer - Enable Dual Opcode Mode"]
pub type DualOpcodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `idle` reader - Set Idle"]
pub type IdleR = crate::BitReader;
#[doc = "Field `idle` writer - Set Idle"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the QSPI controller"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Enable direct access controller"]
    #[inline(always)]
    pub fn enb_dir_acc_ctrl(&self) -> EnbDirAccCtrlR {
        EnbDirAccCtrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the QSPI decoder"]
    #[inline(always)]
    pub fn decode(&self) -> DecodeR {
        DecodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Chip select - CS0: 0b1110, CS1: 0b1101, CS2: 0b1011, CS3: 0b0111"]
    #[inline(always)]
    pub fn chipselect(&self) -> ChipselectR {
        ChipselectR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Enable Direct Memory Access"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Set the QSPI BAUD rate divisor"]
    #[inline(always)]
    pub fn baud(&self) -> BaudR {
        BaudR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn dtr_proto(&self) -> DtrProtoR {
        DtrProtoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Dual Opcode Mode"]
    #[inline(always)]
    pub fn dual_opcode(&self) -> DualOpcodeR {
        DualOpcodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the QSPI controller"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ConfigSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 7 - Enable direct access controller"]
    #[inline(always)]
    #[must_use]
    pub fn enb_dir_acc_ctrl(&mut self) -> EnbDirAccCtrlW<ConfigSpec> {
        EnbDirAccCtrlW::new(self, 7)
    }
    #[doc = "Bit 9 - Enable the QSPI decoder"]
    #[inline(always)]
    #[must_use]
    pub fn decode(&mut self) -> DecodeW<ConfigSpec> {
        DecodeW::new(self, 9)
    }
    #[doc = "Bits 10:13 - Chip select - CS0: 0b1110, CS1: 0b1101, CS2: 0b1011, CS3: 0b0111"]
    #[inline(always)]
    #[must_use]
    pub fn chipselect(&mut self) -> ChipselectW<ConfigSpec> {
        ChipselectW::new(self, 10)
    }
    #[doc = "Bit 15 - Enable Direct Memory Access"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<ConfigSpec> {
        DmaW::new(self, 15)
    }
    #[doc = "Bits 19:22 - Set the QSPI BAUD rate divisor"]
    #[inline(always)]
    #[must_use]
    pub fn baud(&mut self) -> BaudW<ConfigSpec> {
        BaudW::new(self, 19)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    #[must_use]
    pub fn dtr_proto(&mut self) -> DtrProtoW<ConfigSpec> {
        DtrProtoW::new(self, 24)
    }
    #[doc = "Bit 30 - Enable Dual Opcode Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dual_opcode(&mut self) -> DualOpcodeW<ConfigSpec> {
        DualOpcodeW::new(self, 30)
    }
    #[doc = "Bit 31 - Set Idle"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<ConfigSpec> {
        IdleW::new(self, 31)
    }
}
#[doc = "Cadence QSPI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
