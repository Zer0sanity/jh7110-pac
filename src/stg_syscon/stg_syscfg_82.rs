#[doc = "Register `stg_syscfg_82` reader"]
pub type R = crate::R<StgSyscfg82Spec>;
#[doc = "Register `stg_syscfg_82` writer"]
pub type W = crate::W<StgSyscfg82Spec>;
#[doc = "Field `u0_pcie_pf3_offset` reader - PCIE PF3 Offset"]
pub type U0PciePf3OffsetR = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_pf3_offset` writer - PCIE PF3 Offset"]
pub type U0PciePf3OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `u0_pcie_phy_mode` reader - PCIE PHY Mode"]
pub type U0PciePhyModeR = crate::FieldReader;
#[doc = "Field `u0_pcie_phy_mode` writer - PCIE PHY Mode"]
pub type U0PciePhyModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_pl_clkrem_allow` reader - PCIE PL Clock REM Allow"]
pub type U0PciePlClkremAllowR = crate::BitReader;
#[doc = "Field `u0_pcie_pl_clkrem_allow` writer - PCIE PL Clock REM Allow"]
pub type U0PciePlClkremAllowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_pl_clkreq_oen` reader - PCIE PL Clock Request OEN"]
pub type U0PciePlClkreqOenR = crate::BitReader;
#[doc = "Field `u0_pcie_pl_equ_phase` reader - PCIE PL EQU Phase"]
pub type U0PciePlEquPhaseR = crate::FieldReader;
#[doc = "Field `u0_pcie_pl_ltssm` reader - PCIE PL LTSSM"]
pub type U0PciePlLtssmR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - PCIE PF3 Offset"]
    #[inline(always)]
    pub fn u0_pcie_pf3_offset(&self) -> U0PciePf3OffsetR {
        U0PciePf3OffsetR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:21 - PCIE PHY Mode"]
    #[inline(always)]
    pub fn u0_pcie_phy_mode(&self) -> U0PciePhyModeR {
        U0PciePhyModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - PCIE PL Clock REM Allow"]
    #[inline(always)]
    pub fn u0_pcie_pl_clkrem_allow(&self) -> U0PciePlClkremAllowR {
        U0PciePlClkremAllowR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PCIE PL Clock Request OEN"]
    #[inline(always)]
    pub fn u0_pcie_pl_clkreq_oen(&self) -> U0PciePlClkreqOenR {
        U0PciePlClkreqOenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - PCIE PL EQU Phase"]
    #[inline(always)]
    pub fn u0_pcie_pl_equ_phase(&self) -> U0PciePlEquPhaseR {
        U0PciePlEquPhaseR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:30 - PCIE PL LTSSM"]
    #[inline(always)]
    pub fn u0_pcie_pl_ltssm(&self) -> U0PciePlLtssmR {
        U0PciePlLtssmR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - PCIE PF3 Offset"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pf3_offset(&mut self) -> U0PciePf3OffsetW<StgSyscfg82Spec> {
        U0PciePf3OffsetW::new(self, 0)
    }
    #[doc = "Bits 20:21 - PCIE PHY Mode"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_phy_mode(&mut self) -> U0PciePhyModeW<StgSyscfg82Spec> {
        U0PciePhyModeW::new(self, 20)
    }
    #[doc = "Bit 22 - PCIE PL Clock REM Allow"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pl_clkrem_allow(&mut self) -> U0PciePlClkremAllowW<StgSyscfg82Spec> {
        U0PciePlClkremAllowW::new(self, 22)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 328\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_82::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_82::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg82Spec;
impl crate::RegisterSpec for StgSyscfg82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_82::R`](R) reader structure"]
impl crate::Readable for StgSyscfg82Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_82::W`](W) writer structure"]
impl crate::Writable for StgSyscfg82Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_82 to value 0"]
impl crate::Resettable for StgSyscfg82Spec {
    const RESET_VALUE: u32 = 0;
}
