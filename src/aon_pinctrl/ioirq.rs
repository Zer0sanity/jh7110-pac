#[doc = "Register `ioirq[%s]` reader"]
pub type R = crate::R<IoirqSpec>;
#[doc = "Register `ioirq[%s]` writer"]
pub type W = crate::W<IoirqSpec>;
#[doc = "Field `ioirq` reader - is - 0: Level trigger, 1: Edge trigger ic - 0: Clear the register, 1: Do not clear the register ibe - 0: Trigger on a single edge, 1: Trigger on both edges iev - 0: Negative/High, 1: Positive/Low ie - 0: Mask, 1: Unmask"]
pub type IoirqR = crate::FieldReader;
#[doc = "Field `ioirq` writer - is - 0: Level trigger, 1: Edge trigger ic - 0: Clear the register, 1: Do not clear the register ibe - 0: Trigger on a single edge, 1: Trigger on both edges iev - 0: Negative/High, 1: Positive/Low ie - 0: Mask, 1: Unmask"]
pub type IoirqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - is - 0: Level trigger, 1: Edge trigger ic - 0: Clear the register, 1: Do not clear the register ibe - 0: Trigger on a single edge, 1: Trigger on both edges iev - 0: Negative/High, 1: Positive/Low ie - 0: Mask, 1: Unmask"]
    #[inline(always)]
    pub fn ioirq(&self) -> IoirqR {
        IoirqR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - is - 0: Level trigger, 1: Edge trigger ic - 0: Clear the register, 1: Do not clear the register ibe - 0: Trigger on a single edge, 1: Trigger on both edges iev - 0: Negative/High, 1: Positive/Low ie - 0: Mask, 1: Unmask"]
    #[inline(always)]
    #[must_use]
    pub fn ioirq(&mut self) -> IoirqW<IoirqSpec> {
        IoirqW::new(self, 0)
    }
}
#[doc = "Always-on GPIO IO IRQ configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoirqSpec;
impl crate::RegisterSpec for IoirqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq::R`](R) reader structure"]
impl crate::Readable for IoirqSpec {}
#[doc = "`write(|w| ..)` method takes [`ioirq::W`](W) writer structure"]
impl crate::Writable for IoirqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ioirq[%s]
to value 0"]
impl crate::Resettable for IoirqSpec {
    const RESET_VALUE: u32 = 0;
}
