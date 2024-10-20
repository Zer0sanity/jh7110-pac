#[doc = "Register `safety_int_status` reader"]
pub type R = crate::R<SafetyIntStatusSpec>;
#[doc = "Register `safety_int_status` writer"]
pub type W = crate::W<SafetyIntStatusSpec>;
#[doc = "Field `decis` reader - DMA DEC Interrupt Status - Write 1 to clear interrupt"]
pub type DecisR = crate::BitReader;
#[doc = "Field `decis` writer - DMA DEC Interrupt Status - Write 1 to clear interrupt"]
pub type DecisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `deuis` reader - DMA MEU Interrupt Status - Write 1 to clear interrupt"]
pub type DeuisR = crate::BitReader;
#[doc = "Field `deuis` writer - DMA MEU Interrupt Status - Write 1 to clear interrupt"]
pub type DeuisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mscis` reader - DMA MSC Interrupt Status - Write 1 to clear interrupt"]
pub type MscisR = crate::BitReader;
#[doc = "Field `mscis` writer - DMA MSC Interrupt Status - Write 1 to clear interrupt"]
pub type MscisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `msuis` reader - DMA MSU Interrupt Status - Write 1 to clear interrupt"]
pub type MsuisR = crate::BitReader;
#[doc = "Field `msuis` writer - DMA MSU Interrupt Status - Write 1 to clear interrupt"]
pub type MsuisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA DEC Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn decis(&self) -> DecisR {
        DecisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA MEU Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn deuis(&self) -> DeuisR {
        DeuisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA MSC Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn mscis(&self) -> MscisR {
        MscisR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA MSU Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn msuis(&self) -> MsuisR {
        MsuisR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA DEC Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn decis(&mut self) -> DecisW<SafetyIntStatusSpec> {
        DecisW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA MEU Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn deuis(&mut self) -> DeuisW<SafetyIntStatusSpec> {
        DeuisW::new(self, 1)
    }
    #[doc = "Bit 28 - DMA MSC Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mscis(&mut self) -> MscisW<SafetyIntStatusSpec> {
        MscisW::new(self, 28)
    }
    #[doc = "Bit 29 - DMA MSU Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn msuis(&mut self) -> MsuisW<SafetyIntStatusSpec> {
        MsuisW::new(self, 29)
    }
}
#[doc = "DMA Safety Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`safety_int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`safety_int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
