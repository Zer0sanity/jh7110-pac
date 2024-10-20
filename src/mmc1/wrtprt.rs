#[doc = "Register `wrtprt` reader"]
pub type R = crate::R<WrtprtSpec>;
#[doc = "Register `wrtprt` writer"]
pub type W = crate::W<WrtprtSpec>;
#[doc = "MMC card slot write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    #[doc = "0: MMC card slot no write protect"]
    NoProtect = 0,
    #[doc = "1: MMC card slot write protect"]
    Protect = 1,
}
impl From<Protect> for bool {
    #[inline(always)]
    fn from(variant: Protect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `protect(0-31)` reader - MMC card slot write protect"]
pub type ProtectR = crate::BitReader<Protect>;
impl ProtectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protect {
        match self.bits {
            false => Protect::NoProtect,
            true => Protect::Protect,
        }
    }
    #[doc = "MMC card slot no write protect"]
    #[inline(always)]
    pub fn is_no_protect(&self) -> bool {
        *self == Protect::NoProtect
    }
    #[doc = "MMC card slot write protect"]
    #[inline(always)]
    pub fn is_protect(&self) -> bool {
        *self == Protect::Protect
    }
}
#[doc = "Field `protect(0-31)` writer - MMC card slot write protect"]
pub type ProtectW<'a, REG> = crate::BitWriter<'a, REG, Protect>;
impl<'a, REG> ProtectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MMC card slot no write protect"]
    #[inline(always)]
    pub fn no_protect(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::NoProtect)
    }
    #[doc = "MMC card slot write protect"]
    #[inline(always)]
    pub fn protect(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::Protect)
    }
}
impl R {
    #[doc = "MMC card slot write protect"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `protect0` field"]
    #[inline(always)]
    pub fn protect(&self, n: u8) -> ProtectR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ProtectR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC card slot write protect"]
    #[inline(always)]
    pub fn protect_iter(&self) -> impl Iterator<Item = ProtectR> + '_ {
        (0..32).map(move |n| ProtectR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect0(&self) -> ProtectR {
        ProtectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect1(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect2(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect3(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect4(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect5(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect6(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect7(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect8(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect9(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect10(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect11(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect12(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect13(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect14(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect15(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect16(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect17(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect18(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect19(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect20(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect21(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect22(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect23(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect24(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect25(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect26(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect27(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect28(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect29(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect30(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC card slot write protect"]
    #[inline(always)]
    pub fn protect31(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "MMC card slot write protect"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `protect0` field"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self, n: u8) -> ProtectW<WrtprtSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        ProtectW::new(self, n)
    }
    #[doc = "Bit 0 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect0(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect1(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect2(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect3(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect4(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect5(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect6(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect7(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect8(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect9(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect10(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect11(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect12(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect13(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect14(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect15(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 15)
    }
    #[doc = "Bit 16 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect16(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect17(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect18(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect19(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect20(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect21(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect22(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect23(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect24(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect25(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 25)
    }
    #[doc = "Bit 26 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect26(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 26)
    }
    #[doc = "Bit 27 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect27(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 27)
    }
    #[doc = "Bit 28 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect28(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 28)
    }
    #[doc = "Bit 29 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect29(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 29)
    }
    #[doc = "Bit 30 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect30(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 30)
    }
    #[doc = "Bit 31 - MMC card slot write protect"]
    #[inline(always)]
    #[must_use]
    pub fn protect31(&mut self) -> ProtectW<WrtprtSpec> {
        ProtectW::new(self, 31)
    }
}
#[doc = "MMC write protect\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrtprt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wrtprt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrtprtSpec;
impl crate::RegisterSpec for WrtprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrtprt::R`](R) reader structure"]
impl crate::Readable for WrtprtSpec {}
#[doc = "`write(|w| ..)` method takes [`wrtprt::W`](W) writer structure"]
impl crate::Writable for WrtprtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wrtprt to value 0"]
impl crate::Resettable for WrtprtSpec {
    const RESET_VALUE: u32 = 0;
}
