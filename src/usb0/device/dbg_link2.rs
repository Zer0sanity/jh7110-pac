#[doc = "Register `dbg_link2` reader"]
pub type R = crate::R<DbgLink2Spec>;
#[doc = "Register `dbg_link2` writer"]
pub type W = crate::W<DbgLink2Spec>;
#[doc = "Field `dbg_link2` reader - Device debug link 2."]
pub type DbgLink2R = crate::FieldReader<u32>;
#[doc = "Field `dbg_link2` writer - Device debug link 2."]
pub type DbgLink2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Device debug link 2."]
    #[inline(always)]
    pub fn dbg_link2(&self) -> DbgLink2R {
        DbgLink2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device debug link 2."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_link2(&mut self) -> DbgLink2W<DbgLink2Spec> {
        DbgLink2W::new(self, 0)
    }
}
#[doc = "Device debug link 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_link2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_link2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgLink2Spec;
impl crate::RegisterSpec for DbgLink2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_link2::R`](R) reader structure"]
impl crate::Readable for DbgLink2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_link2::W`](W) writer structure"]
impl crate::Writable for DbgLink2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dbg_link2 to value 0"]
impl crate::Resettable for DbgLink2Spec {
    const RESET_VALUE: u32 = 0;
}
