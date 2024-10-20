#[doc = "Register `cap5` reader"]
pub type R = crate::R<Cap5Spec>;
#[doc = "Register `cap5` writer"]
pub type W = crate::W<Cap5Spec>;
#[doc = "Field `ep_support_stream(0-31)` reader - Endpoint supports streaming mode."]
pub type EpSupportStreamR = crate::BitReader;
#[doc = "Field `ep_support_stream(0-31)` writer - Endpoint supports streaming mode."]
pub type EpSupportStreamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Endpoint supports streaming mode."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_support_stream0` field"]
    #[inline(always)]
    pub fn ep_support_stream(&self, n: u8) -> EpSupportStreamR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        EpSupportStreamR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream_iter(&self) -> impl Iterator<Item = EpSupportStreamR> + '_ {
        (0..32).map(move |n| EpSupportStreamR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream0(&self) -> EpSupportStreamR {
        EpSupportStreamR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream1(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream2(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream3(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream4(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream5(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream6(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream7(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream8(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream9(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream10(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream11(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream12(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream13(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream14(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream15(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream16(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream17(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream18(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream19(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream20(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream21(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream22(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream23(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream24(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream25(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream26(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream27(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream28(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream29(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream30(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint supports streaming mode."]
    #[inline(always)]
    pub fn ep_support_stream31(&self) -> EpSupportStreamR {
        EpSupportStreamR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Endpoint supports streaming mode."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_support_stream0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream(&mut self, n: u8) -> EpSupportStreamW<Cap5Spec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        EpSupportStreamW::new(self, n)
    }
    #[doc = "Bit 0 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream0(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream1(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream2(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream3(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream4(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream5(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream6(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 6)
    }
    #[doc = "Bit 7 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream7(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream8(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 8)
    }
    #[doc = "Bit 9 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream9(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 9)
    }
    #[doc = "Bit 10 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream10(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 10)
    }
    #[doc = "Bit 11 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream11(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 11)
    }
    #[doc = "Bit 12 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream12(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 12)
    }
    #[doc = "Bit 13 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream13(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 13)
    }
    #[doc = "Bit 14 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream14(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 14)
    }
    #[doc = "Bit 15 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream15(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 15)
    }
    #[doc = "Bit 16 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream16(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 16)
    }
    #[doc = "Bit 17 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream17(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 17)
    }
    #[doc = "Bit 18 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream18(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 18)
    }
    #[doc = "Bit 19 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream19(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 19)
    }
    #[doc = "Bit 20 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream20(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 20)
    }
    #[doc = "Bit 21 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream21(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 21)
    }
    #[doc = "Bit 22 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream22(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 22)
    }
    #[doc = "Bit 23 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream23(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 23)
    }
    #[doc = "Bit 24 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream24(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 24)
    }
    #[doc = "Bit 25 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream25(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 25)
    }
    #[doc = "Bit 26 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream26(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 26)
    }
    #[doc = "Bit 27 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream27(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 27)
    }
    #[doc = "Bit 28 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream28(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 28)
    }
    #[doc = "Bit 29 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream29(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream30(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint supports streaming mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_stream31(&mut self) -> EpSupportStreamW<Cap5Spec> {
        EpSupportStreamW::new(self, 31)
    }
}
#[doc = "USB3 Global capability 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap5Spec;
impl crate::RegisterSpec for Cap5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap5::R`](R) reader structure"]
impl crate::Readable for Cap5Spec {}
#[doc = "`write(|w| ..)` method takes [`cap5::W`](W) writer structure"]
impl crate::Writable for Cap5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cap5 to value 0"]
impl crate::Resettable for Cap5Spec {
    const RESET_VALUE: u32 = 0;
}
