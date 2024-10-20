#[doc = "Register `istat` reader"]
pub type R = crate::R<IstatSpec>;
#[doc = "Field `rand_rdy` reader - RAND Ready Status"]
pub type RandRdyR = crate::BitReader;
#[doc = "Field `seed_done` reader - Seed Done Status"]
pub type SeedDoneR = crate::BitReader;
#[doc = "Field `lfsr_lockup` reader - LFSR Lockup Status"]
pub type LfsrLockupR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RAND Ready Status"]
    #[inline(always)]
    pub fn rand_rdy(&self) -> RandRdyR {
        RandRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Seed Done Status"]
    #[inline(always)]
    pub fn seed_done(&self) -> SeedDoneR {
        SeedDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LFSR Lockup Status"]
    #[inline(always)]
    pub fn lfsr_lockup(&self) -> LfsrLockupR {
        LfsrLockupR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "TRNG Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstatSpec;
impl crate::RegisterSpec for IstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istat::R`](R) reader structure"]
impl crate::Readable for IstatSpec {}
#[doc = "`reset()` method sets istat to value 0"]
impl crate::Resettable for IstatSpec {
    const RESET_VALUE: u32 = 0;
}
