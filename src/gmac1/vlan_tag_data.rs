#[doc = "Register `vlan_tag_data` reader"]
pub type R = crate::R<VlanTagDataSpec>;
#[doc = "Register `vlan_tag_data` writer"]
pub type W = crate::W<VlanTagDataSpec>;
#[doc = "Field `vid` reader - VLAN Tag Data VID"]
pub type VidR = crate::FieldReader<u16>;
#[doc = "Field `ven` reader - VLAN Tag Data Enable"]
pub type VenR = crate::BitReader;
#[doc = "Field `ven` writer - VLAN Tag Data Enable"]
pub type VenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `etv` reader - VLAN Tag Data ETV"]
pub type EtvR = crate::BitReader;
#[doc = "Field `etv` writer - VLAN Tag Data ETV"]
pub type EtvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Data VID"]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - VLAN Tag Data Enable"]
    #[inline(always)]
    pub fn ven(&self) -> VenR {
        VenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VLAN Tag Data ETV"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - VLAN Tag Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ven(&mut self) -> VenW<VlanTagDataSpec> {
        VenW::new(self, 16)
    }
    #[doc = "Bit 17 - VLAN Tag Data ETV"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> EtvW<VlanTagDataSpec> {
        EtvW::new(self, 17)
    }
}
#[doc = "MAC VLAN Tag Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_tag_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_tag_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VlanTagDataSpec;
impl crate::RegisterSpec for VlanTagDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan_tag_data::R`](R) reader structure"]
impl crate::Readable for VlanTagDataSpec {}
#[doc = "`write(|w| ..)` method takes [`vlan_tag_data::W`](W) writer structure"]
impl crate::Writable for VlanTagDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets vlan_tag_data to value 0"]
impl crate::Resettable for VlanTagDataSpec {
    const RESET_VALUE: u32 = 0;
}
