#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `activity` reader - "]
pub type ActivityR = crate::BitReader;
#[doc = "Field `tfe` reader - "]
pub type TfeR = crate::BitReader;
#[doc = "Field `rfne` reader - "]
pub type RfneR = crate::BitReader;
#[doc = "Field `master_activity` reader - "]
pub type MasterActivityR = crate::BitReader;
#[doc = "Field `slave_activity` reader - "]
pub type SlaveActivityR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn activity(&self) -> ActivityR {
        ActivityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfne(&self) -> RfneR {
        RfneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn master_activity(&self) -> MasterActivityR {
        MasterActivityR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slave_activity(&self) -> SlaveActivityR {
        SlaveActivityR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "DesignWare I2C Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
