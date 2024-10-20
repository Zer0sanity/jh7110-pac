#[doc = "Register `safety_int_status` reader"]
pub type R = crate::R<SafetyIntStatusSpec>;
#[doc = "Register `safety_int_status` writer"]
pub type W = crate::W<SafetyIntStatusSpec>;
#[doc = "Field `mecis` reader - MTL MEC Interrupt Status - Write 1 to clear interrupt"]
pub type MecisR = crate::BitReader;
#[doc = "Field `mecis` writer - MTL MEC Interrupt Status - Write 1 to clear interrupt"]
pub type MecisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `meuis` reader - MTL MEU Interrupt Status - Write 1 to clear interrupt"]
pub type MeuisR = crate::BitReader;
#[doc = "Field `meuis` writer - MTL MEU Interrupt Status - Write 1 to clear interrupt"]
pub type MeuisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcsis` reader - MTL MCS Interrupt Status - Write 1 to clear interrupt"]
pub type McsisR = crate::BitReader;
#[doc = "Field `mcsis` writer - MTL MCS Interrupt Status - Write 1 to clear interrupt"]
pub type McsisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MTL MEC Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn mecis(&self) -> MecisR {
        MecisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MTL MEU Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn meuis(&self) -> MeuisR {
        MeuisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - MTL MCS Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn mcsis(&self) -> McsisR {
        McsisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MTL MEC Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mecis(&mut self) -> MecisW<SafetyIntStatusSpec> {
        MecisW::new(self, 0)
    }
    #[doc = "Bit 1 - MTL MEU Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn meuis(&mut self) -> MeuisW<SafetyIntStatusSpec> {
        MeuisW::new(self, 1)
    }
    #[doc = "Bit 31 - MTL MCS Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mcsis(&mut self) -> McsisW<SafetyIntStatusSpec> {
        McsisW::new(self, 31)
    }
}
#[doc = "MTL Safety Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`safety_int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`safety_int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SafetyIntStatusSpec;
impl crate::RegisterSpec for SafetyIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`safety_int_status::R`](R) reader structure"]
impl crate::Readable for SafetyIntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`safety_int_status::W`](W) writer structure"]
impl crate::Writable for SafetyIntStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets safety_int_status to value 0"]
impl crate::Resettable for SafetyIntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
