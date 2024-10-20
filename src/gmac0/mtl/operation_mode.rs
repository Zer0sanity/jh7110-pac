#[doc = "Register `operation_mode` reader"]
pub type R = crate::R<OperationModeSpec>;
#[doc = "Register `operation_mode` writer"]
pub type W = crate::W<OperationModeSpec>;
#[doc = "RAA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Raa {
    #[doc = "0: RAA SP"]
    Sp = 0,
    #[doc = "1: RAA SP"]
    Wsp = 1,
}
impl From<Raa> for bool {
    #[inline(always)]
    fn from(variant: Raa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `raa` reader - RAA"]
pub type RaaR = crate::BitReader<Raa>;
impl RaaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Raa {
        match self.bits {
            false => Raa::Sp,
            true => Raa::Wsp,
        }
    }
    #[doc = "RAA SP"]
    #[inline(always)]
    pub fn is_sp(&self) -> bool {
        *self == Raa::Sp
    }
    #[doc = "RAA SP"]
    #[inline(always)]
    pub fn is_wsp(&self) -> bool {
        *self == Raa::Wsp
    }
}
#[doc = "Field `raa` writer - RAA"]
pub type RaaW<'a, REG> = crate::BitWriter<'a, REG, Raa>;
impl<'a, REG> RaaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RAA SP"]
    #[inline(always)]
    pub fn sp(self) -> &'a mut crate::W<REG> {
        self.variant(Raa::Sp)
    }
    #[doc = "RAA SP"]
    #[inline(always)]
    pub fn wsp(self) -> &'a mut crate::W<REG> {
        self.variant(Raa::Wsp)
    }
}
#[doc = "Scheduling Algorithm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Schalg {
    #[doc = "0: Schedule Algorithm WRR"]
    Wrr = 0,
    #[doc = "1: Schedule Algorithm WFQ"]
    Wfq = 1,
    #[doc = "2: Schedule Algorithm DWRR"]
    Dwrr = 2,
    #[doc = "3: Schedule Algorithm SP"]
    Sp = 3,
}
impl From<Schalg> for u8 {
    #[inline(always)]
    fn from(variant: Schalg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Schalg {
    type Ux = u8;
}
#[doc = "Field `schalg` reader - Scheduling Algorithm"]
pub type SchalgR = crate::FieldReader<Schalg>;
impl SchalgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Schalg {
        match self.bits {
            0 => Schalg::Wrr,
            1 => Schalg::Wfq,
            2 => Schalg::Dwrr,
            3 => Schalg::Sp,
            _ => unreachable!(),
        }
    }
    #[doc = "Schedule Algorithm WRR"]
    #[inline(always)]
    pub fn is_wrr(&self) -> bool {
        *self == Schalg::Wrr
    }
    #[doc = "Schedule Algorithm WFQ"]
    #[inline(always)]
    pub fn is_wfq(&self) -> bool {
        *self == Schalg::Wfq
    }
    #[doc = "Schedule Algorithm DWRR"]
    #[inline(always)]
    pub fn is_dwrr(&self) -> bool {
        *self == Schalg::Dwrr
    }
    #[doc = "Schedule Algorithm SP"]
    #[inline(always)]
    pub fn is_sp(&self) -> bool {
        *self == Schalg::Sp
    }
}
#[doc = "Field `schalg` writer - Scheduling Algorithm"]
pub type SchalgW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Schalg>;
impl<'a, REG> SchalgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Schedule Algorithm WRR"]
    #[inline(always)]
    pub fn wrr(self) -> &'a mut crate::W<REG> {
        self.variant(Schalg::Wrr)
    }
    #[doc = "Schedule Algorithm WFQ"]
    #[inline(always)]
    pub fn wfq(self) -> &'a mut crate::W<REG> {
        self.variant(Schalg::Wfq)
    }
    #[doc = "Schedule Algorithm DWRR"]
    #[inline(always)]
    pub fn dwrr(self) -> &'a mut crate::W<REG> {
        self.variant(Schalg::Dwrr)
    }
    #[doc = "Schedule Algorithm SP"]
    #[inline(always)]
    pub fn sp(self) -> &'a mut crate::W<REG> {
        self.variant(Schalg::Sp)
    }
}
#[doc = "Field `frpe` reader - FRPE"]
pub type FrpeR = crate::BitReader;
#[doc = "Field `frpe` writer - FRPE"]
pub type FrpeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - RAA"]
    #[inline(always)]
    pub fn raa(&self) -> RaaR {
        RaaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Scheduling Algorithm"]
    #[inline(always)]
    pub fn schalg(&self) -> SchalgR {
        SchalgR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 15 - FRPE"]
    #[inline(always)]
    pub fn frpe(&self) -> FrpeR {
        FrpeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - RAA"]
    #[inline(always)]
    #[must_use]
    pub fn raa(&mut self) -> RaaW<OperationModeSpec> {
        RaaW::new(self, 2)
    }
    #[doc = "Bits 5:6 - Scheduling Algorithm"]
    #[inline(always)]
    #[must_use]
    pub fn schalg(&mut self) -> SchalgW<OperationModeSpec> {
        SchalgW::new(self, 5)
    }
    #[doc = "Bit 15 - FRPE"]
    #[inline(always)]
    #[must_use]
    pub fn frpe(&mut self) -> FrpeW<OperationModeSpec> {
        FrpeW::new(self, 15)
    }
}
#[doc = "MTL Operation Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`operation_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operation_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OperationModeSpec;
impl crate::RegisterSpec for OperationModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`operation_mode::R`](R) reader structure"]
impl crate::Readable for OperationModeSpec {}
#[doc = "`write(|w| ..)` method takes [`operation_mode::W`](W) writer structure"]
impl crate::Writable for OperationModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets operation_mode to value 0"]
impl crate::Resettable for OperationModeSpec {
    const RESET_VALUE: u32 = 0;
}
