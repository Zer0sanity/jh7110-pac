#[doc = "Register `ext_lower` reader"]
pub type R = crate::R<ExtLowerSpec>;
#[doc = "Register `ext_lower` writer"]
pub type W = crate::W<ExtLowerSpec>;
#[doc = "Field `stig` reader - "]
pub type StigR = crate::FieldReader<u16>;
#[doc = "Field `stig` writer - "]
pub type StigW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `write` reader - "]
pub type WriteR = crate::FieldReader;
#[doc = "Field `write` writer - "]
pub type WriteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `read` reader - "]
pub type ReadR = crate::FieldReader;
#[doc = "Field `read` writer - "]
pub type ReadW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn stig(&self) -> StigR {
        StigR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn stig(&mut self) -> StigW<ExtLowerSpec> {
        StigW::new(self, 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<ExtLowerSpec> {
        WriteW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<ExtLowerSpec> {
        ReadW::new(self, 24)
    }
}
#[doc = "Cadence QSPI Extension Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtLowerSpec;
impl crate::RegisterSpec for ExtLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_lower::R`](R) reader structure"]
impl crate::Readable for ExtLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_lower::W`](W) writer structure"]
impl crate::Writable for ExtLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ext_lower to value 0"]
impl crate::Resettable for ExtLowerSpec {
    const RESET_VALUE: u32 = 0;
}
