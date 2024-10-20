#[doc = "Register `ims` reader"]
pub type R = crate::R<ImsSpec>;
#[doc = "Field `ims` reader - interrupt masked status from the watchdog counter."]
pub type ImsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - interrupt masked status from the watchdog counter."]
    #[inline(always)]
    pub fn ims(&self) -> ImsR {
        ImsR::new(self.bits)
    }
}
#[doc = "StarFive JH7110 Watchdog Interrupt Masked Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ims::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImsSpec;
impl crate::RegisterSpec for ImsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ims::R`](R) reader structure"]
impl crate::Readable for ImsSpec {}
#[doc = "`reset()` method sets ims to value 0"]
impl crate::Resettable for ImsSpec {
    const RESET_VALUE: u32 = 0;
}
