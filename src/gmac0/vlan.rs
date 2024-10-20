#[doc = "Register `vlan` reader"]
pub type R = crate::R<VlanSpec>;
#[doc = "Register `vlan` writer"]
pub type W = crate::W<VlanSpec>;
#[doc = "Field `vlht` reader - VLAN Hash Table ID"]
pub type VlhtR = crate::FieldReader<u16>;
#[doc = "Field `vlht` writer - VLAN Hash Table ID"]
pub type VlhtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `vlc` reader - VLAN VLC"]
pub type VlcR = crate::FieldReader;
#[doc = "Field `vlc` writer - VLAN VLC"]
pub type VlcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `csvl` reader - VLAN CSVL"]
pub type CsvlR = crate::BitReader;
#[doc = "Field `csvl` writer - VLAN CSVL"]
pub type CsvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vlti` reader - VLAN VLTI"]
pub type VltiR = crate::BitReader;
#[doc = "Field `vlti` writer - VLAN VLTI"]
pub type VltiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Hash Table ID"]
    #[inline(always)]
    pub fn vlht(&self) -> VlhtR {
        VlhtR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLAN VLC"]
    #[inline(always)]
    pub fn vlc(&self) -> VlcR {
        VlcR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - VLAN CSVL"]
    #[inline(always)]
    pub fn csvl(&self) -> CsvlR {
        CsvlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VLAN VLTI"]
    #[inline(always)]
    pub fn vlti(&self) -> VltiR {
        VltiR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Hash Table ID"]
    #[inline(always)]
    #[must_use]
    pub fn vlht(&mut self) -> VlhtW<VlanSpec> {
        VlhtW::new(self, 0)
    }
    #[doc = "Bits 16:17 - VLAN VLC"]
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VlcW<VlanSpec> {
        VlcW::new(self, 16)
    }
    #[doc = "Bit 19 - VLAN CSVL"]
    #[inline(always)]
    #[must_use]
    pub fn csvl(&mut self) -> CsvlW<VlanSpec> {
        CsvlW::new(self, 19)
    }
    #[doc = "Bit 20 - VLAN VLTI"]
    #[inline(always)]
    #[must_use]
    pub fn vlti(&mut self) -> VltiW<VlanSpec> {
        VltiW::new(self, 20)
    }
}
#[doc = "MAC VLAN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VlanSpec;
impl crate::RegisterSpec for VlanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan::R`](R) reader structure"]
impl crate::Readable for VlanSpec {}
#[doc = "`write(|w| ..)` method takes [`vlan::W`](W) writer structure"]
impl crate::Writable for VlanSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets vlan to value 0"]
impl crate::Resettable for VlanSpec {
    const RESET_VALUE: u32 = 0;
}
