#[doc = "Register `stg_syscfg_123` reader"]
pub type R = crate::R<StgSyscfg123Spec>;
#[doc = "Register `stg_syscfg_123` writer"]
pub type W = crate::W<StgSyscfg123Spec>;
#[doc = "Field `u0_pcie_test_sel` reader - PCIE Test Selector"]
pub type U0PcieTestSelR = crate::FieldReader;
#[doc = "Field `u0_pcie_test_sel` writer - PCIE Test Selector"]
pub type U0PcieTestSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u0_pcie_tl_clock_freq` reader - PCIE TL Clock Frequency"]
pub type U0PcieTlClockFreqR = crate::FieldReader<u32>;
#[doc = "Field `u0_pcie_tl_clock_freq` writer - PCIE TL Clock Frequency"]
pub type U0PcieTlClockFreqW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:3 - PCIE Test Selector"]
    #[inline(always)]
    pub fn u0_pcie_test_sel(&self) -> U0PcieTestSelR {
        U0PcieTestSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:25 - PCIE TL Clock Frequency"]
    #[inline(always)]
    pub fn u0_pcie_tl_clock_freq(&self) -> U0PcieTlClockFreqR {
        U0PcieTlClockFreqR::new((self.bits >> 4) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - PCIE Test Selector"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_test_sel(&mut self) -> U0PcieTestSelW<StgSyscfg123Spec> {
        U0PcieTestSelW::new(self, 0)
    }
    #[doc = "Bits 4:25 - PCIE TL Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_tl_clock_freq(&mut self) -> U0PcieTlClockFreqW<StgSyscfg123Spec> {
        U0PcieTlClockFreqW::new(self, 4)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 492\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_123::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_123::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg123Spec;
impl crate::RegisterSpec for StgSyscfg123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_123::R`](R) reader structure"]
impl crate::Readable for StgSyscfg123Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_123::W`](W) writer structure"]
impl crate::Writable for StgSyscfg123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_123 to value 0x0c80"]
impl crate::Resettable for StgSyscfg123Spec {
    const RESET_VALUE: u32 = 0x0c80;
}
