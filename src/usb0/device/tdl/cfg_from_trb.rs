#[doc = "Register `cfg_from_trb` reader"]
pub type R = crate::R<CfgFromTrbSpec>;
#[doc = "Register `cfg_from_trb` writer"]
pub type W = crate::W<CfgFromTrbSpec>;
#[doc = "Field `cfg_from_trb` reader - TDL configuration source."]
pub type CfgFromTrbR = crate::FieldReader<u32>;
#[doc = "Field `cfg_from_trb` writer - TDL configuration source."]
pub type CfgFromTrbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TDL configuration source."]
    #[inline(always)]
    pub fn cfg_from_trb(&self) -> CfgFromTrbR {
        CfgFromTrbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TDL configuration source."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_from_trb(&mut self) -> CfgFromTrbW<CfgFromTrbSpec> {
        CfgFromTrbW::new(self, 0)
    }
}
#[doc = "TDL configuration source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_from_trb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_from_trb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgFromTrbSpec;
impl crate::RegisterSpec for CfgFromTrbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_from_trb::R`](R) reader structure"]
impl crate::Readable for CfgFromTrbSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_from_trb::W`](W) writer structure"]
impl crate::Writable for CfgFromTrbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cfg_from_trb to value 0"]
impl crate::Resettable for CfgFromTrbSpec {
    const RESET_VALUE: u32 = 0;
}
