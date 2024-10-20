#[doc = "Register `priority[%s]` reader"]
pub type R = crate::R<PrioritySpec>;
#[doc = "Register `priority[%s]` writer"]
pub type W = crate::W<PrioritySpec>;
#[doc = "Field `priority` reader - "]
pub type PriorityR = crate::FieldReader<u32>;
#[doc = "Field `priority` writer - "]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<PrioritySpec> {
        PriorityW::new(self, 0)
    }
}
#[doc = "RISC-V PLIC Interrupt Source Priority: the priority value `0` is reserved to mean `never interrupt`, and interrupt priority increases with increasing integer values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrioritySpec;
impl crate::RegisterSpec for PrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority::R`](R) reader structure"]
impl crate::Readable for PrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`priority::W`](W) writer structure"]
impl crate::Writable for PrioritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets priority[%s]
to value 0"]
impl crate::Resettable for PrioritySpec {
    const RESET_VALUE: u32 = 0;
}
