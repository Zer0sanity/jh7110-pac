#[doc = "Register `sid` reader"]
pub type R = crate::R<SidSpec>;
#[doc = "Register `sid` writer"]
pub type W = crate::W<SidSpec>;
#[doc = "Field `sid` reader - Stream ID - used only in SS mode."]
pub type SidR = crate::FieldReader<u16>;
#[doc = "Field `sid` writer - Stream ID - used only in SS mode."]
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Stream ID - used only in SS mode."]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Stream ID - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn sid(&mut self) -> SidW<SidSpec> {
        SidW::new(self, 0)
    }
}
#[doc = "Endpoint status stream ID - used only in SS mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidSpec;
impl crate::RegisterSpec for SidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sid::R`](R) reader structure"]
impl crate::Readable for SidSpec {}
#[doc = "`write(|w| ..)` method takes [`sid::W`](W) writer structure"]
impl crate::Writable for SidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sid to value 0"]
impl crate::Resettable for SidSpec {
    const RESET_VALUE: u32 = 0;
}
