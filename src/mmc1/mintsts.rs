#[doc = "Register `mintsts` reader"]
pub type R = crate::R<MintstsSpec>;
#[doc = "Register `mintsts` writer"]
pub type W = crate::W<MintstsSpec>;
#[doc = "Field `int(_cd,_resp_err,_cmd_done,_data_over,_txdr,_rxdr,_rcrc,_dcrc,_rto,_drto,_hto_volt_switch,_frun,_hle,_sbe,_acd,_ebe,_sdio0,_sdio1,_sdio2,_sdio3,_sdio4,_sdio5,_sdio6,_sdio7,_sdio8,_sdio9,_sdio10,_sdio11,_sdio12,_sdio13,_sdio14,_sdio15)` reader - MMC MINT status"]
pub type IntR = crate::BitReader;
#[doc = "Field `int(_cd,_resp_err,_cmd_done,_data_over,_txdr,_rxdr,_rcrc,_dcrc,_rto,_drto,_hto_volt_switch,_frun,_hle,_sbe,_acd,_ebe,_sdio0,_sdio1,_sdio2,_sdio3,_sdio4,_sdio5,_sdio6,_sdio7,_sdio8,_sdio9,_sdio10,_sdio11,_sdio12,_sdio13,_sdio14,_sdio15)` writer - MMC MINT status"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "MMC MINT status"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `int_cd` field"]
    #[inline(always)]
    pub fn int(&self, n: u8) -> IntR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        IntR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MMC MINT status"]
    #[inline(always)]
    pub fn int_iter(&self) -> impl Iterator<Item = IntR> + '_ {
        (0..32).map(move |n| IntR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MMC MINT status"]
    #[inline(always)]
    pub fn int_cd(&self) -> IntR {
        IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC MINT status"]
    #[inline(always)]
    pub fn int_resp_err(&self) -> IntR {
        IntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC MINT status"]
    #[inline(always)]
    pub fn int_cmd_done(&self) -> IntR {
        IntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC MINT status"]
    #[inline(always)]
    pub fn int_data_over(&self) -> IntR {
        IntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC MINT status"]
    #[inline(always)]
    pub fn int_txdr(&self) -> IntR {
        IntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC MINT status"]
    #[inline(always)]
    pub fn int_rxdr(&self) -> IntR {
        IntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC MINT status"]
    #[inline(always)]
    pub fn int_rcrc(&self) -> IntR {
        IntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC MINT status"]
    #[inline(always)]
    pub fn int_dcrc(&self) -> IntR {
        IntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC MINT status"]
    #[inline(always)]
    pub fn int_rto(&self) -> IntR {
        IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC MINT status"]
    #[inline(always)]
    pub fn int_drto(&self) -> IntR {
        IntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC MINT status"]
    #[inline(always)]
    pub fn int_hto_volt_switch(&self) -> IntR {
        IntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC MINT status"]
    #[inline(always)]
    pub fn int_frun(&self) -> IntR {
        IntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC MINT status"]
    #[inline(always)]
    pub fn int_hle(&self) -> IntR {
        IntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sbe(&self) -> IntR {
        IntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC MINT status"]
    #[inline(always)]
    pub fn int_acd(&self) -> IntR {
        IntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC MINT status"]
    #[inline(always)]
    pub fn int_ebe(&self) -> IntR {
        IntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio0(&self) -> IntR {
        IntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio1(&self) -> IntR {
        IntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio2(&self) -> IntR {
        IntR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio3(&self) -> IntR {
        IntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio4(&self) -> IntR {
        IntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio5(&self) -> IntR {
        IntR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio6(&self) -> IntR {
        IntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio7(&self) -> IntR {
        IntR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio8(&self) -> IntR {
        IntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio9(&self) -> IntR {
        IntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio10(&self) -> IntR {
        IntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio11(&self) -> IntR {
        IntR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio12(&self) -> IntR {
        IntR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio13(&self) -> IntR {
        IntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio14(&self) -> IntR {
        IntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC MINT status"]
    #[inline(always)]
    pub fn int_sdio15(&self) -> IntR {
        IntR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "MMC MINT status"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `int_cd` field"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self, n: u8) -> IntW<MintstsSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        IntW::new(self, n)
    }
    #[doc = "Bit 0 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_cd(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_resp_err(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 1)
    }
    #[doc = "Bit 2 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_cmd_done(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 2)
    }
    #[doc = "Bit 3 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_data_over(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 3)
    }
    #[doc = "Bit 4 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_txdr(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 4)
    }
    #[doc = "Bit 5 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_rxdr(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 5)
    }
    #[doc = "Bit 6 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_rcrc(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 6)
    }
    #[doc = "Bit 7 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_dcrc(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_rto(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 8)
    }
    #[doc = "Bit 9 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_drto(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 9)
    }
    #[doc = "Bit 10 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_hto_volt_switch(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 10)
    }
    #[doc = "Bit 11 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_frun(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 11)
    }
    #[doc = "Bit 12 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_hle(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 12)
    }
    #[doc = "Bit 13 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sbe(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 13)
    }
    #[doc = "Bit 14 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_acd(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 14)
    }
    #[doc = "Bit 15 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_ebe(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 15)
    }
    #[doc = "Bit 16 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio0(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 16)
    }
    #[doc = "Bit 17 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio1(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 17)
    }
    #[doc = "Bit 18 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio2(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 18)
    }
    #[doc = "Bit 19 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio3(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 19)
    }
    #[doc = "Bit 20 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio4(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 20)
    }
    #[doc = "Bit 21 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio5(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 21)
    }
    #[doc = "Bit 22 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio6(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 22)
    }
    #[doc = "Bit 23 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio7(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 23)
    }
    #[doc = "Bit 24 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio8(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 24)
    }
    #[doc = "Bit 25 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio9(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 25)
    }
    #[doc = "Bit 26 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio10(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 26)
    }
    #[doc = "Bit 27 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio11(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 27)
    }
    #[doc = "Bit 28 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio12(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 28)
    }
    #[doc = "Bit 29 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio13(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 29)
    }
    #[doc = "Bit 30 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio14(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 30)
    }
    #[doc = "Bit 31 - MMC MINT status"]
    #[inline(always)]
    #[must_use]
    pub fn int_sdio15(&mut self) -> IntW<MintstsSpec> {
        IntW::new(self, 31)
    }
}
#[doc = "MMC MINT status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MintstsSpec;
impl crate::RegisterSpec for MintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mintsts::R`](R) reader structure"]
impl crate::Readable for MintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`mintsts::W`](W) writer structure"]
impl crate::Writable for MintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mintsts to value 0"]
impl crate::Resettable for MintstsSpec {
    const RESET_VALUE: u32 = 0;
}
