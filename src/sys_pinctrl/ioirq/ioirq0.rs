#[doc = "Register `ioirq0` reader"]
pub type R = crate::R<Ioirq0Spec>;
#[doc = "Register `ioirq0` writer"]
pub type W = crate::W<Ioirq0Spec>;
#[doc = "Field `gpen0` reader - 1: Enable, 0: Disable"]
pub type Gpen0R = crate::BitReader;
#[doc = "Field `gpen0` writer - 1: Enable, 0: Disable"]
pub type Gpen0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: Enable, 0: Disable"]
    #[inline(always)]
    pub fn gpen0(&self) -> Gpen0R {
        Gpen0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable, 0: Disable"]
    #[inline(always)]
    #[must_use]
    pub fn gpen0(&mut self) -> Gpen0W<Ioirq0Spec> {
        Gpen0W::new(self, 0)
    }
}
#[doc = "Enable IRQ function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq0Spec;
impl crate::RegisterSpec for Ioirq0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq0::R`](R) reader structure"]
impl crate::Readable for Ioirq0Spec {}
#[doc = "`write(|w| ..)` method takes [`ioirq0::W`](W) writer structure"]
impl crate::Writable for Ioirq0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq0 to value 0"]
impl crate::Resettable for Ioirq0Spec {
    const RESET_VALUE: u32 = 0;
}
