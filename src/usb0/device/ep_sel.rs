#[doc = "Register `ep_sel` reader"]
pub type R = crate::R<EpSelSpec>;
#[doc = "Register `ep_sel` writer"]
pub type W = crate::W<EpSelSpec>;
#[doc = "Field `epno` reader - Endpoint number."]
pub type EpnoR = crate::FieldReader;
#[doc = "Field `epno` writer - Endpoint number."]
pub type EpnoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Endpoint direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: OUT direction"]
    Out = 0,
    #[doc = "1: IN direction"]
    In = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dir` reader - Endpoint direction."]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Out,
            true => Dir::In,
        }
    }
    #[doc = "OUT direction"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Dir::Out
    }
    #[doc = "IN direction"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Dir::In
    }
}
#[doc = "Field `dir` writer - Endpoint direction."]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OUT direction"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Out)
    }
    #[doc = "IN direction"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::In)
    }
}
impl R {
    #[doc = "Bits 0:3 - Endpoint number."]
    #[inline(always)]
    pub fn epno(&self) -> EpnoR {
        EpnoR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Endpoint direction."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint number."]
    #[inline(always)]
    #[must_use]
    pub fn epno(&mut self) -> EpnoW<EpSelSpec> {
        EpnoW::new(self, 0)
    }
    #[doc = "Bit 7 - Endpoint direction."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<EpSelSpec> {
        DirW::new(self, 7)
    }
}
#[doc = "USB3 Endpoint select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpSelSpec;
impl crate::RegisterSpec for EpSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_sel::R`](R) reader structure"]
impl crate::Readable for EpSelSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_sel::W`](W) writer structure"]
impl crate::Writable for EpSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ep_sel to value 0"]
impl crate::Resettable for EpSelSpec {
    const RESET_VALUE: u32 = 0;
}
