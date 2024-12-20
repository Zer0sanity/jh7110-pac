#[doc = "Register `cfg[%s]` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `cfg[%s]` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `cfg` reader - Device configuration."]
pub type CfgR = crate::FieldReader<u32>;
#[doc = "Field `cfg` writer - Device configuration."]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device configuration."]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device configuration."]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<CfgSpec> {
        CfgW::new(self, 0)
    }
}
#[doc = "Device configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cfg[%s]
to value 0"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0;
}
