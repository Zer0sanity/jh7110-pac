#[doc = "Register `cdetect` reader"]
pub type R = crate::R<CdetectSpec>;
#[doc = "Register `cdetect` writer"]
pub type W = crate::W<CdetectSpec>;
#[doc = "MMC card present in slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    #[doc = "0: MMC card present in slot"]
    Present = 0,
    #[doc = "1: MMC card not present in slot"]
    NotPresent = 1,
}
impl From<Slot> for bool {
    #[inline(always)]
    fn from(variant: Slot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `slot(0-31)` reader - MMC card present in slot"]
pub type SlotR = crate::BitReader<Slot>;
impl SlotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slot {
        match self.bits {
            false => Slot::Present,
            true => Slot::NotPresent,
        }
    }
    #[doc = "MMC card present in slot"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == Slot::Present
    }
    #[doc = "MMC card not present in slot"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == Slot::NotPresent
    }
}
#[doc = "Field `slot(0-31)` writer - MMC card present in slot"]
pub type SlotW<'a, REG> = crate::BitWriter<'a, REG, Slot>;
impl<'a, REG> SlotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MMC card present in slot"]
    #[inline(always)]
    pub fn present(self) -> &'a mut crate::W<REG> {
        self.variant(Slot::Present)
    }
    #[doc = "MMC card not present in slot"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut crate::W<REG> {
        self.variant(Slot::NotPresent)
    }
}
impl R {
    #[doc = "MMC card present in slot"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slot0` field"]
    #[inline(always)]
    pub fn slot(&self, n: u8) -> SlotR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SlotR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC card present in slot"]
    #[inline(always)]
    pub fn slot_iter(&self) -> impl Iterator<Item = SlotR> + '_ {
        (0..32).map(move |n| SlotR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot0(&self) -> SlotR {
        SlotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot1(&self) -> SlotR {
        SlotR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot2(&self) -> SlotR {
        SlotR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot3(&self) -> SlotR {
        SlotR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot4(&self) -> SlotR {
        SlotR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot5(&self) -> SlotR {
        SlotR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot6(&self) -> SlotR {
        SlotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot7(&self) -> SlotR {
        SlotR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot8(&self) -> SlotR {
        SlotR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot9(&self) -> SlotR {
        SlotR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot10(&self) -> SlotR {
        SlotR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot11(&self) -> SlotR {
        SlotR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot12(&self) -> SlotR {
        SlotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot13(&self) -> SlotR {
        SlotR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot14(&self) -> SlotR {
        SlotR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot15(&self) -> SlotR {
        SlotR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot16(&self) -> SlotR {
        SlotR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot17(&self) -> SlotR {
        SlotR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot18(&self) -> SlotR {
        SlotR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot19(&self) -> SlotR {
        SlotR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot20(&self) -> SlotR {
        SlotR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot21(&self) -> SlotR {
        SlotR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot22(&self) -> SlotR {
        SlotR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot23(&self) -> SlotR {
        SlotR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot24(&self) -> SlotR {
        SlotR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot25(&self) -> SlotR {
        SlotR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot26(&self) -> SlotR {
        SlotR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot27(&self) -> SlotR {
        SlotR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot28(&self) -> SlotR {
        SlotR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot29(&self) -> SlotR {
        SlotR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot30(&self) -> SlotR {
        SlotR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC card present in slot"]
    #[inline(always)]
    pub fn slot31(&self) -> SlotR {
        SlotR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "MMC card present in slot"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `slot0` field"]
    #[inline(always)]
    #[must_use]
    pub fn slot(&mut self, n: u8) -> SlotW<CdetectSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        SlotW::new(self, n)
    }
    #[doc = "Bit 0 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot0(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot1(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot2(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot3(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot4(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot5(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot6(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot7(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot8(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot9(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot10(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot11(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot12(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot13(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot14(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot15(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 15)
    }
    #[doc = "Bit 16 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot16(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot17(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot18(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot19(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot20(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot21(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot22(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot23(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot24(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot25(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 25)
    }
    #[doc = "Bit 26 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot26(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 26)
    }
    #[doc = "Bit 27 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot27(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 27)
    }
    #[doc = "Bit 28 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot28(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 28)
    }
    #[doc = "Bit 29 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot29(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 29)
    }
    #[doc = "Bit 30 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot30(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 30)
    }
    #[doc = "Bit 31 - MMC card present in slot"]
    #[inline(always)]
    #[must_use]
    pub fn slot31(&mut self) -> SlotW<CdetectSpec> {
        SlotW::new(self, 31)
    }
}
#[doc = "MMC card detect\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdetect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdetect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdetectSpec;
impl crate::RegisterSpec for CdetectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdetect::R`](R) reader structure"]
impl crate::Readable for CdetectSpec {}
#[doc = "`write(|w| ..)` method takes [`cdetect::W`](W) writer structure"]
impl crate::Writable for CdetectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cdetect to value 0"]
impl crate::Resettable for CdetectSpec {
    const RESET_VALUE: u32 = 0;
}
