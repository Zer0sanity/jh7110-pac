#[doc = "Register `itop3` reader"]
pub type R = crate::R<Itop3Spec>;
#[doc = "Register `itop3` writer"]
pub type W = crate::W<Itop3Spec>;
#[doc = "Field `tc` reader - You can set the DMACINTTC interrupt request to a certain value in test mode by writing to the register. A read returns the value on the output, after the test multiplexor."]
pub type TcR = crate::BitReader;
#[doc = "Field `tc` writer - You can set the DMACINTTC interrupt request to a certain value in test mode by writing to the register. A read returns the value on the output, after the test multiplexor."]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `err` reader - You can set the DMACINTERR interrupt request to a certain value in test mode by writing to the register. A read returns the value on the output, after the test multiplexor."]
pub type ErrR = crate::BitReader;
#[doc = "Field `err` writer - You can set the DMACINTERR interrupt request to a certain value in test mode by writing to the register. A read returns the value on the output, after the test multiplexor."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - You can set the DMACINTTC interrupt request to a certain value in test mode by writing to the register. A read returns the value on the output, after the test multiplexor."]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - You can set the DMACINTERR interrupt request to a certain value in test mode by writing to the register. A read returns the value on the output, after the test multiplexor."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - You can set the DMACINTTC interrupt request to a certain value in test mode by writing to the register. A read returns the value on the output, after the test multiplexor."]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<Itop3Spec> {
        TcW::new(self, 0)
    }
    #[doc = "Bit 1 - You can set the DMACINTERR interrupt request to a certain value in test mode by writing to the register. A read returns the value on the output, after the test multiplexor."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<Itop3Spec> {
        ErrW::new(self, 1)
    }
}
#[doc = "DMA Integration Test Output 3 register - controls and reads the interrupt request output lines in test mode. The DMACINTR interrupt request signal combines both interrupt requests, DMACINTTC and DMACINTERR, into one interrupt request signal. Therefore, if you set either the TC or E bits, then DMACINTR is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itop3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itop3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itop3Spec;
impl crate::RegisterSpec for Itop3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itop3::R`](R) reader structure"]
impl crate::Readable for Itop3Spec {}
#[doc = "`write(|w| ..)` method takes [`itop3::W`](W) writer structure"]
impl crate::Writable for Itop3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets itop3 to value 0"]
impl crate::Resettable for Itop3Spec {
    const RESET_VALUE: u32 = 0;
}
