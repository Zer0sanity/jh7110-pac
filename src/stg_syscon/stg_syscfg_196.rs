#[doc = "Register `stg_syscfg_196` reader"]
pub type R = crate::R<StgSyscfg196Spec>;
#[doc = "Register `stg_syscfg_196` writer"]
pub type W = crate::W<StgSyscfg196Spec>;
#[doc = "Field `u1_pcie_pl_wake_in` reader - PCIE PL Wake IN"]
pub type U1PciePlWakeInR = crate::BitReader;
#[doc = "Field `u1_pcie_pl_wake_in` writer - PCIE PL Wake IN"]
pub type U1PciePlWakeInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_pl_wake_oen` reader - PCIE PL Wake OEN"]
pub type U1PciePlWakeOenR = crate::BitReader;
#[doc = "Field `u1_pcie_rx_standby_0` reader - PCIE RX Standby"]
pub type U1PcieRxStandby0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PCIE PL Wake IN"]
    #[inline(always)]
    pub fn u1_pcie_pl_wake_in(&self) -> U1PciePlWakeInR {
        U1PciePlWakeInR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCIE PL Wake OEN"]
    #[inline(always)]
    pub fn u1_pcie_pl_wake_oen(&self) -> U1PciePlWakeOenR {
        U1PciePlWakeOenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCIE RX Standby"]
    #[inline(always)]
    pub fn u1_pcie_rx_standby_0(&self) -> U1PcieRxStandby0R {
        U1PcieRxStandby0R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCIE PL Wake IN"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_pl_wake_in(&mut self) -> U1PciePlWakeInW<StgSyscfg196Spec> {
        U1PciePlWakeInW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 784\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_196::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_196::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg196Spec;
impl crate::RegisterSpec for StgSyscfg196Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_196::R`](R) reader structure"]
impl crate::Readable for StgSyscfg196Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_196::W`](W) writer structure"]
impl crate::Writable for StgSyscfg196Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_196 to value 0x01"]
impl crate::Resettable for StgSyscfg196Spec {
    const RESET_VALUE: u32 = 0x01;
}
