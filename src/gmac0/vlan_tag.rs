#[doc = "Register `vlan_tag` reader"]
pub type R = crate::R<VlanTagSpec>;
#[doc = "Register `vlan_tag` writer"]
pub type W = crate::W<VlanTagSpec>;
#[doc = "Field `ob` reader - VLAN OB"]
pub type ObR = crate::BitReader;
#[doc = "Field `ob` writer - VLAN OB"]
pub type ObW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vid` reader - VLAN Tag VID"]
pub type VidR = crate::FieldReader<u16>;
#[doc = "Field `ct` reader - VLAN CT"]
pub type CtR = crate::BitReader;
#[doc = "Field `ct` writer - VLAN CT"]
pub type CtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ofs` reader - VLAN OFS"]
pub type OfsR = crate::FieldReader;
#[doc = "Field `ofs` writer - VLAN OFS"]
pub type OfsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `etv` reader - VLAN Tag ETV"]
pub type EtvR = crate::BitReader;
#[doc = "Field `etv` writer - VLAN Tag ETV"]
pub type EtvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dovltc` reader - VLAN Tag DOVLTC"]
pub type DovltcR = crate::BitReader;
#[doc = "Field `dovltc` writer - VLAN Tag DOVLTC"]
pub type DovltcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VLAN EVLS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evls {
    #[doc = "0: Strip none"]
    StripNone = 0,
    #[doc = "1: Strip pass"]
    StripPass = 1,
    #[doc = "2: Strip fail"]
    StripFail = 2,
    #[doc = "3: Strip all"]
    StripAll = 3,
}
impl From<Evls> for u8 {
    #[inline(always)]
    fn from(variant: Evls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evls {
    type Ux = u8;
}
#[doc = "Field `evls` reader - VLAN EVLS"]
pub type EvlsR = crate::FieldReader<Evls>;
impl EvlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evls {
        match self.bits {
            0 => Evls::StripNone,
            1 => Evls::StripPass,
            2 => Evls::StripFail,
            3 => Evls::StripAll,
            _ => unreachable!(),
        }
    }
    #[doc = "Strip none"]
    #[inline(always)]
    pub fn is_strip_none(&self) -> bool {
        *self == Evls::StripNone
    }
    #[doc = "Strip pass"]
    #[inline(always)]
    pub fn is_strip_pass(&self) -> bool {
        *self == Evls::StripPass
    }
    #[doc = "Strip fail"]
    #[inline(always)]
    pub fn is_strip_fail(&self) -> bool {
        *self == Evls::StripFail
    }
    #[doc = "Strip all"]
    #[inline(always)]
    pub fn is_strip_all(&self) -> bool {
        *self == Evls::StripAll
    }
}
#[doc = "Field `evls` writer - VLAN EVLS"]
pub type EvlsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Evls>;
impl<'a, REG> EvlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Strip none"]
    #[inline(always)]
    pub fn strip_none(self) -> &'a mut crate::W<REG> {
        self.variant(Evls::StripNone)
    }
    #[doc = "Strip pass"]
    #[inline(always)]
    pub fn strip_pass(self) -> &'a mut crate::W<REG> {
        self.variant(Evls::StripPass)
    }
    #[doc = "Strip fail"]
    #[inline(always)]
    pub fn strip_fail(self) -> &'a mut crate::W<REG> {
        self.variant(Evls::StripFail)
    }
    #[doc = "Strip all"]
    #[inline(always)]
    pub fn strip_all(self) -> &'a mut crate::W<REG> {
        self.variant(Evls::StripAll)
    }
}
#[doc = "Field `evlrxs` reader - VLAN EVLRXS"]
pub type EvlrxsR = crate::BitReader;
#[doc = "Field `evlrxs` writer - VLAN EVLRXS"]
pub type EvlrxsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vthm` reader - VLAN VTHM"]
pub type VthmR = crate::BitReader;
#[doc = "Field `vthm` writer - VLAN VTHM"]
pub type VthmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `edvlp` reader - VLAN EDVLP"]
pub type EdvlpR = crate::BitReader;
#[doc = "Field `edvlp` writer - VLAN EDVLP"]
pub type EdvlpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VLAN OB"]
    #[inline(always)]
    pub fn ob(&self) -> ObR {
        ObR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:15 - VLAN Tag VID"]
    #[inline(always)]
    pub fn vid(&self) -> VidR {
        VidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 1 - VLAN CT"]
    #[inline(always)]
    pub fn ct(&self) -> CtR {
        CtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - VLAN OFS"]
    #[inline(always)]
    pub fn ofs(&self) -> OfsR {
        OfsR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - VLAN Tag ETV"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - VLAN Tag DOVLTC"]
    #[inline(always)]
    pub fn dovltc(&self) -> DovltcR {
        DovltcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - VLAN EVLS"]
    #[inline(always)]
    pub fn evls(&self) -> EvlsR {
        EvlsR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - VLAN EVLRXS"]
    #[inline(always)]
    pub fn evlrxs(&self) -> EvlrxsR {
        EvlrxsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VLAN VTHM"]
    #[inline(always)]
    pub fn vthm(&self) -> VthmR {
        VthmR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - VLAN EDVLP"]
    #[inline(always)]
    pub fn edvlp(&self) -> EdvlpR {
        EdvlpR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VLAN OB"]
    #[inline(always)]
    #[must_use]
    pub fn ob(&mut self) -> ObW<VlanTagSpec> {
        ObW::new(self, 0)
    }
    #[doc = "Bit 1 - VLAN CT"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CtW<VlanTagSpec> {
        CtW::new(self, 1)
    }
    #[doc = "Bits 2:6 - VLAN OFS"]
    #[inline(always)]
    #[must_use]
    pub fn ofs(&mut self) -> OfsW<VlanTagSpec> {
        OfsW::new(self, 2)
    }
    #[doc = "Bit 16 - VLAN Tag ETV"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> EtvW<VlanTagSpec> {
        EtvW::new(self, 16)
    }
    #[doc = "Bit 20 - VLAN Tag DOVLTC"]
    #[inline(always)]
    #[must_use]
    pub fn dovltc(&mut self) -> DovltcW<VlanTagSpec> {
        DovltcW::new(self, 20)
    }
    #[doc = "Bits 21:22 - VLAN EVLS"]
    #[inline(always)]
    #[must_use]
    pub fn evls(&mut self) -> EvlsW<VlanTagSpec> {
        EvlsW::new(self, 21)
    }
    #[doc = "Bit 24 - VLAN EVLRXS"]
    #[inline(always)]
    #[must_use]
    pub fn evlrxs(&mut self) -> EvlrxsW<VlanTagSpec> {
        EvlrxsW::new(self, 24)
    }
    #[doc = "Bit 25 - VLAN VTHM"]
    #[inline(always)]
    #[must_use]
    pub fn vthm(&mut self) -> VthmW<VlanTagSpec> {
        VthmW::new(self, 25)
    }
    #[doc = "Bit 26 - VLAN EDVLP"]
    #[inline(always)]
    #[must_use]
    pub fn edvlp(&mut self) -> EdvlpW<VlanTagSpec> {
        EdvlpW::new(self, 26)
    }
}
#[doc = "MAC VLAN Tag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_tag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_tag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VlanTagSpec;
impl crate::RegisterSpec for VlanTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vlan_tag::R`](R) reader structure"]
impl crate::Readable for VlanTagSpec {}
#[doc = "`write(|w| ..)` method takes [`vlan_tag::W`](W) writer structure"]
impl crate::Writable for VlanTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets vlan_tag to value 0"]
impl crate::Resettable for VlanTagSpec {
    const RESET_VALUE: u32 = 0;
}
