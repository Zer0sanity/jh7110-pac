#[doc = "Register `stat` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `nonce_mode` reader - TRNG Nonce operating mode"]
pub type NonceModeR = crate::BitReader;
#[doc = "Field `r256` reader - TRNG 256-bit random number operating mode"]
pub type R256R = crate::BitReader;
#[doc = "Field `mission_mode` reader - TRNG Mission Mode operating mode"]
pub type MissionModeR = crate::BitReader;
#[doc = "Field `seeded` reader - TRNG Seeded operating mode"]
pub type SeededR = crate::BitReader;
#[doc = "Field `last_reseed0` reader - TRNG Last Reseed 0 status"]
pub type LastReseed0R = crate::BitReader;
#[doc = "Field `last_reseed1` reader - TRNG Last Reseed 1 status"]
pub type LastReseed1R = crate::BitReader;
#[doc = "Field `last_reseed2` reader - TRNG Last Reseed 2 status"]
pub type LastReseed2R = crate::BitReader;
#[doc = "Field `last_reseed3` reader - TRNG Last Reseed 3 status"]
pub type LastReseed3R = crate::BitReader;
#[doc = "Field `last_reseed4` reader - TRNG Last Reseed 4 status"]
pub type LastReseed4R = crate::BitReader;
#[doc = "Field `last_reseed5` reader - TRNG Last Reseed 5 status"]
pub type LastReseed5R = crate::BitReader;
#[doc = "Field `last_reseed6` reader - TRNG Last Reseed 6 status"]
pub type LastReseed6R = crate::BitReader;
#[doc = "Field `last_reseed7` reader - TRNG Last Reseed 7 status"]
pub type LastReseed7R = crate::BitReader;
#[doc = "Field `srvc_rqst` reader - TRNG Service Request"]
pub type SrvcRqstR = crate::BitReader;
#[doc = "Field `rand_generating` reader - TRNG Random Number Generating Status"]
pub type RandGeneratingR = crate::BitReader;
#[doc = "Field `rand_seeding` reader - TRNG Random Number Seeding Status"]
pub type RandSeedingR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - TRNG Nonce operating mode"]
    #[inline(always)]
    pub fn nonce_mode(&self) -> NonceModeR {
        NonceModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TRNG 256-bit random number operating mode"]
    #[inline(always)]
    pub fn r256(&self) -> R256R {
        R256R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TRNG Mission Mode operating mode"]
    #[inline(always)]
    pub fn mission_mode(&self) -> MissionModeR {
        MissionModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRNG Seeded operating mode"]
    #[inline(always)]
    pub fn seeded(&self) -> SeededR {
        SeededR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - TRNG Last Reseed 0 status"]
    #[inline(always)]
    pub fn last_reseed0(&self) -> LastReseed0R {
        LastReseed0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TRNG Last Reseed 1 status"]
    #[inline(always)]
    pub fn last_reseed1(&self) -> LastReseed1R {
        LastReseed1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TRNG Last Reseed 2 status"]
    #[inline(always)]
    pub fn last_reseed2(&self) -> LastReseed2R {
        LastReseed2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TRNG Last Reseed 3 status"]
    #[inline(always)]
    pub fn last_reseed3(&self) -> LastReseed3R {
        LastReseed3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TRNG Last Reseed 4 status"]
    #[inline(always)]
    pub fn last_reseed4(&self) -> LastReseed4R {
        LastReseed4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRNG Last Reseed 5 status"]
    #[inline(always)]
    pub fn last_reseed5(&self) -> LastReseed5R {
        LastReseed5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TRNG Last Reseed 6 status"]
    #[inline(always)]
    pub fn last_reseed6(&self) -> LastReseed6R {
        LastReseed6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TRNG Last Reseed 7 status"]
    #[inline(always)]
    pub fn last_reseed7(&self) -> LastReseed7R {
        LastReseed7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - TRNG Service Request"]
    #[inline(always)]
    pub fn srvc_rqst(&self) -> SrvcRqstR {
        SrvcRqstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - TRNG Random Number Generating Status"]
    #[inline(always)]
    pub fn rand_generating(&self) -> RandGeneratingR {
        RandGeneratingR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TRNG Random Number Seeding Status"]
    #[inline(always)]
    pub fn rand_seeding(&self) -> RandSeedingR {
        RandSeedingR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "TRNG STAT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets stat to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
