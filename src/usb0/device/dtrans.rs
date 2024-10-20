#[doc = "Register `dtrans` reader"]
pub type R = crate::R<DtransSpec>;
#[doc = "Register `dtrans` writer"]
pub type W = crate::W<DtransSpec>;
#[doc = "Field `dtrans_out(0-15)` reader - DMA transfer mode - OUT endpoint."]
pub type DtransOutR = crate::BitReader;
#[doc = "Field `dtrans_out(0-15)` writer - DMA transfer mode - OUT endpoint."]
pub type DtransOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dtrans_in(0-15)` reader - DMA transfer mode - IN endpoint."]
pub type DtransInR = crate::BitReader;
#[doc = "Field `dtrans_in(0-15)` writer - DMA transfer mode - IN endpoint."]
pub type DtransInW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "DMA transfer mode - OUT endpoint."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dtrans_out0` field"]
    #[inline(always)]
    pub fn dtrans_out(&self, n: u8) -> DtransOutR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DtransOutR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out_iter(&self) -> impl Iterator<Item = DtransOutR> + '_ {
        (0..16).map(move |n| DtransOutR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out0(&self) -> DtransOutR {
        DtransOutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out1(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out2(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out3(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out4(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out5(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out6(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out7(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out8(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out9(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out10(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out11(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out12(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out13(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out14(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    pub fn dtrans_out15(&self) -> DtransOutR {
        DtransOutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "DMA transfer mode - IN endpoint."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dtrans_in0` field"]
    #[inline(always)]
    pub fn dtrans_in(&self, n: u8) -> DtransInR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DtransInR::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in_iter(&self) -> impl Iterator<Item = DtransInR> + '_ {
        (0..16).map(move |n| DtransInR::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in0(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in1(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in2(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in3(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in4(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in5(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in6(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in7(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in8(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in9(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in10(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in11(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in12(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in13(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in14(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    pub fn dtrans_in15(&self) -> DtransInR {
        DtransInR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "DMA transfer mode - OUT endpoint."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dtrans_out0` field"]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out(&mut self, n: u8) -> DtransOutW<DtransSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DtransOutW::new(self, n)
    }
    #[doc = "Bit 0 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out0(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out1(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out2(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out3(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out4(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out5(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out6(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out7(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 7)
    }
    #[doc = "Bit 8 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out8(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 8)
    }
    #[doc = "Bit 9 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out9(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 9)
    }
    #[doc = "Bit 10 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out10(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out11(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 11)
    }
    #[doc = "Bit 12 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out12(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 12)
    }
    #[doc = "Bit 13 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out13(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 13)
    }
    #[doc = "Bit 14 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out14(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 14)
    }
    #[doc = "Bit 15 - DMA transfer mode - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_out15(&mut self) -> DtransOutW<DtransSpec> {
        DtransOutW::new(self, 15)
    }
    #[doc = "DMA transfer mode - IN endpoint."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `dtrans_in0` field"]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in(&mut self, n: u8) -> DtransInW<DtransSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        DtransInW::new(self, n + 16)
    }
    #[doc = "Bit 16 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in0(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 16)
    }
    #[doc = "Bit 17 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in1(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 17)
    }
    #[doc = "Bit 18 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in2(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 18)
    }
    #[doc = "Bit 19 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in3(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 19)
    }
    #[doc = "Bit 20 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in4(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 20)
    }
    #[doc = "Bit 21 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in5(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in6(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 22)
    }
    #[doc = "Bit 23 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in7(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 23)
    }
    #[doc = "Bit 24 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in8(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in9(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 25)
    }
    #[doc = "Bit 26 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in10(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 26)
    }
    #[doc = "Bit 27 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in11(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 27)
    }
    #[doc = "Bit 28 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in12(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 28)
    }
    #[doc = "Bit 29 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in13(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 29)
    }
    #[doc = "Bit 30 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in14(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 30)
    }
    #[doc = "Bit 31 - DMA transfer mode - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans_in15(&mut self) -> DtransInW<DtransSpec> {
        DtransInW::new(self, 31)
    }
}
#[doc = "USB3 DMA transfer mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtrans::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtrans::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtransSpec;
impl crate::RegisterSpec for DtransSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtrans::R`](R) reader structure"]
impl crate::Readable for DtransSpec {}
#[doc = "`write(|w| ..)` method takes [`dtrans::W`](W) writer structure"]
impl crate::Writable for DtransSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dtrans to value 0"]
impl crate::Resettable for DtransSpec {
    const RESET_VALUE: u32 = 0;
}
