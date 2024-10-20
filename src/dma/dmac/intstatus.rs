#[doc = "Register `intstatus` reader"]
pub type R = crate::R<IntstatusSpec>;
#[doc = "Field `intstat_ch(1-8)` reader - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
pub type IntstatChR = crate::BitReader;
#[doc = "Field `common_intstat` reader - DMAC Common Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
pub type CommonIntstatR = crate::BitReader;
impl R {
    #[doc = "DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `intstat_ch1` field"]
    #[inline(always)]
    pub fn intstat_ch(&self, n: u8) -> IntstatChR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        IntstatChR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch_iter(&self) -> impl Iterator<Item = IntstatChR> + '_ {
        (0..8).map(move |n| IntstatChR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch1(&self) -> IntstatChR {
        IntstatChR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch2(&self) -> IntstatChR {
        IntstatChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch3(&self) -> IntstatChR {
        IntstatChR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch4(&self) -> IntstatChR {
        IntstatChR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch5(&self) -> IntstatChR {
        IntstatChR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch6(&self) -> IntstatChR {
        IntstatChR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch7(&self) -> IntstatChR {
        IntstatChR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC Channel Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn intstat_ch8(&self) -> IntstatChR {
        IntstatChR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - DMAC Common Interrupt Status - 0: interrupt inactive, 1: interrupt active"]
    #[inline(always)]
    pub fn common_intstat(&self) -> CommonIntstatR {
        CommonIntstatR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "DMAC Interrupt Status register contains the DMAC interrupt status. Only exists when DMAX_NUM_CHANNELS &lt;= 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatusSpec;
impl crate::RegisterSpec for IntstatusSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for IntstatusSpec {}
#[doc = "`reset()` method sets intstatus to value 0"]
impl crate::Resettable for IntstatusSpec {
    const RESET_VALUE: u64 = 0;
}
