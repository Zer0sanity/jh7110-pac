#[doc = "Register `osc` reader"]
pub type R = crate::R<OscSpec>;
#[doc = "Register `osc` writer"]
pub type W = crate::W<OscSpec>;
#[doc = "Field `ds` reader - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
pub type DsR = crate::FieldReader;
#[doc = "Field `ds` writer - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Drive Strength (DS): * 00: The rated drive strength is 2 mA. * 01: The rated drive strength is 4 mA. * 10: The rated drive strength is 8 mA. * 11: The rated drive strength is 12 mA."]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DsW<OscSpec> {
        DsW::new(self, 0)
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG 76 - OSC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OscSpec;
impl crate::RegisterSpec for OscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc::R`](R) reader structure"]
impl crate::Readable for OscSpec {}
#[doc = "`write(|w| ..)` method takes [`osc::W`](W) writer structure"]
impl crate::Writable for OscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets osc to value 0x02"]
impl crate::Resettable for OscSpec {
    const RESET_VALUE: u32 = 0x02;
}
