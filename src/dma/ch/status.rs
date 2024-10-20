#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `cmpltd_blk_tr_size` reader - Completed Block Transfer Size."]
pub type CmpltdBlkTrSizeR = crate::FieldReader<u32>;
#[doc = "Field `data_left_in_fifo` reader - Data left in FIFO"]
pub type DataLeftInFifoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:21 - Completed Block Transfer Size."]
    #[inline(always)]
    pub fn cmpltd_blk_tr_size(&self) -> CmpltdBlkTrSizeR {
        CmpltdBlkTrSizeR::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 32:46 - Data left in FIFO"]
    #[inline(always)]
    pub fn data_left_in_fifo(&self) -> DataLeftInFifoR {
        DataLeftInFifoR::new(((self.bits >> 32) & 0x7fff) as u16)
    }
}
#[doc = "DMAC Channel Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u64 = 0;
}
