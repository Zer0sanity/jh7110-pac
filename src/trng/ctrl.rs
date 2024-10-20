#[doc = "Register `ctrl` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `ctrl` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `exec_nop` reader - Execute a NOP instruction"]
pub type ExecNopR = crate::BitReader;
#[doc = "Field `exec_nop` writer - Execute a NOP instruction"]
pub type ExecNopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gen_rand` reader - Generate a random number"]
pub type GenRandR = crate::BitReader;
#[doc = "Field `gen_rand` writer - Generate a random number"]
pub type GenRandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reseed` reader - Reseed the TRNG from noise sources"]
pub type ReseedR = crate::BitReader;
#[doc = "Field `reseed` writer - Reseed the TRNG from noise sources"]
pub type ReseedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Execute a NOP instruction"]
    #[inline(always)]
    pub fn exec_nop(&self) -> ExecNopR {
        ExecNopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate a random number"]
    #[inline(always)]
    pub fn gen_rand(&self) -> GenRandR {
        GenRandR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reseed the TRNG from noise sources"]
    #[inline(always)]
    pub fn reseed(&self) -> ReseedR {
        ReseedR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Execute a NOP instruction"]
    #[inline(always)]
    #[must_use]
    pub fn exec_nop(&mut self) -> ExecNopW<CtrlSpec> {
        ExecNopW::new(self, 0)
    }
    #[doc = "Bit 1 - Generate a random number"]
    #[inline(always)]
    #[must_use]
    pub fn gen_rand(&mut self) -> GenRandW<CtrlSpec> {
        GenRandW::new(self, 1)
    }
    #[doc = "Bit 2 - Reseed the TRNG from noise sources"]
    #[inline(always)]
    #[must_use]
    pub fn reseed(&mut self) -> ReseedW<CtrlSpec> {
        ReseedW::new(self, 2)
    }
}
#[doc = "TRNG Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
