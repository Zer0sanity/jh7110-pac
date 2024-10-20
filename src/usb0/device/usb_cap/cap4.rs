#[doc = "Register `cap4` reader"]
pub type R = crate::R<Cap4Spec>;
#[doc = "Register `cap4` writer"]
pub type W = crate::W<Cap4Spec>;
#[doc = "Field `ep_support_iso(0-31)` reader - Endpoint supports ISO mode."]
pub type EpSupportIsoR = crate::BitReader;
#[doc = "Field `ep_support_iso(0-31)` writer - Endpoint supports ISO mode."]
pub type EpSupportIsoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Endpoint supports ISO mode."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_support_iso0` field"]
    #[inline(always)]
    pub fn ep_support_iso(&self, n: u8) -> EpSupportIsoR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        EpSupportIsoR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso_iter(&self) -> impl Iterator<Item = EpSupportIsoR> + '_ {
        (0..32).map(move |n| EpSupportIsoR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso0(&self) -> EpSupportIsoR {
        EpSupportIsoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso1(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso2(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso3(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso4(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso5(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso6(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso7(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso8(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso9(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso10(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso11(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso12(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso13(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso14(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso15(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso16(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso17(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso18(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso19(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso20(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso21(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso22(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso23(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso24(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso25(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso26(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso27(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso28(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso29(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso30(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint supports ISO mode."]
    #[inline(always)]
    pub fn ep_support_iso31(&self) -> EpSupportIsoR {
        EpSupportIsoR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Endpoint supports ISO mode."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_support_iso0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso(&mut self, n: u8) -> EpSupportIsoW<Cap4Spec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        EpSupportIsoW::new(self, n)
    }
    #[doc = "Bit 0 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso0(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso1(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso2(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso3(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso4(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso5(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso6(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 6)
    }
    #[doc = "Bit 7 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso7(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso8(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 8)
    }
    #[doc = "Bit 9 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso9(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 9)
    }
    #[doc = "Bit 10 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso10(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 10)
    }
    #[doc = "Bit 11 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso11(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 11)
    }
    #[doc = "Bit 12 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso12(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 12)
    }
    #[doc = "Bit 13 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso13(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 13)
    }
    #[doc = "Bit 14 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso14(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 14)
    }
    #[doc = "Bit 15 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso15(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 15)
    }
    #[doc = "Bit 16 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso16(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 16)
    }
    #[doc = "Bit 17 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso17(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 17)
    }
    #[doc = "Bit 18 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso18(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 18)
    }
    #[doc = "Bit 19 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso19(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 19)
    }
    #[doc = "Bit 20 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso20(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 20)
    }
    #[doc = "Bit 21 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso21(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 21)
    }
    #[doc = "Bit 22 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso22(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 22)
    }
    #[doc = "Bit 23 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso23(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 23)
    }
    #[doc = "Bit 24 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso24(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 24)
    }
    #[doc = "Bit 25 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso25(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 25)
    }
    #[doc = "Bit 26 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso26(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 26)
    }
    #[doc = "Bit 27 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso27(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 27)
    }
    #[doc = "Bit 28 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso28(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 28)
    }
    #[doc = "Bit 29 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso29(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso30(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint supports ISO mode."]
    #[inline(always)]
    #[must_use]
    pub fn ep_support_iso31(&mut self) -> EpSupportIsoW<Cap4Spec> {
        EpSupportIsoW::new(self, 31)
    }
}
#[doc = "USB3 Global capability 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap4Spec;
impl crate::RegisterSpec for Cap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap4::R`](R) reader structure"]
impl crate::Readable for Cap4Spec {}
#[doc = "`write(|w| ..)` method takes [`cap4::W`](W) writer structure"]
impl crate::Writable for Cap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cap4 to value 0"]
impl crate::Resettable for Cap4Spec {
    const RESET_VALUE: u32 = 0;
}
