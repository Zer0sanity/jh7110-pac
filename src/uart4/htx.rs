#[doc = "Register `htx` reader"]
pub type R = crate::R<HtxSpec>;
#[doc = "Register `htx` writer"]
pub type W = crate::W<HtxSpec>;
#[doc = "Field `htx` reader - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled Note, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
pub type HtxR = crate::BitReader;
#[doc = "Field `htx` writer - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled Note, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
pub type HtxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled Note, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
    #[inline(always)]
    pub fn htx(&self) -> HtxR {
        HtxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled. 0 = Halt TX disabled 1 = Halt TX enabled Note, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
    #[inline(always)]
    #[must_use]
    pub fn htx(&mut self) -> HtxW<HtxSpec> {
        HtxW::new(self, 0)
    }
}
#[doc = "Halt TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HtxSpec;
impl crate::RegisterSpec for HtxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htx::R`](R) reader structure"]
impl crate::Readable for HtxSpec {}
#[doc = "`write(|w| ..)` method takes [`htx::W`](W) writer structure"]
impl crate::Writable for HtxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets htx to value 0"]
impl crate::Resettable for HtxSpec {
    const RESET_VALUE: u32 = 0;
}
