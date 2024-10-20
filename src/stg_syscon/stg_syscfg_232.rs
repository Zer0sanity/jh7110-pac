#[doc = "Register `stg_syscfg_232` reader"]
pub type R = crate::R<StgSyscfg232Spec>;
#[doc = "Register `stg_syscfg_232` writer"]
pub type W = crate::W<StgSyscfg232Spec>;
#[doc = "Field `u1_pcie_tl_ctrl_hotplug` reader - PCIE TL Control Hotplug"]
pub type U1PcieTlCtrlHotplugR = crate::FieldReader<u16>;
#[doc = "Field `u1_pcie_tl_report_hotplug` reader - PCIE TL Report Hotplug"]
pub type U1PcieTlReportHotplugR = crate::FieldReader<u16>;
#[doc = "Field `u1_pcie_tl_report_hotplug` writer - PCIE TL Report Hotplug"]
pub type U1PcieTlReportHotplugW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - PCIE TL Control Hotplug"]
    #[inline(always)]
    pub fn u1_pcie_tl_ctrl_hotplug(&self) -> U1PcieTlCtrlHotplugR {
        U1PcieTlCtrlHotplugR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PCIE TL Report Hotplug"]
    #[inline(always)]
    pub fn u1_pcie_tl_report_hotplug(&self) -> U1PcieTlReportHotplugR {
        U1PcieTlReportHotplugR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - PCIE TL Report Hotplug"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_tl_report_hotplug(&mut self) -> U1PcieTlReportHotplugW<StgSyscfg232Spec> {
        U1PcieTlReportHotplugW::new(self, 16)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 928\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_232::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_232::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg232Spec;
impl crate::RegisterSpec for StgSyscfg232Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_232::R`](R) reader structure"]
impl crate::Readable for StgSyscfg232Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_232::W`](W) writer structure"]
impl crate::Writable for StgSyscfg232Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_232 to value 0"]
impl crate::Resettable for StgSyscfg232Spec {
    const RESET_VALUE: u32 = 0;
}
