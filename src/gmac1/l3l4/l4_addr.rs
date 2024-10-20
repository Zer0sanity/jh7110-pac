#[doc = "Register `l4_addr` reader"]
pub type R = crate::R<L4AddrSpec>;
#[doc = "Register `l4_addr` writer"]
pub type W = crate::W<L4AddrSpec>;
#[doc = "Field `sp` reader - L4 Filter Address SP"]
pub type SpR = crate::FieldReader<u16>;
#[doc = "Field `sp` writer - L4 Filter Address SP"]
pub type SpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `dp` reader - L4 Filter Address DP"]
pub type DpR = crate::FieldReader<u16>;
#[doc = "Field `dp` writer - L4 Filter Address DP"]
pub type DpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - L4 Filter Address SP"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4 Filter Address DP"]
    #[inline(always)]
    pub fn dp(&self) -> DpR {
        DpR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4 Filter Address SP"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SpW<L4AddrSpec> {
        SpW::new(self, 0)
    }
    #[doc = "Bits 16:31 - L4 Filter Address DP"]
    #[inline(always)]
    #[must_use]
    pub fn dp(&mut self) -> DpW<L4AddrSpec> {
        DpW::new(self, 16)
    }
}
#[doc = "L4 Filter Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l4_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l4_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L4AddrSpec;
impl crate::RegisterSpec for L4AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l4_addr::R`](R) reader structure"]
impl crate::Readable for L4AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`l4_addr::W`](W) writer structure"]
impl crate::Writable for L4AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets l4_addr to value 0"]
impl crate::Resettable for L4AddrSpec {
    const RESET_VALUE: u32 = 0;
}
