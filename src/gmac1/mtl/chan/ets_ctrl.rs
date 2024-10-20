#[doc = "Register `ets_ctrl` reader"]
pub type R = crate::R<EtsCtrlSpec>;
#[doc = "Register `ets_ctrl` writer"]
pub type W = crate::W<EtsCtrlSpec>;
#[doc = "Field `avalg` reader - MTL Channel ETS AV Algorithm"]
pub type AvalgR = crate::BitReader;
#[doc = "Field `avalg` writer - MTL Channel ETS AV Algorithm"]
pub type AvalgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cc` reader - MTL Channel ETS CC"]
pub type CcR = crate::BitReader;
#[doc = "Field `cc` writer - MTL Channel ETS CC"]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - MTL Channel ETS AV Algorithm"]
    #[inline(always)]
    pub fn avalg(&self) -> AvalgR {
        AvalgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MTL Channel ETS CC"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - MTL Channel ETS AV Algorithm"]
    #[inline(always)]
    #[must_use]
    pub fn avalg(&mut self) -> AvalgW<EtsCtrlSpec> {
        AvalgW::new(self, 2)
    }
    #[doc = "Bit 3 - MTL Channel ETS CC"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<EtsCtrlSpec> {
        CcW::new(self, 3)
    }
}
#[doc = "MTL Channel ETS Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ets_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ets_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtsCtrlSpec;
impl crate::RegisterSpec for EtsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ets_ctrl::R`](R) reader structure"]
impl crate::Readable for EtsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ets_ctrl::W`](W) writer structure"]
impl crate::Writable for EtsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ets_ctrl to value 0"]
impl crate::Resettable for EtsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
