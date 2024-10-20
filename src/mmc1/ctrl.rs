#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `reset(_mmc,_fifo,_dma)` reader - MMC Control Reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `reset(_mmc,_fifo,_dma)` writer - MMC Control Reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable(_int,_dma)` reader - MMC Control Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable(_int,_dma)` writer - MMC Control Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `read_wait` reader - MMC Control Read Wait"]
pub type ReadWaitR = crate::BitReader;
#[doc = "Field `read_wait` writer - MMC Control Read Wait"]
pub type ReadWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `send_irq_resp` reader - MMC Control Send IRQ Response"]
pub type SendIrqRespR = crate::BitReader;
#[doc = "Field `send_irq_resp` writer - MMC Control Send IRQ Response"]
pub type SendIrqRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abrt_read_data` reader - MMC Control Abort Read Data"]
pub type AbrtReadDataR = crate::BitReader;
#[doc = "Field `abrt_read_data` writer - MMC Control Abort Read Data"]
pub type AbrtReadDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `send(_ccsd,_as_ccsd)` reader - MMC Control Send"]
pub type SendR = crate::BitReader;
#[doc = "Field `send(_ccsd,_as_ccsd)` writer - MMC Control Send"]
pub type SendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ceata_int_en` reader - MMC Control CEATA Interrupt Enable"]
pub type CeataIntEnR = crate::BitReader;
#[doc = "Field `ceata_int_en` writer - MMC Control CEATA Interrupt Enable"]
pub type CeataIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `use_idmac` reader - MMC Control Use IDMAC"]
pub type UseIdmacR = crate::BitReader;
#[doc = "Field `use_idmac` writer - MMC Control Use IDMAC"]
pub type UseIdmacW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "MMC Control Reset"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `reset_mmc` field"]
    #[inline(always)]
    pub fn reset(&self, n: u8) -> ResetR {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        ResetR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC Control Reset"]
    #[inline(always)]
    pub fn reset_iter(&self) -> impl Iterator<Item = ResetR> + '_ {
        (0..3).map(move |n| ResetR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MMC Control Reset"]
    #[inline(always)]
    pub fn reset_mmc(&self) -> ResetR {
        ResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Control Reset"]
    #[inline(always)]
    pub fn reset_fifo(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Control Reset"]
    #[inline(always)]
    pub fn reset_dma(&self) -> ResetR {
        ResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "MMC Control Enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `enable_int` field"]
    #[inline(always)]
    pub fn enable(&self, n: u8) -> EnableR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        EnableR::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC Control Enable"]
    #[inline(always)]
    pub fn enable_iter(&self) -> impl Iterator<Item = EnableR> + '_ {
        (0..2).map(move |n| EnableR::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - MMC Control Enable"]
    #[inline(always)]
    pub fn enable_int(&self) -> EnableR {
        EnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Control Enable"]
    #[inline(always)]
    pub fn enable_dma(&self) -> EnableR {
        EnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Control Read Wait"]
    #[inline(always)]
    pub fn read_wait(&self) -> ReadWaitR {
        ReadWaitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Control Send IRQ Response"]
    #[inline(always)]
    pub fn send_irq_resp(&self) -> SendIrqRespR {
        SendIrqRespR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Control Abort Read Data"]
    #[inline(always)]
    pub fn abrt_read_data(&self) -> AbrtReadDataR {
        AbrtReadDataR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "MMC Control Send"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `send_ccsd` field"]
    #[inline(always)]
    pub fn send(&self, n: u8) -> SendR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SendR::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC Control Send"]
    #[inline(always)]
    pub fn send_iter(&self) -> impl Iterator<Item = SendR> + '_ {
        (0..2).map(move |n| SendR::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - MMC Control Send"]
    #[inline(always)]
    pub fn send_ccsd(&self) -> SendR {
        SendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Control Send"]
    #[inline(always)]
    pub fn send_as_ccsd(&self) -> SendR {
        SendR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Control CEATA Interrupt Enable"]
    #[inline(always)]
    pub fn ceata_int_en(&self) -> CeataIntEnR {
        CeataIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Control Use IDMAC"]
    #[inline(always)]
    pub fn use_idmac(&self) -> UseIdmacR {
        UseIdmacR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "MMC Control Reset"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `reset_mmc` field"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self, n: u8) -> ResetW<CtrlSpec> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        ResetW::new(self, n)
    }
    #[doc = "Bit 0 - MMC Control Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset_mmc(&mut self) -> ResetW<CtrlSpec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Control Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset_fifo(&mut self) -> ResetW<CtrlSpec> {
        ResetW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Control Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset_dma(&mut self) -> ResetW<CtrlSpec> {
        ResetW::new(self, 2)
    }
    #[doc = "MMC Control Enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `enable_int` field"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self, n: u8) -> EnableW<CtrlSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        EnableW::new(self, n + 4)
    }
    #[doc = "Bit 4 - MMC Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable_int(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable_dma(&mut self) -> EnableW<CtrlSpec> {
        EnableW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Control Read Wait"]
    #[inline(always)]
    #[must_use]
    pub fn read_wait(&mut self) -> ReadWaitW<CtrlSpec> {
        ReadWaitW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC Control Send IRQ Response"]
    #[inline(always)]
    #[must_use]
    pub fn send_irq_resp(&mut self) -> SendIrqRespW<CtrlSpec> {
        SendIrqRespW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Control Abort Read Data"]
    #[inline(always)]
    #[must_use]
    pub fn abrt_read_data(&mut self) -> AbrtReadDataW<CtrlSpec> {
        AbrtReadDataW::new(self, 8)
    }
    #[doc = "MMC Control Send"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `send_ccsd` field"]
    #[inline(always)]
    #[must_use]
    pub fn send(&mut self, n: u8) -> SendW<CtrlSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SendW::new(self, n + 9)
    }
    #[doc = "Bit 9 - MMC Control Send"]
    #[inline(always)]
    #[must_use]
    pub fn send_ccsd(&mut self) -> SendW<CtrlSpec> {
        SendW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Control Send"]
    #[inline(always)]
    #[must_use]
    pub fn send_as_ccsd(&mut self) -> SendW<CtrlSpec> {
        SendW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Control CEATA Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ceata_int_en(&mut self) -> CeataIntEnW<CtrlSpec> {
        CeataIntEnW::new(self, 11)
    }
    #[doc = "Bit 25 - MMC Control Use IDMAC"]
    #[inline(always)]
    #[must_use]
    pub fn use_idmac(&mut self) -> UseIdmacW<CtrlSpec> {
        UseIdmacW::new(self, 25)
    }
}
#[doc = "MMC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
