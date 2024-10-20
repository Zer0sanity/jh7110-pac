#[doc = "Register `ctrl_status` reader"]
pub type R = crate::R<CtrlStatusSpec>;
#[doc = "Register `ctrl_status` writer"]
pub type W = crate::W<CtrlStatusSpec>;
#[doc = "Field `tlpi(en,ex)` reader - Transmit LPI - 0: Entry, 1: Exit"]
pub type TlpiR = crate::BitReader;
#[doc = "Field `tlpi(en,ex)` writer - Transmit LPI - 0: Entry, 1: Exit"]
pub type TlpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rlpi(en,ex)` reader - Receive LPI - 0: Entry, 1: Exit"]
pub type RlpiR = crate::BitReader;
#[doc = "Field `rlpi(en,ex)` writer - Receive LPI - 0: Entry, 1: Exit"]
pub type RlpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lpien` reader - LPI Enable"]
pub type LpienR = crate::BitReader;
#[doc = "Field `lpien` writer - LPI Enable"]
pub type LpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pls` reader - PHY Link Status"]
pub type PlsR = crate::BitReader;
#[doc = "Field `pls` writer - PHY Link Status"]
pub type PlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lpitxa` reader - Enable LPI TX Automate"]
pub type LpitxaR = crate::BitReader;
#[doc = "Field `lpitxa` writer - Enable LPI TX Automate"]
pub type LpitxaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lpiate` reader - LPI Timer Enable"]
pub type LpiateR = crate::BitReader;
#[doc = "Field `lpiate` writer - LPI Timer Enable"]
pub type LpiateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lpitcse` reader - LPI TX Clock Stop Enable"]
pub type LpitcseR = crate::BitReader;
#[doc = "Field `lpitcse` writer - LPI TX Clock Stop Enable"]
pub type LpitcseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Transmit LPI - 0: Entry, 1: Exit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `tlpien` field"]
    #[inline(always)]
    pub fn tlpi(&self, n: u8) -> TlpiR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TlpiR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transmit LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    pub fn tlpi_iter(&self) -> impl Iterator<Item = TlpiR> + '_ {
        (0..2).map(move |n| TlpiR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Transmit LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    pub fn tlpien(&self) -> TlpiR {
        TlpiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    pub fn tlpiex(&self) -> TlpiR {
        TlpiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Receive LPI - 0: Entry, 1: Exit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rlpien` field"]
    #[inline(always)]
    pub fn rlpi(&self, n: u8) -> RlpiR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RlpiR::new(((self.bits >> (n + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Receive LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    pub fn rlpi_iter(&self) -> impl Iterator<Item = RlpiR> + '_ {
        (0..2).map(move |n| RlpiR::new(((self.bits >> (n + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Receive LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    pub fn rlpien(&self) -> RlpiR {
        RlpiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    pub fn rlpiex(&self) -> RlpiR {
        RlpiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    pub fn lpien(&self) -> LpienR {
        LpienR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable LPI TX Automate"]
    #[inline(always)]
    pub fn lpitxa(&self) -> LpitxaR {
        LpitxaR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPI Timer Enable"]
    #[inline(always)]
    pub fn lpiate(&self) -> LpiateR {
        LpiateR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPI TX Clock Stop Enable"]
    #[inline(always)]
    pub fn lpitcse(&self) -> LpitcseR {
        LpitcseR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Transmit LPI - 0: Entry, 1: Exit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `tlpien` field"]
    #[inline(always)]
    #[must_use]
    pub fn tlpi(&mut self, n: u8) -> TlpiW<CtrlStatusSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TlpiW::new(self, n)
    }
    #[doc = "Bit 0 - Transmit LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    #[must_use]
    pub fn tlpien(&mut self) -> TlpiW<CtrlStatusSpec> {
        TlpiW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    #[must_use]
    pub fn tlpiex(&mut self) -> TlpiW<CtrlStatusSpec> {
        TlpiW::new(self, 1)
    }
    #[doc = "Receive LPI - 0: Entry, 1: Exit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rlpien` field"]
    #[inline(always)]
    #[must_use]
    pub fn rlpi(&mut self, n: u8) -> RlpiW<CtrlStatusSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        RlpiW::new(self, n + 2)
    }
    #[doc = "Bit 2 - Receive LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    #[must_use]
    pub fn rlpien(&mut self) -> RlpiW<CtrlStatusSpec> {
        RlpiW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive LPI - 0: Entry, 1: Exit"]
    #[inline(always)]
    #[must_use]
    pub fn rlpiex(&mut self) -> RlpiW<CtrlStatusSpec> {
        RlpiW::new(self, 3)
    }
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpien(&mut self) -> LpienW<CtrlStatusSpec> {
        LpienW::new(self, 16)
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PlsW<CtrlStatusSpec> {
        PlsW::new(self, 17)
    }
    #[doc = "Bit 19 - Enable LPI TX Automate"]
    #[inline(always)]
    #[must_use]
    pub fn lpitxa(&mut self) -> LpitxaW<CtrlStatusSpec> {
        LpitxaW::new(self, 19)
    }
    #[doc = "Bit 20 - LPI Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpiate(&mut self) -> LpiateW<CtrlStatusSpec> {
        LpiateW::new(self, 20)
    }
    #[doc = "Bit 21 - LPI TX Clock Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpitcse(&mut self) -> LpitcseW<CtrlStatusSpec> {
        LpitcseW::new(self, 21)
    }
}
#[doc = "MAC LPI Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlStatusSpec;
impl crate::RegisterSpec for CtrlStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_status::R`](R) reader structure"]
impl crate::Readable for CtrlStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_status::W`](W) writer structure"]
impl crate::Writable for CtrlStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl_status to value 0"]
impl crate::Resettable for CtrlStatusSpec {
    const RESET_VALUE: u32 = 0;
}
