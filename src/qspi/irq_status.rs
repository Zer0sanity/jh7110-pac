#[doc = "Register `irq_status` reader"]
pub type R = crate::R<IrqStatusSpec>;
#[doc = "Register `irq_status` writer"]
pub type W = crate::W<IrqStatusSpec>;
#[doc = "Field `mode_err` reader - Mode error interrupt"]
pub type ModeErrR = crate::BitReader;
#[doc = "Field `mode_err` writer - Mode error interrupt"]
pub type ModeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `underflow` reader - Buffer underflow interrupt"]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `underflow` writer - Buffer underflow interrupt"]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ind_comp` reader - Indirect computation interrupt"]
pub type IndCompR = crate::BitReader;
#[doc = "Field `ind_comp` writer - Indirect computation interrupt"]
pub type IndCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ind_rd_reject` reader - Indirect read rejection interrupt"]
pub type IndRdRejectR = crate::BitReader;
#[doc = "Field `ind_rd_reject` writer - Indirect read rejection interrupt"]
pub type IndRdRejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_protected_err` reader - Write protected error interrupt"]
pub type WrProtectedErrR = crate::BitReader;
#[doc = "Field `wr_protected_err` writer - Write protected error interrupt"]
pub type WrProtectedErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `illegal_ahb_err` reader - Illegal AHB clock error interrupt"]
pub type IllegalAhbErrR = crate::BitReader;
#[doc = "Field `illegal_ahb_err` writer - Illegal AHB clock error interrupt"]
pub type IllegalAhbErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `watermark` reader - Watermark interrupt"]
pub type WatermarkR = crate::BitReader;
#[doc = "Field `watermark` writer - Watermark interrupt"]
pub type WatermarkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ind_sram_full` reader - Indirect SRAM full interrupt"]
pub type IndSramFullR = crate::BitReader;
#[doc = "Field `ind_sram_full` writer - Indirect SRAM full interrupt"]
pub type IndSramFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mode error interrupt"]
    #[inline(always)]
    pub fn mode_err(&self) -> ModeErrR {
        ModeErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer underflow interrupt"]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indirect computation interrupt"]
    #[inline(always)]
    pub fn ind_comp(&self) -> IndCompR {
        IndCompR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indirect read rejection interrupt"]
    #[inline(always)]
    pub fn ind_rd_reject(&self) -> IndRdRejectR {
        IndRdRejectR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protected error interrupt"]
    #[inline(always)]
    pub fn wr_protected_err(&self) -> WrProtectedErrR {
        WrProtectedErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Illegal AHB clock error interrupt"]
    #[inline(always)]
    pub fn illegal_ahb_err(&self) -> IllegalAhbErrR {
        IllegalAhbErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watermark interrupt"]
    #[inline(always)]
    pub fn watermark(&self) -> WatermarkR {
        WatermarkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Indirect SRAM full interrupt"]
    #[inline(always)]
    pub fn ind_sram_full(&self) -> IndSramFullR {
        IndSramFullR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mode_err(&mut self) -> ModeErrW<IrqStatusSpec> {
        ModeErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer underflow interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UnderflowW<IrqStatusSpec> {
        UnderflowW::new(self, 1)
    }
    #[doc = "Bit 2 - Indirect computation interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ind_comp(&mut self) -> IndCompW<IrqStatusSpec> {
        IndCompW::new(self, 2)
    }
    #[doc = "Bit 3 - Indirect read rejection interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ind_rd_reject(&mut self) -> IndRdRejectW<IrqStatusSpec> {
        IndRdRejectW::new(self, 3)
    }
    #[doc = "Bit 4 - Write protected error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wr_protected_err(&mut self) -> WrProtectedErrW<IrqStatusSpec> {
        WrProtectedErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Illegal AHB clock error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn illegal_ahb_err(&mut self) -> IllegalAhbErrW<IrqStatusSpec> {
        IllegalAhbErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Watermark interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn watermark(&mut self) -> WatermarkW<IrqStatusSpec> {
        WatermarkW::new(self, 6)
    }
    #[doc = "Bit 12 - Indirect SRAM full interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ind_sram_full(&mut self) -> IndSramFullW<IrqStatusSpec> {
        IndSramFullW::new(self, 12)
    }
}
#[doc = "Cadence QSPI IRQ Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqStatusSpec;
impl crate::RegisterSpec for IrqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_status::R`](R) reader structure"]
impl crate::Readable for IrqStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_status::W`](W) writer structure"]
impl crate::Writable for IrqStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets irq_status to value 0x0001_ffff"]
impl crate::Resettable for IrqStatusSpec {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
