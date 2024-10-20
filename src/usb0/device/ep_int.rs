#[doc = "Register `ep_int[%s]` reader"]
pub type R = crate::R<EpIntSpec>;
#[doc = "Register `ep_int[%s]` writer"]
pub type W = crate::W<EpIntSpec>;
#[doc = "Field `ep_out(0-15)` reader - OUT endpoint."]
pub type EpOutR = crate::BitReader;
#[doc = "Field `ep_out(0-15)` writer - OUT endpoint."]
pub type EpOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ep_in(0-15)` reader - IN endpoint."]
pub type EpInR = crate::BitReader;
#[doc = "Field `ep_in(0-15)` writer - IN endpoint."]
pub type EpInW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "OUT endpoint."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_out0` field"]
    #[inline(always)]
    pub fn ep_out(&self, n: u8) -> EpOutR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        EpOutR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "OUT endpoint."]
    #[inline(always)]
    pub fn ep_out_iter(&self) -> impl Iterator<Item = EpOutR> + '_ {
        (0..16).map(move |n| EpOutR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out0(&self) -> EpOutR {
        EpOutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out1(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out2(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out3(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out4(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out5(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out6(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out7(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out8(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out9(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out10(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out11(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out12(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out13(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out14(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OUT endpoint."]
    #[inline(always)]
    pub fn ep_out15(&self) -> EpOutR {
        EpOutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "IN endpoint."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_in0` field"]
    #[inline(always)]
    pub fn ep_in(&self, n: u8) -> EpInR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        EpInR::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "IN endpoint."]
    #[inline(always)]
    pub fn ep_in_iter(&self) -> impl Iterator<Item = EpInR> + '_ {
        (0..16).map(move |n| EpInR::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in0(&self) -> EpInR {
        EpInR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in1(&self) -> EpInR {
        EpInR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in2(&self) -> EpInR {
        EpInR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in3(&self) -> EpInR {
        EpInR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in4(&self) -> EpInR {
        EpInR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in5(&self) -> EpInR {
        EpInR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in6(&self) -> EpInR {
        EpInR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in7(&self) -> EpInR {
        EpInR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in8(&self) -> EpInR {
        EpInR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in9(&self) -> EpInR {
        EpInR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in10(&self) -> EpInR {
        EpInR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in11(&self) -> EpInR {
        EpInR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in12(&self) -> EpInR {
        EpInR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in13(&self) -> EpInR {
        EpInR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in14(&self) -> EpInR {
        EpInR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IN endpoint."]
    #[inline(always)]
    pub fn ep_in15(&self) -> EpInR {
        EpInR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "OUT endpoint."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_out0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ep_out(&mut self, n: u8) -> EpOutW<EpIntSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        EpOutW::new(self, n)
    }
    #[doc = "Bit 0 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out0(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 0)
    }
    #[doc = "Bit 1 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out1(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 1)
    }
    #[doc = "Bit 2 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out2(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 2)
    }
    #[doc = "Bit 3 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out3(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out4(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 4)
    }
    #[doc = "Bit 5 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out5(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 5)
    }
    #[doc = "Bit 6 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out6(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 6)
    }
    #[doc = "Bit 7 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out7(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 7)
    }
    #[doc = "Bit 8 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out8(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 8)
    }
    #[doc = "Bit 9 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out9(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 9)
    }
    #[doc = "Bit 10 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out10(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 10)
    }
    #[doc = "Bit 11 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out11(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 11)
    }
    #[doc = "Bit 12 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out12(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 12)
    }
    #[doc = "Bit 13 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out13(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 13)
    }
    #[doc = "Bit 14 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out14(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 14)
    }
    #[doc = "Bit 15 - OUT endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_out15(&mut self) -> EpOutW<EpIntSpec> {
        EpOutW::new(self, 15)
    }
    #[doc = "IN endpoint."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ep_in0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ep_in(&mut self, n: u8) -> EpInW<EpIntSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        EpInW::new(self, n + 16)
    }
    #[doc = "Bit 16 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in0(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 16)
    }
    #[doc = "Bit 17 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in1(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 17)
    }
    #[doc = "Bit 18 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in2(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 18)
    }
    #[doc = "Bit 19 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in3(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 19)
    }
    #[doc = "Bit 20 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in4(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 20)
    }
    #[doc = "Bit 21 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in5(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 21)
    }
    #[doc = "Bit 22 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in6(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 22)
    }
    #[doc = "Bit 23 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in7(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 23)
    }
    #[doc = "Bit 24 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in8(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 24)
    }
    #[doc = "Bit 25 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in9(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 25)
    }
    #[doc = "Bit 26 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in10(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 26)
    }
    #[doc = "Bit 27 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in11(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 27)
    }
    #[doc = "Bit 28 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in12(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 28)
    }
    #[doc = "Bit 29 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in13(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 29)
    }
    #[doc = "Bit 30 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in14(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 30)
    }
    #[doc = "Bit 31 - IN endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn ep_in15(&mut self) -> EpInW<EpIntSpec> {
        EpInW::new(self, 31)
    }
}
#[doc = "USB3 Endpoint interrupt registers - ep_int0: enable, ep_int1: status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpIntSpec;
impl crate::RegisterSpec for EpIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_int::R`](R) reader structure"]
impl crate::Readable for EpIntSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_int::W`](W) writer structure"]
impl crate::Writable for EpIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ep_int[%s]
to value 0"]
impl crate::Resettable for EpIntSpec {
    const RESET_VALUE: u32 = 0;
}
