#[doc = "Register `stg_syscfg_233` reader"]
pub type R = crate::R<StgSyscfg233Spec>;
#[doc = "Register `stg_syscfg_233` writer"]
pub type W = crate::W<StgSyscfg233Spec>;
#[doc = "Field `u1_pcie_tx_pattern` reader - PCIE TX Pattern"]
pub type U1PcieTxPatternR = crate::FieldReader;
#[doc = "Field `u1_pcie_tx_pattern` writer - PCIE TX Pattern"]
pub type U1PcieTxPatternW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_usb3_bus_width` reader - PCIE USB3 Bus Width"]
pub type U1PcieUsb3BusWidthR = crate::FieldReader;
#[doc = "Field `u1_pcie_usb3_bus_width` writer - PCIE USB3 Bus Width"]
pub type U1PcieUsb3BusWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_usb3_phy_enable` reader - PCIE USB3 PHY Enable"]
pub type U1PcieUsb3PhyEnableR = crate::BitReader;
#[doc = "Field `u1_pcie_usb3_phy_enable` writer - PCIE USB3 PHY Enable"]
pub type U1PcieUsb3PhyEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_usb3_rate` reader - PCIE USB3 Rate"]
pub type U1PcieUsb3RateR = crate::FieldReader;
#[doc = "Field `u1_pcie_usb3_rate` writer - PCIE USB3 Rate"]
pub type U1PcieUsb3RateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_usb3_rx_standby` reader - PCIE USB3 RX Standby"]
pub type U1PcieUsb3RxStandbyR = crate::BitReader;
#[doc = "Field `u1_pcie_usb3_rx_standby` writer - PCIE USB3 RX Standby"]
pub type U1PcieUsb3RxStandbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_xwdecerr` reader - PCIE XWDECERR"]
pub type U1PcieXwdecerrR = crate::BitReader;
#[doc = "Field `u1_pcie_xwerrclr` reader - PCIE XWERRCLR"]
pub type U1PcieXwerrclrR = crate::BitReader;
#[doc = "Field `u1_pcie_xwerrclr` writer - PCIE XWERRCLR"]
pub type U1PcieXwerrclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_xwslverr` reader - PCIE XWSLVERR"]
pub type U1PcieXwslverrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - PCIE TX Pattern"]
    #[inline(always)]
    pub fn u1_pcie_tx_pattern(&self) -> U1PcieTxPatternR {
        U1PcieTxPatternR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PCIE USB3 Bus Width"]
    #[inline(always)]
    pub fn u1_pcie_usb3_bus_width(&self) -> U1PcieUsb3BusWidthR {
        U1PcieUsb3BusWidthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PCIE USB3 PHY Enable"]
    #[inline(always)]
    pub fn u1_pcie_usb3_phy_enable(&self) -> U1PcieUsb3PhyEnableR {
        U1PcieUsb3PhyEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - PCIE USB3 Rate"]
    #[inline(always)]
    pub fn u1_pcie_usb3_rate(&self) -> U1PcieUsb3RateR {
        U1PcieUsb3RateR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - PCIE USB3 RX Standby"]
    #[inline(always)]
    pub fn u1_pcie_usb3_rx_standby(&self) -> U1PcieUsb3RxStandbyR {
        U1PcieUsb3RxStandbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCIE XWDECERR"]
    #[inline(always)]
    pub fn u1_pcie_xwdecerr(&self) -> U1PcieXwdecerrR {
        U1PcieXwdecerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCIE XWERRCLR"]
    #[inline(always)]
    pub fn u1_pcie_xwerrclr(&self) -> U1PcieXwerrclrR {
        U1PcieXwerrclrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCIE XWSLVERR"]
    #[inline(always)]
    pub fn u1_pcie_xwslverr(&self) -> U1PcieXwslverrR {
        U1PcieXwslverrR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCIE TX Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_tx_pattern(&mut self) -> U1PcieTxPatternW<StgSyscfg233Spec> {
        U1PcieTxPatternW::new(self, 0)
    }
    #[doc = "Bits 2:3 - PCIE USB3 Bus Width"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_usb3_bus_width(&mut self) -> U1PcieUsb3BusWidthW<StgSyscfg233Spec> {
        U1PcieUsb3BusWidthW::new(self, 2)
    }
    #[doc = "Bit 4 - PCIE USB3 PHY Enable"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_usb3_phy_enable(&mut self) -> U1PcieUsb3PhyEnableW<StgSyscfg233Spec> {
        U1PcieUsb3PhyEnableW::new(self, 4)
    }
    #[doc = "Bits 5:6 - PCIE USB3 Rate"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_usb3_rate(&mut self) -> U1PcieUsb3RateW<StgSyscfg233Spec> {
        U1PcieUsb3RateW::new(self, 5)
    }
    #[doc = "Bit 7 - PCIE USB3 RX Standby"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_usb3_rx_standby(&mut self) -> U1PcieUsb3RxStandbyW<StgSyscfg233Spec> {
        U1PcieUsb3RxStandbyW::new(self, 7)
    }
    #[doc = "Bit 9 - PCIE XWERRCLR"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_xwerrclr(&mut self) -> U1PcieXwerrclrW<StgSyscfg233Spec> {
        U1PcieXwerrclrW::new(self, 9)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 932\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_233::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_233::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg233Spec;
impl crate::RegisterSpec for StgSyscfg233Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_233::R`](R) reader structure"]
impl crate::Readable for StgSyscfg233Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_233::W`](W) writer structure"]
impl crate::Writable for StgSyscfg233Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_233 to value 0x08"]
impl crate::Resettable for StgSyscfg233Spec {
    const RESET_VALUE: u32 = 0x08;
}
