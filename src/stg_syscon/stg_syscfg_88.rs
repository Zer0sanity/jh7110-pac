#[doc = "Register `stg_syscfg_88` reader"]
pub type R = crate::R<StgSyscfg88Spec>;
#[doc = "Register `stg_syscfg_88` writer"]
pub type W = crate::W<StgSyscfg88Spec>;
#[doc = "Field `u0_pcie_pl_wake_in` reader - PCIE PL Wake IN"]
pub type U0PciePlWakeInR = crate::BitReader;
#[doc = "Field `u0_pcie_pl_wake_in` writer - PCIE PL Wake IN"]
pub type U0PciePlWakeInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_pl_wake_oen` reader - PCIE PL Wake OEN"]
pub type U0PciePlWakeOenR = crate::BitReader;
#[doc = "Field `u0_pcie_rx_standby_0` reader - PCIE RX Standby"]
pub type U0PcieRxStandby0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PCIE PL Wake IN"]
    #[inline(always)]
    pub fn u0_pcie_pl_wake_in(&self) -> U0PciePlWakeInR {
        U0PciePlWakeInR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCIE PL Wake OEN"]
    #[inline(always)]
    pub fn u0_pcie_pl_wake_oen(&self) -> U0PciePlWakeOenR {
        U0PciePlWakeOenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCIE RX Standby"]
    #[inline(always)]
    pub fn u0_pcie_rx_standby_0(&self) -> U0PcieRxStandby0R {
        U0PcieRxStandby0R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCIE PL Wake IN"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pl_wake_in(&mut self) -> U0PciePlWakeInW<StgSyscfg88Spec> {
        U0PciePlWakeInW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 352\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_88::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_88::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg88Spec;
impl crate::RegisterSpec for StgSyscfg88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_88::R`](R) reader structure"]
impl crate::Readable for StgSyscfg88Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_88::W`](W) writer structure"]
impl crate::Writable for StgSyscfg88Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_88 to value 0x01"]
impl crate::Resettable for StgSyscfg88Spec {
    const RESET_VALUE: u32 = 0x01;
}
