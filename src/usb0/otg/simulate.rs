#[doc = "Register `simulate` reader"]
pub type R = crate::R<SimulateSpec>;
#[doc = "Register `simulate` writer"]
pub type W = crate::W<SimulateSpec>;
#[doc = "Field `power_lost` reader - USB3 OTG simulation - indicates if power was lost before."]
pub type PowerLostR = crate::BitReader;
#[doc = "Field `power_lost` writer - USB3 OTG simulation - indicates if power was lost before."]
pub type PowerLostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB3 OTG simulation - indicates if power was lost before."]
    #[inline(always)]
    pub fn power_lost(&self) -> PowerLostR {
        PowerLostR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB3 OTG simulation - indicates if power was lost before."]
    #[inline(always)]
    #[must_use]
    pub fn power_lost(&mut self) -> PowerLostW<SimulateSpec> {
        PowerLostW::new(self, 0)
    }
}
#[doc = "USB3 OTG simulate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simulate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simulate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SimulateSpec;
impl crate::RegisterSpec for SimulateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simulate::R`](R) reader structure"]
impl crate::Readable for SimulateSpec {}
#[doc = "`write(|w| ..)` method takes [`simulate::W`](W) writer structure"]
impl crate::Writable for SimulateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets simulate to value 0"]
impl crate::Resettable for SimulateSpec {
    const RESET_VALUE: u32 = 0;
}
