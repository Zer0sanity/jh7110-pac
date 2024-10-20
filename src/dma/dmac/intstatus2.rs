#[doc = "Register `intstatus2` reader"]
pub type R = crate::R<Intstatus2Spec>;
#[doc = "Field `intstat(_ch1,_ch2,_ch3,_ch4,_ch5,_ch6,_ch7,_ch8,_ch9,_ch10,_ch11,_ch12,_ch13,_ch14,_ch15,_ch16,_ch17,_ch18,_ch19,_ch20,_ch21,_ch22,_ch23,_ch24,_ch25,_ch26,_ch27,_ch28,_ch29,_ch30,_ch31,_ch32,_common)` reader - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
pub type IntstatR = crate::BitReader;
impl R {
    #[doc = "DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `intstat_ch1` field"]
    #[inline(always)]
    pub fn intstat(&self, n: u8) -> IntstatR {
        #[allow(clippy::no_effect)]
        [(); 33][n as usize];
        IntstatR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_iter(&self) -> impl Iterator<Item = IntstatR> + '_ {
        (0..33).map(move |n| IntstatR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch1(&self) -> IntstatR {
        IntstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch2(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch3(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch4(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch5(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch6(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch7(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch8(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch9(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch10(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch11(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch12(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch13(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch14(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch15(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch16(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch17(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch18(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch19(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch20(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch21(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch22(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch23(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch24(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch25(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch26(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch27(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch28(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch29(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch30(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch31(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch32(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 32 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_common(&self) -> IntstatR {
        IntstatR::new(((self.bits >> 32) & 1) != 0)
    }
}
#[doc = "DMAC Channel Interrupt Status register contains the DMAC channel interrupt status. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intstatus2Spec;
impl crate::RegisterSpec for Intstatus2Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intstatus2::R`](R) reader structure"]
impl crate::Readable for Intstatus2Spec {}
#[doc = "`reset()` method sets intstatus2 to value 0"]
impl crate::Resettable for Intstatus2Spec {
    const RESET_VALUE: u64 = 0;
}
