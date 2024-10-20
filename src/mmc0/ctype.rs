#[doc = "Register `ctype` reader"]
pub type R = crate::R<CtypeSpec>;
#[doc = "Register `ctype` writer"]
pub type W = crate::W<CtypeSpec>;
#[doc = "MMC card type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ctype {
    #[doc = "0: 1-bit serial transfer MMC card type."]
    Type1 = 0,
    #[doc = "1: 4-bit serial transfer MMC card type."]
    Type4 = 1,
    #[doc = "65536: 8-bit serial transfer MMC card type."]
    Type8 = 65536,
}
impl From<Ctype> for u32 {
    #[inline(always)]
    fn from(variant: Ctype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctype {
    type Ux = u32;
}
#[doc = "Field `ctype` reader - MMC card type"]
pub type CtypeR = crate::FieldReader<Ctype>;
impl CtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctype> {
        match self.bits {
            0 => Some(Ctype::Type1),
            1 => Some(Ctype::Type4),
            65536 => Some(Ctype::Type8),
            _ => None,
        }
    }
    #[doc = "1-bit serial transfer MMC card type."]
    #[inline(always)]
    pub fn is_type1(&self) -> bool {
        *self == Ctype::Type1
    }
    #[doc = "4-bit serial transfer MMC card type."]
    #[inline(always)]
    pub fn is_type4(&self) -> bool {
        *self == Ctype::Type4
    }
    #[doc = "8-bit serial transfer MMC card type."]
    #[inline(always)]
    pub fn is_type8(&self) -> bool {
        *self == Ctype::Type8
    }
}
#[doc = "Field `ctype` writer - MMC card type"]
pub type CtypeW<'a, REG> = crate::FieldWriter<'a, REG, 17, Ctype>;
impl<'a, REG> CtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "1-bit serial transfer MMC card type."]
    #[inline(always)]
    pub fn type1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctype::Type1)
    }
    #[doc = "4-bit serial transfer MMC card type."]
    #[inline(always)]
    pub fn type4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctype::Type4)
    }
    #[doc = "8-bit serial transfer MMC card type."]
    #[inline(always)]
    pub fn type8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctype::Type8)
    }
}
impl R {
    #[doc = "Bits 0:16 - MMC card type"]
    #[inline(always)]
    pub fn ctype(&self) -> CtypeR {
        CtypeR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - MMC card type"]
    #[inline(always)]
    #[must_use]
    pub fn ctype(&mut self) -> CtypeW<CtypeSpec> {
        CtypeW::new(self, 0)
    }
}
#[doc = "MMC card type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtypeSpec;
impl crate::RegisterSpec for CtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctype::R`](R) reader structure"]
impl crate::Readable for CtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`ctype::W`](W) writer structure"]
impl crate::Writable for CtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctype to value 0"]
impl crate::Resettable for CtypeSpec {
    const RESET_VALUE: u32 = 0;
}
