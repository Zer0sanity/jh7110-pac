#[doc = "Register `stg_syscfg_231` reader"]
pub type R = crate::R<StgSyscfg231Spec>;
#[doc = "Register `stg_syscfg_231` writer"]
pub type W = crate::W<StgSyscfg231Spec>;
#[doc = "Field `u1_pcie_test_sel` reader - PCIE Test Selector"]
pub type U1PcieTestSelR = crate::FieldReader;
#[doc = "Field `u1_pcie_test_sel` writer - PCIE Test Selector"]
pub type U1PcieTestSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u1_pcie_tl_clock_freq` reader - PCIE TL Clock Frequency"]
pub type U1PcieTlClockFreqR = crate::FieldReader<u32>;
#[doc = "Field `u1_pcie_tl_clock_freq` writer - PCIE TL Clock Frequency"]
pub type U1PcieTlClockFreqW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:3 - PCIE Test Selector"]
    #[inline(always)]
    pub fn u1_pcie_test_sel(&self) -> U1PcieTestSelR {
        U1PcieTestSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:25 - PCIE TL Clock Frequency"]
    #[inline(always)]
    pub fn u1_pcie_tl_clock_freq(&self) -> U1PcieTlClockFreqR {
        U1PcieTlClockFreqR::new((self.bits >> 4) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - PCIE Test Selector"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_test_sel(&mut self) -> U1PcieTestSelW<StgSyscfg231Spec> {
        U1PcieTestSelW::new(self, 0)
    }
    #[doc = "Bits 4:25 - PCIE TL Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_tl_clock_freq(&mut self) -> U1PcieTlClockFreqW<StgSyscfg231Spec> {
        U1PcieTlClockFreqW::new(self, 4)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 924\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_231::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_231::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg231Spec;
impl crate::RegisterSpec for StgSyscfg231Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_231::R`](R) reader structure"]
impl crate::Readable for StgSyscfg231Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_231::W`](W) writer structure"]
impl crate::Writable for StgSyscfg231Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_231 to value 0x0c80"]
impl crate::Resettable for StgSyscfg231Spec {
    const RESET_VALUE: u32 = 0x0c80;
}
