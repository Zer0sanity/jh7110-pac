#[doc = "Register `idinten_idsts64` reader"]
pub type R = crate::R<IdintenIdsts64Spec>;
#[doc = "Register `idinten_idsts64` writer"]
pub type W = crate::W<IdintenIdsts64Spec>;
#[doc = "Field `idinten_idsts64` reader - MMC Internal DMAC interrupt enable / status"]
pub type IdintenIdsts64R = crate::FieldReader<u32>;
#[doc = "Field `idinten_idsts64` writer - MMC Internal DMAC interrupt enable / status"]
pub type IdintenIdsts64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Internal DMAC interrupt enable / status"]
    #[inline(always)]
    pub fn idinten_idsts64(&self) -> IdintenIdsts64R {
        IdintenIdsts64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Internal DMAC interrupt enable / status"]
    #[inline(always)]
    #[must_use]
    pub fn idinten_idsts64(&mut self) -> IdintenIdsts64W<IdintenIdsts64Spec> {
        IdintenIdsts64W::new(self, 0)
    }
}
#[doc = "MMC internal DMAC interrupt enable / status - HCON\\[ADDR_CONFIG\\]
32-bit(0): interrupt enable, HCON\\[ADDR_CONFIG\\]
64-bit(1): status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idinten_idsts64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idinten_idsts64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdintenIdsts64Spec;
impl crate::RegisterSpec for IdintenIdsts64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idinten_idsts64::R`](R) reader structure"]
impl crate::Readable for IdintenIdsts64Spec {}
#[doc = "`write(|w| ..)` method takes [`idinten_idsts64::W`](W) writer structure"]
impl crate::Writable for IdintenIdsts64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets idinten_idsts64 to value 0"]
impl crate::Resettable for IdintenIdsts64Spec {
    const RESET_VALUE: u32 = 0;
}
