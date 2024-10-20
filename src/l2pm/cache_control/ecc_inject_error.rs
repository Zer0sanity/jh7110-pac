#[doc = "Register `ecc_inject_error` reader"]
pub type R = crate::R<EccInjectErrorSpec>;
#[doc = "Register `ecc_inject_error` writer"]
pub type W = crate::W<EccInjectErrorSpec>;
#[doc = "Field `ecc_toggle_bit` reader - Toggle (corrupt) this bit index on the next cache operation."]
pub type EccToggleBitR = crate::FieldReader;
#[doc = "Field `ecc_toggle_bit` writer - Toggle (corrupt) this bit index on the next cache operation."]
pub type EccToggleBitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Toggle (corrupt) this bit index on the next cache operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EccToggleType {
    #[doc = "0: ECC `data` type corruption."]
    Data = 0,
    #[doc = "1: ECC `directory` type corruption."]
    Directory = 1,
}
impl From<EccToggleType> for bool {
    #[inline(always)]
    fn from(variant: EccToggleType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ecc_toggle_type` reader - Toggle (corrupt) this bit index on the next cache operation."]
pub type EccToggleTypeR = crate::BitReader<EccToggleType>;
impl EccToggleTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EccToggleType {
        match self.bits {
            false => EccToggleType::Data,
            true => EccToggleType::Directory,
        }
    }
    #[doc = "ECC `data` type corruption."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == EccToggleType::Data
    }
    #[doc = "ECC `directory` type corruption."]
    #[inline(always)]
    pub fn is_directory(&self) -> bool {
        *self == EccToggleType::Directory
    }
}
#[doc = "Field `ecc_toggle_type` writer - Toggle (corrupt) this bit index on the next cache operation."]
pub type EccToggleTypeW<'a, REG> = crate::BitWriter<'a, REG, EccToggleType>;
impl<'a, REG> EccToggleTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ECC `data` type corruption."]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(EccToggleType::Data)
    }
    #[doc = "ECC `directory` type corruption."]
    #[inline(always)]
    pub fn directory(self) -> &'a mut crate::W<REG> {
        self.variant(EccToggleType::Directory)
    }
}
impl R {
    #[doc = "Bits 0:7 - Toggle (corrupt) this bit index on the next cache operation."]
    #[inline(always)]
    pub fn ecc_toggle_bit(&self) -> EccToggleBitR {
        EccToggleBitR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Toggle (corrupt) this bit index on the next cache operation."]
    #[inline(always)]
    pub fn ecc_toggle_type(&self) -> EccToggleTypeR {
        EccToggleTypeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Toggle (corrupt) this bit index on the next cache operation."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_toggle_bit(&mut self) -> EccToggleBitW<EccInjectErrorSpec> {
        EccToggleBitW::new(self, 0)
    }
    #[doc = "Bit 16 - Toggle (corrupt) this bit index on the next cache operation."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_toggle_type(&mut self) -> EccToggleTypeW<EccInjectErrorSpec> {
        EccToggleTypeW::new(self, 16)
    }
}
#[doc = "L2 Cache Control ECC Error Injection register. Can be used to insert an ECC error into either the backing data or metadata SRAM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_inject_error::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_inject_error::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccInjectErrorSpec;
impl crate::RegisterSpec for EccInjectErrorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_inject_error::R`](R) reader structure"]
impl crate::Readable for EccInjectErrorSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_inject_error::W`](W) writer structure"]
impl crate::Writable for EccInjectErrorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ecc_inject_error to value 0"]
impl crate::Resettable for EccInjectErrorSpec {
    const RESET_VALUE: u32 = 0;
}
