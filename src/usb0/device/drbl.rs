#[doc = "Register `drbl` reader"]
pub type R = crate::R<DrblSpec>;
#[doc = "Register `drbl` writer"]
pub type W = crate::W<DrblSpec>;
#[doc = "Field `out_ep(0-15)` reader - Doorbell OUT."]
pub type OutEpR = crate::BitReader;
#[doc = "Field `out_ep(0-15)` writer - Doorbell OUT."]
pub type OutEpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `in_ep(0-15)` reader - Doorbell IN."]
pub type InEpR = crate::BitReader;
#[doc = "Field `in_ep(0-15)` writer - Doorbell IN."]
pub type InEpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Doorbell OUT."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `out_ep0` field"]
    #[inline(always)]
    pub fn out_ep(&self, n: u8) -> OutEpR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OutEpR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep_iter(&self) -> impl Iterator<Item = OutEpR> + '_ {
        (0..16).map(move |n| OutEpR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep0(&self) -> OutEpR {
        OutEpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep1(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep2(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep3(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep4(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep5(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep6(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep7(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep8(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep9(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep10(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep11(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep12(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep13(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep14(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Doorbell OUT."]
    #[inline(always)]
    pub fn out_ep15(&self) -> OutEpR {
        OutEpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Doorbell IN."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `in_ep0` field"]
    #[inline(always)]
    pub fn in_ep(&self, n: u8) -> InEpR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        InEpR::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Doorbell IN."]
    #[inline(always)]
    pub fn in_ep_iter(&self) -> impl Iterator<Item = InEpR> + '_ {
        (0..16).map(move |n| InEpR::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep0(&self) -> InEpR {
        InEpR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep1(&self) -> InEpR {
        InEpR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep2(&self) -> InEpR {
        InEpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep3(&self) -> InEpR {
        InEpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep4(&self) -> InEpR {
        InEpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep5(&self) -> InEpR {
        InEpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep6(&self) -> InEpR {
        InEpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep7(&self) -> InEpR {
        InEpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep8(&self) -> InEpR {
        InEpR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep9(&self) -> InEpR {
        InEpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep10(&self) -> InEpR {
        InEpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep11(&self) -> InEpR {
        InEpR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep12(&self) -> InEpR {
        InEpR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep13(&self) -> InEpR {
        InEpR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep14(&self) -> InEpR {
        InEpR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Doorbell IN."]
    #[inline(always)]
    pub fn in_ep15(&self) -> InEpR {
        InEpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Doorbell OUT."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `out_ep0` field"]
    #[inline(always)]
    #[must_use]
    pub fn out_ep(&mut self, n: u8) -> OutEpW<DrblSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OutEpW::new(self, n)
    }
    #[doc = "Bit 0 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep0(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 0)
    }
    #[doc = "Bit 1 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep1(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 1)
    }
    #[doc = "Bit 2 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep2(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 2)
    }
    #[doc = "Bit 3 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep3(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 3)
    }
    #[doc = "Bit 4 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep4(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 4)
    }
    #[doc = "Bit 5 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep5(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 5)
    }
    #[doc = "Bit 6 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep6(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 6)
    }
    #[doc = "Bit 7 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep7(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 7)
    }
    #[doc = "Bit 8 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep8(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 8)
    }
    #[doc = "Bit 9 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep9(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 9)
    }
    #[doc = "Bit 10 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep10(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 10)
    }
    #[doc = "Bit 11 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep11(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 11)
    }
    #[doc = "Bit 12 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep12(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 12)
    }
    #[doc = "Bit 13 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep13(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 13)
    }
    #[doc = "Bit 14 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep14(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 14)
    }
    #[doc = "Bit 15 - Doorbell OUT."]
    #[inline(always)]
    #[must_use]
    pub fn out_ep15(&mut self) -> OutEpW<DrblSpec> {
        OutEpW::new(self, 15)
    }
    #[doc = "Doorbell IN."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `in_ep0` field"]
    #[inline(always)]
    #[must_use]
    pub fn in_ep(&mut self, n: u8) -> InEpW<DrblSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        InEpW::new(self, n + 16)
    }
    #[doc = "Bit 16 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep0(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 16)
    }
    #[doc = "Bit 17 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep1(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 17)
    }
    #[doc = "Bit 18 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep2(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 18)
    }
    #[doc = "Bit 19 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep3(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 19)
    }
    #[doc = "Bit 20 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep4(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 20)
    }
    #[doc = "Bit 21 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep5(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 21)
    }
    #[doc = "Bit 22 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep6(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 22)
    }
    #[doc = "Bit 23 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep7(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 23)
    }
    #[doc = "Bit 24 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep8(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 24)
    }
    #[doc = "Bit 25 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep9(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 25)
    }
    #[doc = "Bit 26 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep10(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 26)
    }
    #[doc = "Bit 27 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep11(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 27)
    }
    #[doc = "Bit 28 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep12(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 28)
    }
    #[doc = "Bit 29 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep13(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 29)
    }
    #[doc = "Bit 30 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep14(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 30)
    }
    #[doc = "Bit 31 - Doorbell IN."]
    #[inline(always)]
    #[must_use]
    pub fn in_ep15(&mut self) -> InEpW<DrblSpec> {
        InEpW::new(self, 31)
    }
}
#[doc = "USB3 doorbell.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drbl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drbl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrblSpec;
impl crate::RegisterSpec for DrblSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drbl::R`](R) reader structure"]
impl crate::Readable for DrblSpec {}
#[doc = "`write(|w| ..)` method takes [`drbl::W`](W) writer structure"]
impl crate::Writable for DrblSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets drbl to value 0"]
impl crate::Resettable for DrblSpec {
    const RESET_VALUE: u32 = 0;
}
