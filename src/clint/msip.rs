#[doc = "Register `msip[%s]` reader"]
pub type R = crate::R<MsipSpec>;
#[doc = "Register `msip[%s]` writer"]
pub type W = crate::W<MsipSpec>;
#[doc = "Field `control` reader - "]
pub type ControlR = crate::BitReader;
#[doc = "Field `control` writer - "]
pub type ControlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn control(&self) -> ControlR {
        ControlR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn control(&mut self) -> ControlW<MsipSpec> {
        ControlW::new(self, 0)
    }
}
#[doc = "CLINT MSIP (Machine Software Interrupt) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsipSpec;
impl crate::RegisterSpec for MsipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msip::R`](R) reader structure"]
impl crate::Readable for MsipSpec {}
#[doc = "`write(|w| ..)` method takes [`msip::W`](W) writer structure"]
impl crate::Writable for MsipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets msip[%s]
to value 0"]
impl crate::Resettable for MsipSpec {
    const RESET_VALUE: u32 = 0;
}
