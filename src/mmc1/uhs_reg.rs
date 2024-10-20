#[doc = "Register `uhs_reg` reader"]
pub type R = crate::R<UhsRegSpec>;
#[doc = "Register `uhs_reg` writer"]
pub type W = crate::W<UhsRegSpec>;
#[doc = "MMC slot signal voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Voltage {
    #[doc = "0: MMC slot 3.3v signal voltage"]
    V33 = 0,
    #[doc = "1: MMC slot 1.8v signal voltage"]
    V18 = 1,
}
impl From<Voltage> for bool {
    #[inline(always)]
    fn from(variant: Voltage) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `voltage(0-31)` reader - MMC slot signal voltage"]
pub type VoltageR = crate::BitReader<Voltage>;
impl VoltageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Voltage {
        match self.bits {
            false => Voltage::V33,
            true => Voltage::V18,
        }
    }
    #[doc = "MMC slot 3.3v signal voltage"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == Voltage::V33
    }
    #[doc = "MMC slot 1.8v signal voltage"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == Voltage::V18
    }
}
#[doc = "Field `voltage(0-31)` writer - MMC slot signal voltage"]
pub type VoltageW<'a, REG> = crate::BitWriter<'a, REG, Voltage>;
impl<'a, REG> VoltageW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MMC slot 3.3v signal voltage"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut crate::W<REG> {
        self.variant(Voltage::V33)
    }
    #[doc = "MMC slot 1.8v signal voltage"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut crate::W<REG> {
        self.variant(Voltage::V18)
    }
}
impl R {
    #[doc = "MMC slot signal voltage"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `voltage0` field"]
    #[inline(always)]
    pub fn voltage(&self, n: u8) -> VoltageR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        VoltageR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage_iter(&self) -> impl Iterator<Item = VoltageR> + '_ {
        (0..32).map(move |n| VoltageR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage0(&self) -> VoltageR {
        VoltageR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage1(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage2(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage3(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage4(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage5(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage6(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage7(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage8(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage9(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage10(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage11(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage12(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage13(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage14(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage15(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage16(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage17(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage18(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage19(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage20(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage21(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage22(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage23(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage24(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage25(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage26(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage27(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage28(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage29(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage30(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC slot signal voltage"]
    #[inline(always)]
    pub fn voltage31(&self) -> VoltageR {
        VoltageR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "MMC slot signal voltage"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `voltage0` field"]
    #[inline(always)]
    #[must_use]
    pub fn voltage(&mut self, n: u8) -> VoltageW<UhsRegSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        VoltageW::new(self, n)
    }
    #[doc = "Bit 0 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage0(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage1(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage2(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage3(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage4(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage5(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage6(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage7(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage8(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage9(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage10(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage11(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage12(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage13(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage14(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage15(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 15)
    }
    #[doc = "Bit 16 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage16(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage17(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage18(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage19(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage20(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage21(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage22(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage23(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage24(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage25(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 25)
    }
    #[doc = "Bit 26 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage26(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 26)
    }
    #[doc = "Bit 27 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage27(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 27)
    }
    #[doc = "Bit 28 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage28(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 28)
    }
    #[doc = "Bit 29 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage29(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 29)
    }
    #[doc = "Bit 30 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage30(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 30)
    }
    #[doc = "Bit 31 - MMC slot signal voltage"]
    #[inline(always)]
    #[must_use]
    pub fn voltage31(&mut self) -> VoltageW<UhsRegSpec> {
        VoltageW::new(self, 31)
    }
}
#[doc = "MMC UHS-1 regulator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhs_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhs_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsRegSpec;
impl crate::RegisterSpec for UhsRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_reg::R`](R) reader structure"]
impl crate::Readable for UhsRegSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_reg::W`](W) writer structure"]
impl crate::Writable for UhsRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets uhs_reg to value 0"]
impl crate::Resettable for UhsRegSpec {
    const RESET_VALUE: u32 = 0;
}
