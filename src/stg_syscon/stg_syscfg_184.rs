#[doc = "Register `stg_syscfg_184` reader"]
pub type R = crate::R<StgSyscfg184Spec>;
#[doc = "Register `stg_syscfg_184` writer"]
pub type W = crate::W<StgSyscfg184Spec>;
#[doc = "Field `u1_pcie_k_phyparam_839_832` reader - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam839_832R = crate::FieldReader;
#[doc = "Field `u1_pcie_k_phyparam_839_832` writer - PCIE PHY Parameter (little-endian)"]
pub type U1PcieKPhyparam839_832W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `u1_pcie_k_rp_nep` reader - PCIE RP NEP"]
pub type U1PcieKRpNepR = crate::BitReader;
#[doc = "Field `u1_pcie_k_rp_nep` writer - PCIE RP NEP"]
pub type U1PcieKRpNepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_l1sub_entack` reader - PCIE L1SUB ENTACK"]
pub type U1PcieL1subEntackR = crate::BitReader;
#[doc = "Field `u1_pcie_l1sub_entreq` reader - PCIE L1SUB ENREQ"]
pub type U1PcieL1subEntreqR = crate::BitReader;
#[doc = "Field `u1_pcie_l1sub_entreq` writer - PCIE L1SUB ENREQ"]
pub type U1PcieL1subEntreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    pub fn u1_pcie_k_phyparam_839_832(&self) -> U1PcieKPhyparam839_832R {
        U1PcieKPhyparam839_832R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PCIE RP NEP"]
    #[inline(always)]
    pub fn u1_pcie_k_rp_nep(&self) -> U1PcieKRpNepR {
        U1PcieKRpNepR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCIE L1SUB ENTACK"]
    #[inline(always)]
    pub fn u1_pcie_l1sub_entack(&self) -> U1PcieL1subEntackR {
        U1PcieL1subEntackR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCIE L1SUB ENREQ"]
    #[inline(always)]
    pub fn u1_pcie_l1sub_entreq(&self) -> U1PcieL1subEntreqR {
        U1PcieL1subEntreqR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCIE PHY Parameter (little-endian)"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_k_phyparam_839_832(&mut self) -> U1PcieKPhyparam839_832W<StgSyscfg184Spec> {
        U1PcieKPhyparam839_832W::new(self, 0)
    }
    #[doc = "Bit 8 - PCIE RP NEP"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_k_rp_nep(&mut self) -> U1PcieKRpNepW<StgSyscfg184Spec> {
        U1PcieKRpNepW::new(self, 8)
    }
    #[doc = "Bit 10 - PCIE L1SUB ENREQ"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_l1sub_entreq(&mut self) -> U1PcieL1subEntreqW<StgSyscfg184Spec> {
        U1PcieL1subEntreqW::new(self, 10)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 736\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_184::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_184::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg184Spec;
impl crate::RegisterSpec for StgSyscfg184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_184::R`](R) reader structure"]
impl crate::Readable for StgSyscfg184Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_184::W`](W) writer structure"]
impl crate::Writable for StgSyscfg184Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_184 to value 0"]
impl crate::Resettable for StgSyscfg184Spec {
    const RESET_VALUE: u32 = 0;
}
