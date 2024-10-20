#[doc = "Register `chen2` reader"]
pub type R = crate::R<Chen2Spec>;
#[doc = "Register `chen2` writer"]
pub type W = crate::W<Chen2Spec>;
#[doc = "Field `en0_ch(1-16)` reader - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
pub type En0ChR = crate::BitReader;
#[doc = "Field `en0_ch(1-16)` writer - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
pub type En0ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_we0_ch(1-16)` writer - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
pub type EnWe0ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en1_ch(17-32)` reader - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
pub type En1ChR = crate::BitReader;
#[doc = "Field `en1_ch(17-32)` writer - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
pub type En1ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_we1_ch(17-32)` writer - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
pub type EnWe1ChW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en0_ch1` field"]
    #[inline(always)]
    pub fn en0_ch(&self, n: u8) -> En0ChR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        En0ChR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch_iter(&self) -> impl Iterator<Item = En0ChR> + '_ {
        (0..16).map(move |n| En0ChR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch1(&self) -> En0ChR {
        En0ChR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch2(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch3(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch4(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch5(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch6(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch7(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch8(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch9(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch10(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch11(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch12(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch13(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch14(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch15(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en0_ch16(&self) -> En0ChR {
        En0ChR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en1_ch17` field"]
    #[inline(always)]
    pub fn en1_ch(&self, n: u8) -> En1ChR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        En1ChR::new(((self.bits >> (n + 32)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch_iter(&self) -> impl Iterator<Item = En1ChR> + '_ {
        (0..16).map(move |n| En1ChR::new(((self.bits >> (n + 32)) & 1) != 0))
    }
    #[doc = "Bit 32 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch17(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 32) & 1) != 0)
    }
    #[doc = "Bit 33 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch18(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 33) & 1) != 0)
    }
    #[doc = "Bit 34 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch19(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 34) & 1) != 0)
    }
    #[doc = "Bit 35 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch20(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 35) & 1) != 0)
    }
    #[doc = "Bit 36 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch21(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 36) & 1) != 0)
    }
    #[doc = "Bit 37 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch22(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Bit 38 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch23(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bit 39 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch24(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 39) & 1) != 0)
    }
    #[doc = "Bit 40 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch25(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 40) & 1) != 0)
    }
    #[doc = "Bit 41 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch26(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 41) & 1) != 0)
    }
    #[doc = "Bit 42 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch27(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 42) & 1) != 0)
    }
    #[doc = "Bit 43 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch28(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 43) & 1) != 0)
    }
    #[doc = "Bit 44 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch29(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 44) & 1) != 0)
    }
    #[doc = "Bit 45 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch30(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 45) & 1) != 0)
    }
    #[doc = "Bit 46 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch31(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 46) & 1) != 0)
    }
    #[doc = "Bit 47 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en1_ch32(&self) -> En1ChR {
        En1ChR::new(((self.bits >> 47) & 1) != 0)
    }
}
impl W {
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en0_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch(&mut self, n: u8) -> En0ChW<Chen2Spec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        En0ChW::new(self, n)
    }
    #[doc = "Bit 0 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch1(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch2(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch3(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch4(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch5(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch6(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch7(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch8(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 7)
    }
    #[doc = "Bit 8 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch9(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 8)
    }
    #[doc = "Bit 9 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch10(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 9)
    }
    #[doc = "Bit 10 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch11(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 10)
    }
    #[doc = "Bit 11 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch12(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 11)
    }
    #[doc = "Bit 12 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch13(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 12)
    }
    #[doc = "Bit 13 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch14(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 13)
    }
    #[doc = "Bit 14 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch15(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 14)
    }
    #[doc = "Bit 15 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en0_ch16(&mut self) -> En0ChW<Chen2Spec> {
        En0ChW::new(self, 15)
    }
    #[doc = "DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en_we0_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch(&mut self, n: u8) -> EnWe0ChW<Chen2Spec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        EnWe0ChW::new(self, n + 16)
    }
    #[doc = "Bit 16 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch1(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 16)
    }
    #[doc = "Bit 17 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch2(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 17)
    }
    #[doc = "Bit 18 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch3(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 18)
    }
    #[doc = "Bit 19 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch4(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 19)
    }
    #[doc = "Bit 20 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch5(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 20)
    }
    #[doc = "Bit 21 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch6(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 21)
    }
    #[doc = "Bit 22 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch7(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 22)
    }
    #[doc = "Bit 23 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch8(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch9(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 24)
    }
    #[doc = "Bit 25 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch10(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 25)
    }
    #[doc = "Bit 26 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch11(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 26)
    }
    #[doc = "Bit 27 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch12(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 27)
    }
    #[doc = "Bit 28 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch13(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 28)
    }
    #[doc = "Bit 29 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch14(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 29)
    }
    #[doc = "Bit 30 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch15(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 30)
    }
    #[doc = "Bit 31 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we0_ch16(&mut self) -> EnWe0ChW<Chen2Spec> {
        EnWe0ChW::new(self, 31)
    }
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en1_ch17` field"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch(&mut self, n: u8) -> En1ChW<Chen2Spec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        En1ChW::new(self, n + 32)
    }
    #[doc = "Bit 32 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch17(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 32)
    }
    #[doc = "Bit 33 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch18(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 33)
    }
    #[doc = "Bit 34 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch19(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 34)
    }
    #[doc = "Bit 35 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch20(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 35)
    }
    #[doc = "Bit 36 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch21(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 36)
    }
    #[doc = "Bit 37 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch22(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 37)
    }
    #[doc = "Bit 38 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch23(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 38)
    }
    #[doc = "Bit 39 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch24(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 39)
    }
    #[doc = "Bit 40 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch25(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 40)
    }
    #[doc = "Bit 41 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch26(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 41)
    }
    #[doc = "Bit 42 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch27(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 42)
    }
    #[doc = "Bit 43 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch28(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 43)
    }
    #[doc = "Bit 44 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch29(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 44)
    }
    #[doc = "Bit 45 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch30(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 45)
    }
    #[doc = "Bit 46 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch31(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 46)
    }
    #[doc = "Bit 47 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en1_ch32(&mut self) -> En1ChW<Chen2Spec> {
        En1ChW::new(self, 47)
    }
    #[doc = "DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en_we1_ch17` field"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch(&mut self, n: u8) -> EnWe1ChW<Chen2Spec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        EnWe1ChW::new(self, n + 48)
    }
    #[doc = "Bit 48 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch17(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 48)
    }
    #[doc = "Bit 49 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch18(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 49)
    }
    #[doc = "Bit 50 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch19(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 50)
    }
    #[doc = "Bit 51 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch20(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 51)
    }
    #[doc = "Bit 52 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch21(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 52)
    }
    #[doc = "Bit 53 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch22(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 53)
    }
    #[doc = "Bit 54 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch23(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 54)
    }
    #[doc = "Bit 55 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch24(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 55)
    }
    #[doc = "Bit 56 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch25(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 56)
    }
    #[doc = "Bit 57 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch26(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 57)
    }
    #[doc = "Bit 58 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch27(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 58)
    }
    #[doc = "Bit 59 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch28(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 59)
    }
    #[doc = "Bit 60 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch29(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 60)
    }
    #[doc = "Bit 61 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch30(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 61)
    }
    #[doc = "Bit 62 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch31(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 62)
    }
    #[doc = "Bit 63 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we1_ch32(&mut self) -> EnWe1ChW<Chen2Spec> {
        EnWe1ChW::new(self, 63)
    }
}
#[doc = "DMAC Channel Enable register contains the DMAC channel enable settings. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chen2Spec;
impl crate::RegisterSpec for Chen2Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`chen2::R`](R) reader structure"]
impl crate::Readable for Chen2Spec {}
#[doc = "`write(|w| ..)` method takes [`chen2::W`](W) writer structure"]
impl crate::Writable for Chen2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets chen2 to value 0"]
impl crate::Resettable for Chen2Spec {
    const RESET_VALUE: u64 = 0;
}
