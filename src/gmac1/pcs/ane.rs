#[doc = "Register `ane[%s]` reader"]
pub type R = crate::R<AneSpec>;
#[doc = "Register `ane[%s]` writer"]
pub type W = crate::W<AneSpec>;
#[doc = "Field `fd` reader - ANE Full Duplex"]
pub type FdR = crate::BitReader;
#[doc = "Field `fd` writer - ANE Full Duplex"]
pub type FdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hd` reader - ANE Half Duplex"]
pub type HdR = crate::BitReader;
#[doc = "Field `hd` writer - ANE Half Duplex"]
pub type HdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pse` reader - ANE Pause"]
pub type PseR = crate::FieldReader;
#[doc = "Field `pse` writer - ANE Pause"]
pub type PseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rfe` reader - ANE RFE"]
pub type RfeR = crate::FieldReader;
#[doc = "Field `rfe` writer - ANE RFE"]
pub type RfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ack` reader - ANE ACK"]
pub type AckR = crate::BitReader;
#[doc = "Field `ack` writer - ANE ACK"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - ANE Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FdR {
        FdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ANE Half Duplex"]
    #[inline(always)]
    pub fn hd(&self) -> HdR {
        HdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - ANE Pause"]
    #[inline(always)]
    pub fn pse(&self) -> PseR {
        PseR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 12:13 - ANE RFE"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - ANE ACK"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - ANE Full Duplex"]
    #[inline(always)]
    #[must_use]
    pub fn fd(&mut self) -> FdW<AneSpec> {
        FdW::new(self, 5)
    }
    #[doc = "Bit 6 - ANE Half Duplex"]
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HdW<AneSpec> {
        HdW::new(self, 6)
    }
    #[doc = "Bits 7:8 - ANE Pause"]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PseW<AneSpec> {
        PseW::new(self, 7)
    }
    #[doc = "Bits 12:13 - ANE RFE"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<AneSpec> {
        RfeW::new(self, 12)
    }
    #[doc = "Bit 14 - ANE ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<AneSpec> {
        AckW::new(self, 14)
    }
}
#[doc = "Auto-Negotiation Extend Advertisement and Link Partner Ability\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ane::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ane::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AneSpec;
impl crate::RegisterSpec for AneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ane::R`](R) reader structure"]
impl crate::Readable for AneSpec {}
#[doc = "`write(|w| ..)` method takes [`ane::W`](W) writer structure"]
impl crate::Writable for AneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ane[%s]
to value 0"]
impl crate::Resettable for AneSpec {
    const RESET_VALUE: u32 = 0;
}
