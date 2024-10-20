#[doc = "Register `stg_syscfg_4` reader"]
pub type R = crate::R<StgSyscfg4Spec>;
#[doc = "Field `u0_usb_xhci_debug_bus` reader - "]
pub type U0UsbXhciDebugBusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn u0_usb_xhci_debug_bus(&self) -> U0UsbXhciDebugBusR {
        U0UsbXhciDebugBusR::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg4Spec;
impl crate::RegisterSpec for StgSyscfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_4::R`](R) reader structure"]
impl crate::Readable for StgSyscfg4Spec {}
#[doc = "`reset()` method sets stg_syscfg_4 to value 0"]
impl crate::Resettable for StgSyscfg4Spec {
    const RESET_VALUE: u32 = 0;
}
