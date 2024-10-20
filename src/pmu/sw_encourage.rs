#[doc = "Register `sw_encourage` reader"]
pub type R = crate::R<SwEncourageSpec>;
#[doc = "Register `sw_encourage` writer"]
pub type W = crate::W<SwEncourageSpec>;
#[doc = "Software encouragement variants.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwEncourage {
    #[doc = "0: Disable Software Encourage."]
    Off = 0,
    #[doc = "5: Enable LO Software Encourage."]
    EnLo = 5,
    #[doc = "10: Disable LO Software Encourage."]
    DisLo = 10,
    #[doc = "80: Enable HI Software Encourage."]
    EnHi = 80,
    #[doc = "160: Disable HI Software Encourage."]
    DisHi = 160,
    #[doc = "255: Enable Software Encourage."]
    On = 255,
}
impl From<SwEncourage> for u8 {
    #[inline(always)]
    fn from(variant: SwEncourage) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwEncourage {
    type Ux = u8;
}
#[doc = "Field `sw_encourage` reader - Software encouragement variants."]
pub type SwEncourageR = crate::FieldReader<SwEncourage>;
impl SwEncourageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwEncourage> {
        match self.bits {
            0 => Some(SwEncourage::Off),
            5 => Some(SwEncourage::EnLo),
            10 => Some(SwEncourage::DisLo),
            80 => Some(SwEncourage::EnHi),
            160 => Some(SwEncourage::DisHi),
            255 => Some(SwEncourage::On),
            _ => None,
        }
    }
    #[doc = "Disable Software Encourage."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SwEncourage::Off
    }
    #[doc = "Enable LO Software Encourage."]
    #[inline(always)]
    pub fn is_en_lo(&self) -> bool {
        *self == SwEncourage::EnLo
    }
    #[doc = "Disable LO Software Encourage."]
    #[inline(always)]
    pub fn is_dis_lo(&self) -> bool {
        *self == SwEncourage::DisLo
    }
    #[doc = "Enable HI Software Encourage."]
    #[inline(always)]
    pub fn is_en_hi(&self) -> bool {
        *self == SwEncourage::EnHi
    }
    #[doc = "Disable HI Software Encourage."]
    #[inline(always)]
    pub fn is_dis_hi(&self) -> bool {
        *self == SwEncourage::DisHi
    }
    #[doc = "Enable Software Encourage."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == SwEncourage::On
    }
}
#[doc = "Field `sw_encourage` writer - Software encouragement variants."]
pub type SwEncourageW<'a, REG> = crate::FieldWriter<'a, REG, 8, SwEncourage>;
impl<'a, REG> SwEncourageW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable Software Encourage."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(SwEncourage::Off)
    }
    #[doc = "Enable LO Software Encourage."]
    #[inline(always)]
    pub fn en_lo(self) -> &'a mut crate::W<REG> {
        self.variant(SwEncourage::EnLo)
    }
    #[doc = "Disable LO Software Encourage."]
    #[inline(always)]
    pub fn dis_lo(self) -> &'a mut crate::W<REG> {
        self.variant(SwEncourage::DisLo)
    }
    #[doc = "Enable HI Software Encourage."]
    #[inline(always)]
    pub fn en_hi(self) -> &'a mut crate::W<REG> {
        self.variant(SwEncourage::EnHi)
    }
    #[doc = "Disable HI Software Encourage."]
    #[inline(always)]
    pub fn dis_hi(self) -> &'a mut crate::W<REG> {
        self.variant(SwEncourage::DisHi)
    }
    #[doc = "Enable Software Encourage."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(SwEncourage::On)
    }
}
impl R {
    #[doc = "Bits 0:7 - Software encouragement variants."]
    #[inline(always)]
    pub fn sw_encourage(&self) -> SwEncourageR {
        SwEncourageR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software encouragement variants."]
    #[inline(always)]
    #[must_use]
    pub fn sw_encourage(&mut self) -> SwEncourageW<SwEncourageSpec> {
        SwEncourageW::new(self, 0)
    }
}
#[doc = "Software encouragement register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_encourage::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_encourage::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwEncourageSpec;
impl crate::RegisterSpec for SwEncourageSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_encourage::R`](R) reader structure"]
impl crate::Readable for SwEncourageSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_encourage::W`](W) writer structure"]
impl crate::Writable for SwEncourageSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sw_encourage to value 0"]
impl crate::Resettable for SwEncourageSpec {
    const RESET_VALUE: u32 = 0;
}
