#[doc = "Register `sys_syscfg_noc_bus_oic_qch_clock_stop[%s]` reader"]
pub type R = crate::R<SysSyscfgNocBusOicQchClockStopSpec>;
#[doc = "Register `sys_syscfg_noc_bus_oic_qch_clock_stop[%s]` writer"]
pub type W = crate::W<SysSyscfgNocBusOicQchClockStopSpec>;
#[doc = "Field `threshold` reader - "]
pub type ThresholdR = crate::FieldReader<u32>;
#[doc = "Field `threshold` writer - "]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> ThresholdW<SysSyscfgNocBusOicQchClockStopSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 60 - 92: NOC Bus OIC QCH Clock Stop Threshold registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_noc_bus_oic_qch_clock_stop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_noc_bus_oic_qch_clock_stop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfgNocBusOicQchClockStopSpec;
impl crate::RegisterSpec for SysSyscfgNocBusOicQchClockStopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg_noc_bus_oic_qch_clock_stop::R`](R) reader structure"]
impl crate::Readable for SysSyscfgNocBusOicQchClockStopSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg_noc_bus_oic_qch_clock_stop::W`](W) writer structure"]
impl crate::Writable for SysSyscfgNocBusOicQchClockStopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg_noc_bus_oic_qch_clock_stop[%s]
to value 0"]
impl crate::Resettable for SysSyscfgNocBusOicQchClockStopSpec {
    const RESET_VALUE: u32 = 0;
}
