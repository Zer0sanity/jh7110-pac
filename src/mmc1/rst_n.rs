#[doc = "Register `rst_n` reader"]
pub type R = crate::R<RstNSpec>;
#[doc = "Register `rst_n` writer"]
pub type W = crate::W<RstNSpec>;
#[doc = "MMC Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstN {
    #[doc = "0: MMC hardware inactive"]
    Hwinactive = 0,
    #[doc = "1: MMC hardware active"]
    Hwactive = 1,
}
impl From<RstN> for bool {
    #[inline(always)]
    fn from(variant: RstN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rst_n(0-31)` reader - MMC Reset"]
pub type RstNR = crate::BitReader<RstN>;
impl RstNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstN {
        match self.bits {
            false => RstN::Hwinactive,
            true => RstN::Hwactive,
        }
    }
    #[doc = "MMC hardware inactive"]
    #[inline(always)]
    pub fn is_hwinactive(&self) -> bool {
        *self == RstN::Hwinactive
    }
    #[doc = "MMC hardware active"]
    #[inline(always)]
    pub fn is_hwactive(&self) -> bool {
        *self == RstN::Hwactive
    }
}
#[doc = "Field `rst_n(0-31)` writer - MMC Reset"]
pub type RstNW<'a, REG> = crate::BitWriter<'a, REG, RstN>;
impl<'a, REG> RstNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MMC hardware inactive"]
    #[inline(always)]
    pub fn hwinactive(self) -> &'a mut crate::W<REG> {
        self.variant(RstN::Hwinactive)
    }
    #[doc = "MMC hardware active"]
    #[inline(always)]
    pub fn hwactive(self) -> &'a mut crate::W<REG> {
        self.variant(RstN::Hwactive)
    }
}
impl R {
    #[doc = "MMC Reset"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rst_n0` field"]
    #[inline(always)]
    pub fn rst_n(&self, n: u8) -> RstNR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        RstNR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC Reset"]
    #[inline(always)]
    pub fn rst_n_iter(&self) -> impl Iterator<Item = RstNR> + '_ {
        (0..32).map(move |n| RstNR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n0(&self) -> RstNR {
        RstNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n1(&self) -> RstNR {
        RstNR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n2(&self) -> RstNR {
        RstNR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n3(&self) -> RstNR {
        RstNR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n4(&self) -> RstNR {
        RstNR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n5(&self) -> RstNR {
        RstNR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n6(&self) -> RstNR {
        RstNR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n7(&self) -> RstNR {
        RstNR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n8(&self) -> RstNR {
        RstNR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n9(&self) -> RstNR {
        RstNR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n10(&self) -> RstNR {
        RstNR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n11(&self) -> RstNR {
        RstNR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n12(&self) -> RstNR {
        RstNR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n13(&self) -> RstNR {
        RstNR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n14(&self) -> RstNR {
        RstNR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n15(&self) -> RstNR {
        RstNR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n16(&self) -> RstNR {
        RstNR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n17(&self) -> RstNR {
        RstNR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n18(&self) -> RstNR {
        RstNR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n19(&self) -> RstNR {
        RstNR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n20(&self) -> RstNR {
        RstNR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n21(&self) -> RstNR {
        RstNR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n22(&self) -> RstNR {
        RstNR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n23(&self) -> RstNR {
        RstNR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n24(&self) -> RstNR {
        RstNR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n25(&self) -> RstNR {
        RstNR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n26(&self) -> RstNR {
        RstNR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n27(&self) -> RstNR {
        RstNR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n28(&self) -> RstNR {
        RstNR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n29(&self) -> RstNR {
        RstNR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n30(&self) -> RstNR {
        RstNR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC Reset"]
    #[inline(always)]
    pub fn rst_n31(&self) -> RstNR {
        RstNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "MMC Reset"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `rst_n0` field"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n(&mut self, n: u8) -> RstNW<RstNSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        RstNW::new(self, n)
    }
    #[doc = "Bit 0 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n0(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n1(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n2(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n3(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n4(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n5(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n6(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n7(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n8(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n9(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n10(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n11(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n12(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n13(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n14(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n15(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 15)
    }
    #[doc = "Bit 16 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n16(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n17(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n18(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n19(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n20(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n21(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n22(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n23(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n24(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n25(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 25)
    }
    #[doc = "Bit 26 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n26(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 26)
    }
    #[doc = "Bit 27 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n27(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 27)
    }
    #[doc = "Bit 28 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n28(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 28)
    }
    #[doc = "Bit 29 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n29(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 29)
    }
    #[doc = "Bit 30 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n30(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 30)
    }
    #[doc = "Bit 31 - MMC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_n31(&mut self) -> RstNW<RstNSpec> {
        RstNW::new(self, 31)
    }
}
#[doc = "MMC Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_n::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst_n::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstNSpec;
impl crate::RegisterSpec for RstNSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_n::R`](R) reader structure"]
impl crate::Readable for RstNSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_n::W`](W) writer structure"]
impl crate::Writable for RstNSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rst_n to value 0"]
impl crate::Resettable for RstNSpec {
    const RESET_VALUE: u32 = 0;
}
