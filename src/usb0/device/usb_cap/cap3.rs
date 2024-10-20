#[doc = "Register `cap3` reader"]
pub type R = crate::R<Cap3Spec>;
#[doc = "Register `cap3` writer"]
pub type W = crate::W<Cap3Spec>;
#[doc = "Field `ep_is_implemented(0-31)` reader - Endpoint is implemented."]
pub type EpIsImplementedR = crate::BitReader;
#[doc = "Field `ep_is_implemented(0-31)` writer - Endpoint is implemented."]
pub type EpIsImplementedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Endpoint is implemented."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_is_implemented0` field"]
    #[inline(always)]
    pub fn ep_is_implemented(&self, n: u8) -> EpIsImplementedR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        EpIsImplementedR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented_iter(&self) -> impl Iterator<Item = EpIsImplementedR> + '_ {
        (0..32).map(move |n| EpIsImplementedR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented0(&self) -> EpIsImplementedR {
        EpIsImplementedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented1(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented2(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented3(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented4(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented5(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented6(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented7(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented8(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented9(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented10(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented11(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented12(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented13(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented14(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented15(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented16(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented17(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented18(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented19(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented20(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented21(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented22(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented23(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented24(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented25(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented26(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented27(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented28(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented29(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented30(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint is implemented."]
    #[inline(always)]
    pub fn ep_is_implemented31(&self) -> EpIsImplementedR {
        EpIsImplementedR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Endpoint is implemented."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_is_implemented0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented(&mut self, n: u8) -> EpIsImplementedW<Cap3Spec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        EpIsImplementedW::new(self, n)
    }
    #[doc = "Bit 0 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented0(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented1(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented2(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented3(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented4(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented5(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented6(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 6)
    }
    #[doc = "Bit 7 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented7(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented8(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 8)
    }
    #[doc = "Bit 9 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented9(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 9)
    }
    #[doc = "Bit 10 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented10(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 10)
    }
    #[doc = "Bit 11 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented11(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 11)
    }
    #[doc = "Bit 12 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented12(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 12)
    }
    #[doc = "Bit 13 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented13(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 13)
    }
    #[doc = "Bit 14 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented14(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 14)
    }
    #[doc = "Bit 15 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented15(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 15)
    }
    #[doc = "Bit 16 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented16(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 16)
    }
    #[doc = "Bit 17 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented17(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 17)
    }
    #[doc = "Bit 18 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented18(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 18)
    }
    #[doc = "Bit 19 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented19(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 19)
    }
    #[doc = "Bit 20 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented20(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 20)
    }
    #[doc = "Bit 21 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented21(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 21)
    }
    #[doc = "Bit 22 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented22(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 22)
    }
    #[doc = "Bit 23 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented23(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 23)
    }
    #[doc = "Bit 24 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented24(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 24)
    }
    #[doc = "Bit 25 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented25(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 25)
    }
    #[doc = "Bit 26 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented26(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 26)
    }
    #[doc = "Bit 27 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented27(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 27)
    }
    #[doc = "Bit 28 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented28(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 28)
    }
    #[doc = "Bit 29 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented29(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented30(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ep_is_implemented31(&mut self) -> EpIsImplementedW<Cap3Spec> {
        EpIsImplementedW::new(self, 31)
    }
}
#[doc = "USB3 Global capability 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap3Spec;
impl crate::RegisterSpec for Cap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap3::R`](R) reader structure"]
impl crate::Readable for Cap3Spec {}
#[doc = "`write(|w| ..)` method takes [`cap3::W`](W) writer structure"]
impl crate::Writable for Cap3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cap3 to value 0"]
impl crate::Resettable for Cap3Spec {
    const RESET_VALUE: u32 = 0;
}
