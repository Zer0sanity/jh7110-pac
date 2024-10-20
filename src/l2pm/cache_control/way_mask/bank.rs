#[doc = "Register `bank` reader"]
pub type R = crate::R<BankSpec>;
#[doc = "Register `bank` writer"]
pub type W = crate::W<BankSpec>;
#[doc = "Field `way_mask(0-15)` reader - Way enable mask."]
pub type WayMaskR = crate::BitReader;
#[doc = "Field `way_mask(0-15)` writer - Way enable mask."]
pub type WayMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Way enable mask."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `way_mask0` field"]
    #[inline(always)]
    pub fn way_mask(&self, n: u8) -> WayMaskR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        WayMaskR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Way enable mask."]
    #[inline(always)]
    pub fn way_mask_iter(&self) -> impl Iterator<Item = WayMaskR> + '_ {
        (0..16).map(move |n| WayMaskR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask0(&self) -> WayMaskR {
        WayMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask1(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask2(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask3(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask4(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask5(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask6(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask7(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask8(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask9(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask10(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask11(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask12(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask13(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask14(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Way enable mask."]
    #[inline(always)]
    pub fn way_mask15(&self) -> WayMaskR {
        WayMaskR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Way enable mask."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `way_mask0` field"]
    #[inline(always)]
    #[must_use]
    pub fn way_mask(&mut self, n: u8) -> WayMaskW<BankSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        WayMaskW::new(self, n)
    }
    #[doc = "Bit 0 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask0(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask1(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask2(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask3(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask4(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask5(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask6(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask7(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 7)
    }
    #[doc = "Bit 8 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask8(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 8)
    }
    #[doc = "Bit 9 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask9(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask10(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 10)
    }
    #[doc = "Bit 11 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask11(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 11)
    }
    #[doc = "Bit 12 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask12(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 12)
    }
    #[doc = "Bit 13 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask13(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 13)
    }
    #[doc = "Bit 14 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask14(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 14)
    }
    #[doc = "Bit 15 - Way enable mask."]
    #[inline(always)]
    #[must_use]
    pub fn way_mask15(&mut self) -> WayMaskW<BankSpec> {
        WayMaskW::new(self, 15)
    }
}
#[doc = "L2 Cache Control Way Mask bank registers. Configures the masks to enable cache bank ways.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BankSpec;
impl crate::RegisterSpec for BankSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bank::R`](R) reader structure"]
impl crate::Readable for BankSpec {}
#[doc = "`write(|w| ..)` method takes [`bank::W`](W) writer structure"]
impl crate::Writable for BankSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bank to value 0"]
impl crate::Resettable for BankSpec {
    const RESET_VALUE: u32 = 0;
}
