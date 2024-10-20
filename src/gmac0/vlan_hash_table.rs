#[doc = "Register `vlan_hash_table` reader"]
pub type R = crate::R<VlanHashTableSpec>;
#[doc = "Register `vlan_hash_table` writer"]
pub type W = crate::W<VlanHashTableSpec>;
#[doc = "Field `vlht` reader - VLAN Hash Table"]
pub type VlhtR = crate::FieldReader<u16>;
#[doc = "Field `vlht` writer - VLAN Hash Table"]
pub type VlhtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    pub fn vlht(&self) -> VlhtR {
        VlhtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Hash Table"]
    #[inline(always)]
    #[must_use]
    pub fn vlht(&mut self) -> VlhtW<VlanHashTableSpec> {
        VlhtW::new(self, 0)
    }
}
#[doc = "MAC VLAN Hash Table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_hash_table::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_hash_table::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VlanHashTableSpec;
impl crate::RegisterSpec for VlanHashTableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan_hash_table::R`](R) reader structure"]
impl crate::Readable for VlanHashTableSpec {}
#[doc = "`write(|w| ..)` method takes [`vlan_hash_table::W`](W) writer structure"]
impl crate::Writable for VlanHashTableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets vlan_hash_table to value 0"]
impl crate::Resettable for VlanHashTableSpec {
    const RESET_VALUE: u32 = 0;
}
