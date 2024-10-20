#[doc = "Register `base[%s]` reader"]
pub type R = crate::R<BaseSpec>;
#[doc = "Register `base[%s]` writer"]
pub type W = crate::W<BaseSpec>;
#[doc = "Field `base` reader - "]
pub type BaseR = crate::FieldReader<u32>;
#[doc = "Field `base` writer - "]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<BaseSpec> {
        BaseW::new(self, 0)
    }
}
#[doc = "DDR Memory Control PHY Base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseSpec;
impl crate::RegisterSpec for BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base::R`](R) reader structure"]
impl crate::Readable for BaseSpec {}
#[doc = "`write(|w| ..)` method takes [`base::W`](W) writer structure"]
impl crate::Writable for BaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets base[%s]
to value 0"]
impl crate::Resettable for BaseSpec {
    const RESET_VALUE: u32 = 0;
}
