#[doc = "Register `smode` reader"]
pub type R = crate::R<SmodeSpec>;
#[doc = "Register `smode` writer"]
pub type W = crate::W<SmodeSpec>;
#[doc = "Field `nonce_mode` reader - Nonce operation mode"]
pub type NonceModeR = crate::BitReader;
#[doc = "Field `nonce_mode` writer - Nonce operation mode"]
pub type NonceModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mission_mode` reader - Mission operation mode"]
pub type MissionModeR = crate::BitReader;
#[doc = "Field `mission_mode` writer - Mission operation mode"]
pub type MissionModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `max_rejects` reader - TRNG Maximum Rejects"]
pub type MaxRejectsR = crate::FieldReader<u16>;
#[doc = "Field `max_rejects` writer - TRNG Maximum Rejects"]
pub type MaxRejectsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 2 - Nonce operation mode"]
    #[inline(always)]
    pub fn nonce_mode(&self) -> NonceModeR {
        NonceModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mission operation mode"]
    #[inline(always)]
    pub fn mission_mode(&self) -> MissionModeR {
        MissionModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - TRNG Maximum Rejects"]
    #[inline(always)]
    pub fn max_rejects(&self) -> MaxRejectsR {
        MaxRejectsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 2 - Nonce operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn nonce_mode(&mut self) -> NonceModeW<SmodeSpec> {
        NonceModeW::new(self, 2)
    }
    #[doc = "Bit 8 - Mission operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn mission_mode(&mut self) -> MissionModeW<SmodeSpec> {
        MissionModeW::new(self, 8)
    }
    #[doc = "Bits 16:31 - TRNG Maximum Rejects"]
    #[inline(always)]
    #[must_use]
    pub fn max_rejects(&mut self) -> MaxRejectsW<SmodeSpec> {
        MaxRejectsW::new(self, 16)
    }
}
#[doc = "TRNG SMODE Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmodeSpec;
impl crate::RegisterSpec for SmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smode::R`](R) reader structure"]
impl crate::Readable for SmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`smode::W`](W) writer structure"]
impl crate::Writable for SmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets smode to value 0"]
impl crate::Resettable for SmodeSpec {
    const RESET_VALUE: u32 = 0;
}
