#[doc = "Register `l3_addr[%s]` reader"]
pub type R = crate::R<L3AddrSpec>;
#[doc = "Register `l3_addr[%s]` writer"]
pub type W = crate::W<L3AddrSpec>;
#[doc = "Field `addr` reader - L3 Filter Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `addr` writer - L3 Filter Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3 Filter Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3 Filter Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<L3AddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "L3 Filter Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l3_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l3_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L3AddrSpec;
impl crate::RegisterSpec for L3AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l3_addr::R`](R) reader structure"]
impl crate::Readable for L3AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`l3_addr::W`](W) writer structure"]
impl crate::Writable for L3AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets l3_addr[%s]
to value 0"]
impl crate::Resettable for L3AddrSpec {
    const RESET_VALUE: u32 = 0;
}
