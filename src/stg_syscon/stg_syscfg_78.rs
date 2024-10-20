#[doc = "Register `stg_syscfg_78` reader"]
pub type R = crate::R<StgSyscfg78Spec>;
#[doc = "Register `stg_syscfg_78` writer"]
pub type W = crate::W<StgSyscfg78Spec>;
#[doc = "Field `u0_pcie_mperstn` reader - PCIE MPERSTN"]
pub type U0PcieMperstnR = crate::BitReader;
#[doc = "Field `u0_pcie_mperstn` writer - PCIE MPERSTN"]
pub type U0PcieMperstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_ebuf_mode` reader - PCIE EBUF Mode"]
pub type U0PcieEbufModeR = crate::BitReader;
#[doc = "Field `u0_pcie_ebuf_mode` writer - PCIE EBUF Mode"]
pub type U0PcieEbufModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_phy_test_cfg` reader - PCIE PHY Test Config"]
pub type U0PciePhyTestCfgR = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_phy_test_cfg` writer - PCIE PHY Test Config"]
pub type U0PciePhyTestCfgW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `u0_pcie_rx_eq_training` reader - PCIE RX EQ Training"]
pub type U0PcieRxEqTrainingR = crate::BitReader;
#[doc = "Field `u0_pcie_rx_eq_training` writer - PCIE RX EQ Training"]
pub type U0PcieRxEqTrainingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_rxterm_en` reader - PCIE RXTERM Enable"]
pub type U0PcieRxtermEnR = crate::BitReader;
#[doc = "Field `u0_pcie_rxterm_en` writer - PCIE RXTERM Enable"]
pub type U0PcieRxtermEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_tx_onezeros` reader - PCIE TX One Zeros"]
pub type U0PcieTxOnezerosR = crate::BitReader;
#[doc = "Field `u0_pcie_tx_onezeros` writer - PCIE TX One Zeros"]
pub type U0PcieTxOnezerosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PCIE MPERSTN"]
    #[inline(always)]
    pub fn u0_pcie_mperstn(&self) -> U0PcieMperstnR {
        U0PcieMperstnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCIE EBUF Mode"]
    #[inline(always)]
    pub fn u0_pcie_ebuf_mode(&self) -> U0PcieEbufModeR {
        U0PcieEbufModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:24 - PCIE PHY Test Config"]
    #[inline(always)]
    pub fn u0_pcie_phy_test_cfg(&self) -> U0PciePhyTestCfgR {
        U0PciePhyTestCfgR::new((self.bits >> 2) & 0x007f_ffff)
    }
    #[doc = "Bit 25 - PCIE RX EQ Training"]
    #[inline(always)]
    pub fn u0_pcie_rx_eq_training(&self) -> U0PcieRxEqTrainingR {
        U0PcieRxEqTrainingR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCIE RXTERM Enable"]
    #[inline(always)]
    pub fn u0_pcie_rxterm_en(&self) -> U0PcieRxtermEnR {
        U0PcieRxtermEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PCIE TX One Zeros"]
    #[inline(always)]
    pub fn u0_pcie_tx_onezeros(&self) -> U0PcieTxOnezerosR {
        U0PcieTxOnezerosR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCIE MPERSTN"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_mperstn(&mut self) -> U0PcieMperstnW<StgSyscfg78Spec> {
        U0PcieMperstnW::new(self, 0)
    }
    #[doc = "Bit 1 - PCIE EBUF Mode"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_ebuf_mode(&mut self) -> U0PcieEbufModeW<StgSyscfg78Spec> {
        U0PcieEbufModeW::new(self, 1)
    }
    #[doc = "Bits 2:24 - PCIE PHY Test Config"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_phy_test_cfg(&mut self) -> U0PciePhyTestCfgW<StgSyscfg78Spec> {
        U0PciePhyTestCfgW::new(self, 2)
    }
    #[doc = "Bit 25 - PCIE RX EQ Training"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_rx_eq_training(&mut self) -> U0PcieRxEqTrainingW<StgSyscfg78Spec> {
        U0PcieRxEqTrainingW::new(self, 25)
    }
    #[doc = "Bit 26 - PCIE RXTERM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_rxterm_en(&mut self) -> U0PcieRxtermEnW<StgSyscfg78Spec> {
        U0PcieRxtermEnW::new(self, 26)
    }
    #[doc = "Bit 27 - PCIE TX One Zeros"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_tx_onezeros(&mut self) -> U0PcieTxOnezerosW<StgSyscfg78Spec> {
        U0PcieTxOnezerosW::new(self, 27)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 312\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_78::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_78::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg78Spec;
impl crate::RegisterSpec for StgSyscfg78Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_78::R`](R) reader structure"]
impl crate::Readable for StgSyscfg78Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_78::W`](W) writer structure"]
impl crate::Writable for StgSyscfg78Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_78 to value 0x0480_0001"]
impl crate::Resettable for StgSyscfg78Spec {
    const RESET_VALUE: u32 = 0x0480_0001;
}
