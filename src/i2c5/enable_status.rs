#[doc = "Register `enable_status` reader"]
pub type R = crate::R<EnableStatusSpec>;
#[doc = "Register `enable_status` writer"]
pub type W = crate::W<EnableStatusSpec>;
#[doc = "Field `activity` reader - "]
pub type ActivityR = crate::BitReader;
#[doc = "Field `activity` writer - "]
pub type ActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tfe` reader - "]
pub type TfeR = crate::BitReader;
#[doc = "Field `tfe` writer - "]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfne` reader - "]
pub type RfneR = crate::BitReader;
#[doc = "Field `rfne` writer - "]
pub type RfneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `master_activity` reader - "]
pub type MasterActivityR = crate::BitReader;
#[doc = "Field `master_activity` writer - "]
pub type MasterActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slave_activity` reader - "]
pub type SlaveActivityR = crate::BitReader;
#[doc = "Field `slave_activity` writer - "]
pub type SlaveActivityW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn activity(&mut self) -> ActivityW<EnableStatusSpec> {
        ActivityW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<EnableStatusSpec> {
        TfeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rfne(&mut self) -> RfneW<EnableStatusSpec> {
        RfneW::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn master_activity(&mut self) -> MasterActivityW<EnableStatusSpec> {
        MasterActivityW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn slave_activity(&mut self) -> SlaveActivityW<EnableStatusSpec> {
        SlaveActivityW::new(self, 6)
    }
}
#[doc = "DesignWare I2C Enable Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableStatusSpec;
impl crate::RegisterSpec for EnableStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_status::R`](R) reader structure"]
impl crate::Readable for EnableStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`enable_status::W`](W) writer structure"]
impl crate::Writable for EnableStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets enable_status to value 0"]
impl crate::Resettable for EnableStatusSpec {
    const RESET_VALUE: u32 = 0;
}
