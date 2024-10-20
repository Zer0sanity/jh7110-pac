#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "DMA memory-access read protection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Marprot {
    #[doc = "0: Protected access to memory - can cause problem with cache"]
    Secure = 0,
    #[doc = "2: Unprotected access to memory"]
    NonSecure = 2,
}
impl From<Marprot> for u8 {
    #[inline(always)]
    fn from(variant: Marprot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Marprot {
    type Ux = u8;
}
#[doc = "Field `marprot` reader - DMA memory-access read protection."]
pub type MarprotR = crate::FieldReader<Marprot>;
impl MarprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Marprot> {
        match self.bits {
            0 => Some(Marprot::Secure),
            2 => Some(Marprot::NonSecure),
            _ => None,
        }
    }
    #[doc = "Protected access to memory - can cause problem with cache"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Marprot::Secure
    }
    #[doc = "Unprotected access to memory"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Marprot::NonSecure
    }
}
#[doc = "Field `marprot` writer - DMA memory-access read protection."]
pub type MarprotW<'a, REG> = crate::FieldWriter<'a, REG, 3, Marprot>;
impl<'a, REG> MarprotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Protected access to memory - can cause problem with cache"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Marprot::Secure)
    }
    #[doc = "Unprotected access to memory"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Marprot::NonSecure)
    }
}
#[doc = "DMA memory-access write protection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mawprot {
    #[doc = "0: Protected access to memory - can cause problem with cache"]
    Secure = 0,
    #[doc = "2: Unprotected access to memory"]
    NonSecure = 2,
}
impl From<Mawprot> for u8 {
    #[inline(always)]
    fn from(variant: Mawprot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mawprot {
    type Ux = u8;
}
#[doc = "Field `mawprot` reader - DMA memory-access write protection."]
pub type MawprotR = crate::FieldReader<Mawprot>;
impl MawprotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mawprot> {
        match self.bits {
            0 => Some(Mawprot::Secure),
            2 => Some(Mawprot::NonSecure),
            _ => None,
        }
    }
    #[doc = "Protected access to memory - can cause problem with cache"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Mawprot::Secure
    }
    #[doc = "Unprotected access to memory"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == Mawprot::NonSecure
    }
}
#[doc = "Field `mawprot` writer - DMA memory-access write protection."]
pub type MawprotW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mawprot>;
impl<'a, REG> MawprotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Protected access to memory - can cause problem with cache"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Mawprot::Secure)
    }
    #[doc = "Unprotected access to memory"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(Mawprot::NonSecure)
    }
}
impl R {
    #[doc = "Bits 0:2 - DMA memory-access read protection."]
    #[inline(always)]
    pub fn marprot(&self) -> MarprotR {
        MarprotR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - DMA memory-access write protection."]
    #[inline(always)]
    pub fn mawprot(&self) -> MawprotR {
        MawprotR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DMA memory-access read protection."]
    #[inline(always)]
    #[must_use]
    pub fn marprot(&mut self) -> MarprotW<CtrlSpec> {
        MarprotW::new(self, 0)
    }
    #[doc = "Bits 16:18 - DMA memory-access write protection."]
    #[inline(always)]
    #[must_use]
    pub fn mawprot(&mut self) -> MawprotW<CtrlSpec> {
        MawprotW::new(self, 16)
    }
}
#[doc = "Device DMA AXI control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
