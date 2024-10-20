#[doc = "Register `resp[%s]` reader"]
pub type R = crate::R<RespSpec>;
#[doc = "Register `resp[%s]` writer"]
pub type W = crate::W<RespSpec>;
#[doc = "Field `resp` reader - MMC response"]
pub type RespR = crate::FieldReader<u32>;
#[doc = "Field `resp` writer - MMC response"]
pub type RespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC response"]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC response"]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<RespSpec> {
        RespW::new(self, 0)
    }
}
#[doc = "MMC response\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RespSpec;
impl crate::RegisterSpec for RespSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp::R`](R) reader structure"]
impl crate::Readable for RespSpec {}
#[doc = "`write(|w| ..)` method takes [`resp::W`](W) writer structure"]
impl crate::Writable for RespSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets resp[%s]
to value 0"]
impl crate::Resettable for RespSpec {
    const RESET_VALUE: u32 = 0;
}
