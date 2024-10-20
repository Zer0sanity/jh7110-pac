#[doc = "Register `llp` reader"]
pub type R = crate::R<LlpSpec>;
#[doc = "Register `llp` writer"]
pub type W = crate::W<LlpSpec>;
#[doc = "Field `lms` reader - LLI master select - 0: AXI Master 1, 1: AXI Master 2."]
pub type LmsR = crate::BitReader;
#[doc = "Field `lms` writer - LLI master select - 0: AXI Master 1, 1: AXI Master 2."]
pub type LmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `loc` reader - Starting address memory of LLI block - **NOTE** the lower six bits are unassigned because addresses are assumed to be 64-byte aligned."]
pub type LocR = crate::FieldReader<u64>;
#[doc = "Field `loc` writer - Starting address memory of LLI block - **NOTE** the lower six bits are unassigned because addresses are assumed to be 64-byte aligned."]
pub type LocW<'a, REG> = crate::FieldWriter<'a, REG, 58, u64>;
impl R {
    #[doc = "Bit 0 - LLI master select - 0: AXI Master 1, 1: AXI Master 2."]
    #[inline(always)]
    pub fn lms(&self) -> LmsR {
        LmsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:63 - Starting address memory of LLI block - **NOTE** the lower six bits are unassigned because addresses are assumed to be 64-byte aligned."]
    #[inline(always)]
    pub fn loc(&self) -> LocR {
        LocR::new((self.bits >> 6) & 0x03ff_ffff_ffff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - LLI master select - 0: AXI Master 1, 1: AXI Master 2."]
    #[inline(always)]
    #[must_use]
    pub fn lms(&mut self) -> LmsW<LlpSpec> {
        LmsW::new(self, 0)
    }
    #[doc = "Bits 6:63 - Starting address memory of LLI block - **NOTE** the lower six bits are unassigned because addresses are assumed to be 64-byte aligned."]
    #[inline(always)]
    #[must_use]
    pub fn loc(&mut self) -> LocW<LlpSpec> {
        LocW::new(self, 6)
    }
}
#[doc = "Linked list pointer register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LlpSpec;
impl crate::RegisterSpec for LlpSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`llp::R`](R) reader structure"]
impl crate::Readable for LlpSpec {}
#[doc = "`write(|w| ..)` method takes [`llp::W`](W) writer structure"]
impl crate::Writable for LlpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets llp to value 0"]
impl crate::Resettable for LlpSpec {
    const RESET_VALUE: u64 = 0;
}
