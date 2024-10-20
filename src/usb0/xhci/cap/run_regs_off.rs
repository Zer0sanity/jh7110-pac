#[doc = "Register `run_regs_off` reader"]
pub type R = crate::R<RunRegsOffSpec>;
#[doc = "Field `run_regs_off` reader - USB3 XHCI host controller `run` register cluster offset."]
pub type RunRegsOffR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 XHCI host controller `run` register cluster offset."]
    #[inline(always)]
    pub fn run_regs_off(&self) -> RunRegsOffR {
        RunRegsOffR::new(self.bits)
    }
}
#[doc = "USB3 XHCI host controller run register cluster offset - runtime register space offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`run_regs_off::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RunRegsOffSpec;
impl crate::RegisterSpec for RunRegsOffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`run_regs_off::R`](R) reader structure"]
impl crate::Readable for RunRegsOffSpec {}
#[doc = "`reset()` method sets run_regs_off to value 0"]
impl crate::Resettable for RunRegsOffSpec {
    const RESET_VALUE: u32 = 0;
}
