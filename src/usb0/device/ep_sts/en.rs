#[doc = "Register `en` reader"]
pub type R = crate::R<EnSpec>;
#[doc = "Register `en` writer"]
pub type W = crate::W<EnSpec>;
#[doc = "Field `setup` reader - Stream SETUP interrupt enable."]
pub type SetupR = crate::BitReader;
#[doc = "Field `setup` writer - Stream SETUP interrupt enable."]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `descmis` reader - OUT transfer descriptor missing enable."]
pub type DescmisR = crate::BitReader;
#[doc = "Field `descmis` writer - OUT transfer descriptor missing enable."]
pub type DescmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `streamr` reader - Stream Rejected enable."]
pub type StreamrR = crate::BitReader;
#[doc = "Field `streamr` writer - Stream Rejected enable."]
pub type StreamrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `md_exit` reader - Move Data Exit enable."]
pub type MdExitR = crate::BitReader;
#[doc = "Field `md_exit` writer - Move Data Exit enable."]
pub type MdExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trberr` reader - TRB Error enable."]
pub type TrberrR = crate::BitReader;
#[doc = "Field `trberr` writer - TRB Error enable."]
pub type TrberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nrdy` reader - Not Ready enable."]
pub type NrdyR = crate::BitReader;
#[doc = "Field `nrdy` writer - Not Ready enable."]
pub type NrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prime` reader - Prime enable."]
pub type PrimeR = crate::BitReader;
#[doc = "Field `prime` writer - Prime enable."]
pub type PrimeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `siderr` reader - Stream error enable."]
pub type SiderrR = crate::BitReader;
#[doc = "Field `siderr` writer - Stream error enable."]
pub type SiderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `outsmm` reader - OUT size mismatch enable."]
pub type OutsmmR = crate::BitReader;
#[doc = "Field `outsmm` writer - OUT size mismatch enable."]
pub type OutsmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isoerr` reader - ISO transmission error enable."]
pub type IsoerrR = crate::BitReader;
#[doc = "Field `isoerr` writer - ISO transmission error enable."]
pub type IsoerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `iot` reader - Interrtup On Transmission enable."]
pub type IotR = crate::BitReader;
#[doc = "Field `iot` writer - Interrtup On Transmission enable."]
pub type IotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stpwait` reader - Setup Wait interrupt enable."]
pub type StpwaitR = crate::BitReader;
#[doc = "Field `stpwait` writer - Setup Wait interrupt enable."]
pub type StpwaitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream SETUP interrupt enable."]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - OUT transfer descriptor missing enable."]
    #[inline(always)]
    pub fn descmis(&self) -> DescmisR {
        DescmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream Rejected enable."]
    #[inline(always)]
    pub fn streamr(&self) -> StreamrR {
        StreamrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Move Data Exit enable."]
    #[inline(always)]
    pub fn md_exit(&self) -> MdExitR {
        MdExitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TRB Error enable."]
    #[inline(always)]
    pub fn trberr(&self) -> TrberrR {
        TrberrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Ready enable."]
    #[inline(always)]
    pub fn nrdy(&self) -> NrdyR {
        NrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Prime enable."]
    #[inline(always)]
    pub fn prime(&self) -> PrimeR {
        PrimeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stream error enable."]
    #[inline(always)]
    pub fn siderr(&self) -> SiderrR {
        SiderrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OUT size mismatch enable."]
    #[inline(always)]
    pub fn outsmm(&self) -> OutsmmR {
        OutsmmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ISO transmission error enable."]
    #[inline(always)]
    pub fn isoerr(&self) -> IsoerrR {
        IsoerrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrtup On Transmission enable."]
    #[inline(always)]
    pub fn iot(&self) -> IotR {
        IotR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - Setup Wait interrupt enable."]
    #[inline(always)]
    pub fn stpwait(&self) -> StpwaitR {
        StpwaitR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stream SETUP interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SetupW<EnSpec> {
        SetupW::new(self, 0)
    }
    #[doc = "Bit 4 - OUT transfer descriptor missing enable."]
    #[inline(always)]
    #[must_use]
    pub fn descmis(&mut self) -> DescmisW<EnSpec> {
        DescmisW::new(self, 4)
    }
    #[doc = "Bit 5 - Stream Rejected enable."]
    #[inline(always)]
    #[must_use]
    pub fn streamr(&mut self) -> StreamrW<EnSpec> {
        StreamrW::new(self, 5)
    }
    #[doc = "Bit 6 - Move Data Exit enable."]
    #[inline(always)]
    #[must_use]
    pub fn md_exit(&mut self) -> MdExitW<EnSpec> {
        MdExitW::new(self, 6)
    }
    #[doc = "Bit 7 - TRB Error enable."]
    #[inline(always)]
    #[must_use]
    pub fn trberr(&mut self) -> TrberrW<EnSpec> {
        TrberrW::new(self, 7)
    }
    #[doc = "Bit 8 - Not Ready enable."]
    #[inline(always)]
    #[must_use]
    pub fn nrdy(&mut self) -> NrdyW<EnSpec> {
        NrdyW::new(self, 8)
    }
    #[doc = "Bit 12 - Prime enable."]
    #[inline(always)]
    #[must_use]
    pub fn prime(&mut self) -> PrimeW<EnSpec> {
        PrimeW::new(self, 12)
    }
    #[doc = "Bit 13 - Stream error enable."]
    #[inline(always)]
    #[must_use]
    pub fn siderr(&mut self) -> SiderrW<EnSpec> {
        SiderrW::new(self, 13)
    }
    #[doc = "Bit 14 - OUT size mismatch enable."]
    #[inline(always)]
    #[must_use]
    pub fn outsmm(&mut self) -> OutsmmW<EnSpec> {
        OutsmmW::new(self, 14)
    }
    #[doc = "Bit 15 - ISO transmission error enable."]
    #[inline(always)]
    #[must_use]
    pub fn isoerr(&mut self) -> IsoerrW<EnSpec> {
        IsoerrW::new(self, 15)
    }
    #[doc = "Bit 19 - Interrtup On Transmission enable."]
    #[inline(always)]
    #[must_use]
    pub fn iot(&mut self) -> IotW<EnSpec> {
        IotW::new(self, 19)
    }
    #[doc = "Bit 31 - Setup Wait interrupt enable."]
    #[inline(always)]
    #[must_use]
    pub fn stpwait(&mut self) -> StpwaitW<EnSpec> {
        StpwaitW::new(self, 31)
    }
}
#[doc = "Endpoint status enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnSpec;
impl crate::RegisterSpec for EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EnSpec {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets en to value 0"]
impl crate::Resettable for EnSpec {
    const RESET_VALUE: u32 = 0;
}
