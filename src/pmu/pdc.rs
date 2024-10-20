#[doc = "Register `pdc[%s]` reader"]
pub type R = crate::R<PdcSpec>;
#[doc = "Register `pdc[%s]` writer"]
pub type W = crate::W<PdcSpec>;
#[doc = "Field `pd0_off_cas` reader - Power domain 0 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd0OffCasR = crate::FieldReader;
#[doc = "Field `pd0_off_cas` writer - Power domain 0 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd0OffCasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd0_on_cas` reader - Power domain 0 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd0OnCasR = crate::FieldReader;
#[doc = "Field `pd0_on_cas` writer - Power domain 0 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd0OnCasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd1_off_cas` reader - Power domain 1 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd1OffCasR = crate::FieldReader;
#[doc = "Field `pd1_off_cas` writer - Power domain 1 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd1OffCasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd1_on_cas` reader - Power domain 1 turn-off cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd1OnCasR = crate::FieldReader;
#[doc = "Field `pd1_on_cas` writer - Power domain 1 turn-off cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd1OnCasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd2_off_cas` reader - Power domain 2 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd2OffCasR = crate::FieldReader;
#[doc = "Field `pd2_off_cas` writer - Power domain 2 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd2OffCasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `pd2_on_cas` reader - Power domain 1 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd2OnCasR = crate::FieldReader;
#[doc = "Field `pd2_on_cas` writer - Power domain 1 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
pub type Pd2OnCasW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Power domain 0 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    pub fn pd0_off_cas(&self) -> Pd0OffCasR {
        Pd0OffCasR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Power domain 0 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    pub fn pd0_on_cas(&self) -> Pd0OnCasR {
        Pd0OnCasR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Power domain 1 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    pub fn pd1_off_cas(&self) -> Pd1OffCasR {
        Pd1OffCasR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Power domain 1 turn-off cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    pub fn pd1_on_cas(&self) -> Pd1OnCasR {
        Pd1OnCasR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Power domain 2 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    pub fn pd2_off_cas(&self) -> Pd2OffCasR {
        Pd2OffCasR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - Power domain 1 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    pub fn pd2_on_cas(&self) -> Pd2OnCasR {
        Pd2OnCasR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Power domain 0 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd0_off_cas(&mut self) -> Pd0OffCasW<PdcSpec> {
        Pd0OffCasW::new(self, 0)
    }
    #[doc = "Bits 5:9 - Power domain 0 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd0_on_cas(&mut self) -> Pd0OnCasW<PdcSpec> {
        Pd0OnCasW::new(self, 5)
    }
    #[doc = "Bits 10:14 - Power domain 1 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd1_off_cas(&mut self) -> Pd1OffCasW<PdcSpec> {
        Pd1OffCasW::new(self, 10)
    }
    #[doc = "Bits 15:19 - Power domain 1 turn-off cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd1_on_cas(&mut self) -> Pd1OnCasW<PdcSpec> {
        Pd1OnCasW::new(self, 15)
    }
    #[doc = "Bits 20:24 - Power domain 2 turn-off cascade. The register value indicates the power-off sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd2_off_cas(&mut self) -> Pd2OffCasW<PdcSpec> {
        Pd2OffCasW::new(self, 20)
    }
    #[doc = "Bits 25:29 - Power domain 1 turn-on cascade. The register value indicates the power-on sequence of this domain. 0 means the highest priority. System only accepts value from 0 to 7 any other value is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn pd2_on_cas(&mut self) -> Pd2OnCasW<PdcSpec> {
        Pd2OnCasW::new(self, 25)
    }
}
#[doc = "Power Domain Cascade register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdcSpec;
impl crate::RegisterSpec for PdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdc::R`](R) reader structure"]
impl crate::Readable for PdcSpec {}
#[doc = "`write(|w| ..)` method takes [`pdc::W`](W) writer structure"]
impl crate::Writable for PdcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pdc[%s]
to value 0"]
impl crate::Resettable for PdcSpec {
    const RESET_VALUE: u32 = 0;
}
