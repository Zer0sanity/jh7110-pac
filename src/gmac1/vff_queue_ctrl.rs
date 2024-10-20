#[doc = "Register `vff_queue_ctrl` reader"]
pub type R = crate::R<VffQueueCtrlSpec>;
#[doc = "Register `vff_queue_ctrl` writer"]
pub type W = crate::W<VffQueueCtrlSpec>;
#[doc = "Field `qe` reader - VLAN Tag Filter Fail Queue Enable"]
pub type QeR = crate::BitReader;
#[doc = "Field `qe` writer - VLAN Tag Filter Fail Queue Enable"]
pub type QeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vffq` reader - VLAN Tag Filter Fail Queue"]
pub type VffqR = crate::FieldReader;
#[doc = "Field `vffq` writer - VLAN Tag Filter Fail Queue"]
pub type VffqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 16 - VLAN Tag Filter Fail Queue Enable"]
    #[inline(always)]
    pub fn qe(&self) -> QeR {
        QeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - VLAN Tag Filter Fail Queue"]
    #[inline(always)]
    pub fn vffq(&self) -> VffqR {
        VffqR::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - VLAN Tag Filter Fail Queue Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qe(&mut self) -> QeW<VffQueueCtrlSpec> {
        QeW::new(self, 16)
    }
    #[doc = "Bits 17:19 - VLAN Tag Filter Fail Queue"]
    #[inline(always)]
    #[must_use]
    pub fn vffq(&mut self) -> VffqW<VffQueueCtrlSpec> {
        VffqW::new(self, 17)
    }
}
#[doc = "MAC EQoS VLAN Tag Filter Fail Packets Queuing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vff_queue_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vff_queue_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VffQueueCtrlSpec;
impl crate::RegisterSpec for VffQueueCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vff_queue_ctrl::R`](R) reader structure"]
impl crate::Readable for VffQueueCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vff_queue_ctrl::W`](W) writer structure"]
impl crate::Writable for VffQueueCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets vff_queue_ctrl to value 0"]
impl crate::Resettable for VffQueueCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
