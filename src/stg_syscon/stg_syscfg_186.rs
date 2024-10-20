#[doc = "Register `stg_syscfg_186` reader"]
pub type R = crate::R<StgSyscfg186Spec>;
#[doc = "Register `stg_syscfg_186` writer"]
pub type W = crate::W<StgSyscfg186Spec>;
#[doc = "Field `u1_pcie_mperstn` reader - PCIE MPERSTN"]
pub type U1PcieMperstnR = crate::BitReader;
#[doc = "Field `u1_pcie_mperstn` writer - PCIE MPERSTN"]
pub type U1PcieMperstnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_ebuf_mode` reader - PCIE EBUF Mode"]
pub type U1PcieEbufModeR = crate::BitReader;
#[doc = "Field `u1_pcie_ebuf_mode` writer - PCIE EBUF Mode"]
pub type U1PcieEbufModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_phy_test_cfg` reader - PCIE PHY Test Config"]
pub type U1PciePhyTestCfgR = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_phy_test_cfg` writer - PCIE PHY Test Config"]
pub type U1PciePhyTestCfgW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `u1_pcie_rx_eq_training` reader - PCIE RX EQ Training"]
pub type U1PcieRxEqTrainingR = crate::BitReader;
#[doc = "Field `u1_pcie_rx_eq_training` writer - PCIE RX EQ Training"]
pub type U1PcieRxEqTrainingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_rxterm_en` reader - PCIE RXTERM Enable"]
pub type U1PcieRxtermEnR = crate::BitReader;
#[doc = "Field `u1_pcie_rxterm_en` writer - PCIE RXTERM Enable"]
pub type U1PcieRxtermEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_tx_onezeros` reader - PCIE TX One Zeros"]
pub type U1PcieTxOnezerosR = crate::BitReader;
#[doc = "Field `u1_pcie_tx_onezeros` writer - PCIE TX One Zeros"]
pub type U1PcieTxOnezerosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PCIE MPERSTN"]
    #[inline(always)]
    pub fn u1_pcie_mperstn(&self) -> U1PcieMperstnR {
        U1PcieMperstnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCIE EBUF Mode"]
    #[inline(always)]
    pub fn u1_pcie_ebuf_mode(&self) -> U1PcieEbufModeR {
        U1PcieEbufModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:24 - PCIE PHY Test Config"]
    #[inline(always)]
    pub fn u1_pcie_phy_test_cfg(&self) -> U1PciePhyTestCfgR {
        U1PciePhyTestCfgR::new((self.bits >> 2) & 0x007f_ffff)
    }
    #[doc = "Bit 25 - PCIE RX EQ Training"]
    #[inline(always)]
    pub fn u1_pcie_rx_eq_training(&self) -> U1PcieRxEqTrainingR {
        U1PcieRxEqTrainingR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCIE RXTERM Enable"]
    #[inline(always)]
    pub fn u1_pcie_rxterm_en(&self) -> U1PcieRxtermEnR {
        U1PcieRxtermEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PCIE TX One Zeros"]
    #[inline(always)]
    pub fn u1_pcie_tx_onezeros(&self) -> U1PcieTxOnezerosR {
        U1PcieTxOnezerosR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCIE MPERSTN"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_mperstn(&mut self) -> U1PcieMperstnW<StgSyscfg186Spec> {
        U1PcieMperstnW::new(self, 0)
    }
    #[doc = "Bit 1 - PCIE EBUF Mode"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_ebuf_mode(&mut self) -> U1PcieEbufModeW<StgSyscfg186Spec> {
        U1PcieEbufModeW::new(self, 1)
    }
    #[doc = "Bits 2:24 - PCIE PHY Test Config"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_phy_test_cfg(&mut self) -> U1PciePhyTestCfgW<StgSyscfg186Spec> {
        U1PciePhyTestCfgW::new(self, 2)
    }
    #[doc = "Bit 25 - PCIE RX EQ Training"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_rx_eq_training(&mut self) -> U1PcieRxEqTrainingW<StgSyscfg186Spec> {
        U1PcieRxEqTrainingW::new(self, 25)
    }
    #[doc = "Bit 26 - PCIE RXTERM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_rxterm_en(&mut self) -> U1PcieRxtermEnW<StgSyscfg186Spec> {
        U1PcieRxtermEnW::new(self, 26)
    }
    #[doc = "Bit 27 - PCIE TX One Zeros"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_tx_onezeros(&mut self) -> U1PcieTxOnezerosW<StgSyscfg186Spec> {
        U1PcieTxOnezerosW::new(self, 27)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 744\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_186::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_186::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg186Spec;
impl crate::RegisterSpec for StgSyscfg186Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_186::R`](R) reader structure"]
impl crate::Readable for StgSyscfg186Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_186::W`](W) writer structure"]
impl crate::Writable for StgSyscfg186Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_186 to value 0x0480_0001"]
impl crate::Resettable for StgSyscfg186Spec {
    const RESET_VALUE: u32 = 0x0480_0001;
}
