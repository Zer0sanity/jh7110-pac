#[doc = "Register `dscaddr_idinten64` reader"]
pub type R = crate::R<DscaddrIdinten64Spec>;
#[doc = "Register `dscaddr_idinten64` writer"]
pub type W = crate::W<DscaddrIdinten64Spec>;
#[doc = "Field `dscaddr_idinten64` reader - MMC Internal DMAC DSC address / interrupt enable"]
pub type DscaddrIdinten64R = crate::FieldReader<u32>;
#[doc = "Field `dscaddr_idinten64` writer - MMC Internal DMAC DSC address / interrupt enable"]
pub type DscaddrIdinten64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Internal DMAC DSC address / interrupt enable"]
    #[inline(always)]
    pub fn dscaddr_idinten64(&self) -> DscaddrIdinten64R {
        DscaddrIdinten64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Internal DMAC DSC address / interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dscaddr_idinten64(&mut self) -> DscaddrIdinten64W<DscaddrIdinten64Spec> {
        DscaddrIdinten64W::new(self, 0)
    }
}
#[doc = "MMC internal DMAC DSC address / interrupt enable - HCON\\[ADDR_CONFIG\\]
32-bit(0): DSC address, HCON\\[ADDR_CONFIG\\]
64-bit(1): interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscaddr_idinten64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscaddr_idinten64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscaddrIdinten64Spec;
impl crate::RegisterSpec for DscaddrIdinten64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscaddr_idinten64::R`](R) reader structure"]
impl crate::Readable for DscaddrIdinten64Spec {}
#[doc = "`write(|w| ..)` method takes [`dscaddr_idinten64::W`](W) writer structure"]
impl crate::Writable for DscaddrIdinten64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dscaddr_idinten64 to value 0"]
impl crate::Resettable for DscaddrIdinten64Spec {
    const RESET_VALUE: u32 = 0;
}
