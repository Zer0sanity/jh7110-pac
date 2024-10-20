#[doc = "Register `ac_base[%s]` reader"]
pub type R = crate::R<AcBaseSpec>;
#[doc = "Register `ac_base[%s]` writer"]
pub type W = crate::W<AcBaseSpec>;
#[doc = "Field `ac_base` reader - "]
pub type AcBaseR = crate::FieldReader<u32>;
#[doc = "Field `ac_base` writer - "]
pub type AcBaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ac_base(&self) -> AcBaseR {
        AcBaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ac_base(&mut self) -> AcBaseW<AcBaseSpec> {
        AcBaseW::new(self, 0)
    }
}
#[doc = "DDR Memory Control PHY AC Base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcBaseSpec;
impl crate::RegisterSpec for AcBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac_base::R`](R) reader structure"]
impl crate::Readable for AcBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`ac_base::W`](W) writer structure"]
impl crate::Writable for AcBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ac_base[%s]
to value 0"]
impl crate::Resettable for AcBaseSpec {
    const RESET_VALUE: u32 = 0;
}
