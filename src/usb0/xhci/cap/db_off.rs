#[doc = "Register `db_off` reader"]
pub type R = crate::R<DbOffSpec>;
#[doc = "Field `db_off` reader - USB3 XHCI host controller doorbell array offset."]
pub type DbOffR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 XHCI host controller doorbell array offset."]
    #[inline(always)]
    pub fn db_off(&self) -> DbOffR {
        DbOffR::new(self.bits)
    }
}
#[doc = "USB3 XHCI host controller doorbell array offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`db_off::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbOffSpec;
impl crate::RegisterSpec for DbOffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`db_off::R`](R) reader structure"]
impl crate::Readable for DbOffSpec {}
#[doc = "`reset()` method sets db_off to value 0"]
impl crate::Resettable for DbOffSpec {
    const RESET_VALUE: u32 = 0;
}
