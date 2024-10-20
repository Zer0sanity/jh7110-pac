#[doc = "Register `iv[%s]` reader"]
pub type R = crate::R<IvSpec>;
#[doc = "Register `iv[%s]` writer"]
pub type W = crate::W<IvSpec>;
#[doc = "Field `iv` reader - "]
pub type IvR = crate::FieldReader<u32>;
#[doc = "Field `iv` writer - "]
pub type IvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iv(&self) -> IvR {
        IvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IvW<IvSpec> {
        IvW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto AES IV\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvSpec;
impl crate::RegisterSpec for IvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iv::R`](R) reader structure"]
impl crate::Readable for IvSpec {}
#[doc = "`write(|w| ..)` method takes [`iv::W`](W) writer structure"]
impl crate::Writable for IvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets iv[%s]
to value 0"]
impl crate::Resettable for IvSpec {
    const RESET_VALUE: u32 = 0;
}
