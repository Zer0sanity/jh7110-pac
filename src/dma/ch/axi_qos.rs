#[doc = "Register `axi_qos` reader"]
pub type R = crate::R<AxiQosSpec>;
#[doc = "Register `axi_qos` writer"]
pub type W = crate::W<AxiQosSpec>;
#[doc = "Field `qos(_aw,_ar)` reader - AXI QOS suffix"]
pub type QosR = crate::FieldReader;
#[doc = "Field `qos(_aw,_ar)` writer - AXI QOS suffix"]
pub type QosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "AXI QOS suffix"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `qos_aw` field"]
    #[inline(always)]
    pub fn qos(&self, n: u8) -> QosR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        QosR::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "AXI QOS suffix"]
    #[inline(always)]
    pub fn qos_iter(&self) -> impl Iterator<Item = QosR> + '_ {
        (0..2).map(move |n| QosR::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - AXI QOS suffix"]
    #[inline(always)]
    pub fn qos_aw(&self) -> QosR {
        QosR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AXI QOS suffix"]
    #[inline(always)]
    pub fn qos_ar(&self) -> QosR {
        QosR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "AXI QOS suffix"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `qos_aw` field"]
    #[inline(always)]
    #[must_use]
    pub fn qos(&mut self, n: u8) -> QosW<AxiQosSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        QosW::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - AXI QOS suffix"]
    #[inline(always)]
    #[must_use]
    pub fn qos_aw(&mut self) -> QosW<AxiQosSpec> {
        QosW::new(self, 0)
    }
    #[doc = "Bits 4:7 - AXI QOS suffix"]
    #[inline(always)]
    #[must_use]
    pub fn qos_ar(&mut self) -> QosW<AxiQosSpec> {
        QosW::new(self, 4)
    }
}
#[doc = "Channel AXI QOS register - **NOTE** this register is only allowed to be modified when the channel is disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_qos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_qos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiQosSpec;
impl crate::RegisterSpec for AxiQosSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`axi_qos::R`](R) reader structure"]
impl crate::Readable for AxiQosSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_qos::W`](W) writer structure"]
impl crate::Writable for AxiQosSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets axi_qos to value 0"]
impl crate::Resettable for AxiQosSpec {
    const RESET_VALUE: u64 = 0;
}
