#[doc = "Register `stg_syscfg_5` reader"]
pub type R = crate::R<StgSyscfg5Spec>;
#[doc = "Field `u0_usb_xhci_debug_link_state` reader - "]
pub type U0UsbXhciDebugLinkStateR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn u0_usb_xhci_debug_link_state(&self) -> U0UsbXhciDebugLinkStateR {
        U0UsbXhciDebugLinkStateR::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg5Spec;
impl crate::RegisterSpec for StgSyscfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_5::R`](R) reader structure"]
impl crate::Readable for StgSyscfg5Spec {}
#[doc = "`reset()` method sets stg_syscfg_5 to value 0"]
impl crate::Resettable for StgSyscfg5Spec {
    const RESET_VALUE: u32 = 0;
}
