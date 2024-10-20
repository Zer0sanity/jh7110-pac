#[doc = "Register `phyif_ctrl_status` reader"]
pub type R = crate::R<PhyifCtrlStatusSpec>;
#[doc = "Register `phyif_ctrl_status` writer"]
pub type W = crate::W<PhyifCtrlStatusSpec>;
#[doc = "Field `tc` reader - PHY TC"]
pub type TcR = crate::BitReader;
#[doc = "Field `tc` writer - PHY TC"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lud` reader - PHY LUD"]
pub type LudR = crate::BitReader;
#[doc = "Field `lud` writer - PHY LUD"]
pub type LudW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `smidrxs` reader - PHY SMID RXS"]
pub type SmidrxsR = crate::BitReader;
#[doc = "Field `smidrxs` writer - PHY SMID RXS"]
pub type SmidrxsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lnkmod` reader - PHY Link Mode"]
pub type LnkmodR = crate::BitReader;
#[doc = "Field `lnkmod` writer - PHY Link Mode"]
pub type LnkmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PHY Link Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speed {
    #[doc = "0: PHY Link Speed 2.5"]
    Speed2_5 = 0,
    #[doc = "1: PHY Link Speed 25"]
    Speed25 = 1,
    #[doc = "2: PHY Link Speed 125"]
    Speed125 = 2,
}
impl From<Speed> for u8 {
    #[inline(always)]
    fn from(variant: Speed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speed {
    type Ux = u8;
}
#[doc = "Field `speed` reader - PHY Link Speed"]
pub type SpeedR = crate::FieldReader<Speed>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speed> {
        match self.bits {
            0 => Some(Speed::Speed2_5),
            1 => Some(Speed::Speed25),
            2 => Some(Speed::Speed125),
            _ => None,
        }
    }
    #[doc = "PHY Link Speed 2.5"]
    #[inline(always)]
    pub fn is_speed2_5(&self) -> bool {
        *self == Speed::Speed2_5
    }
    #[doc = "PHY Link Speed 25"]
    #[inline(always)]
    pub fn is_speed25(&self) -> bool {
        *self == Speed::Speed25
    }
    #[doc = "PHY Link Speed 125"]
    #[inline(always)]
    pub fn is_speed125(&self) -> bool {
        *self == Speed::Speed125
    }
}
#[doc = "Field `speed` writer - PHY Link Speed"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Speed>;
impl<'a, REG> SpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PHY Link Speed 2.5"]
    #[inline(always)]
    pub fn speed2_5(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Speed2_5)
    }
    #[doc = "PHY Link Speed 25"]
    #[inline(always)]
    pub fn speed25(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Speed25)
    }
    #[doc = "PHY Link Speed 125"]
    #[inline(always)]
    pub fn speed125(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Speed125)
    }
}
#[doc = "Field `lnksts` reader - PHY Link Status"]
pub type LnkstsR = crate::BitReader;
#[doc = "Field `lnksts` writer - PHY Link Status"]
pub type LnkstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `jabto` reader - PHY Jabber TO"]
pub type JabtoR = crate::BitReader;
#[doc = "Field `jabto` writer - PHY Jabber TO"]
pub type JabtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `falsecardet` reader - PHY False CARDET"]
pub type FalsecardetR = crate::BitReader;
#[doc = "Field `falsecardet` writer - PHY False CARDET"]
pub type FalsecardetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PHY TC"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PHY LUD"]
    #[inline(always)]
    pub fn lud(&self) -> LudR {
        LudR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY SMID RXS"]
    #[inline(always)]
    pub fn smidrxs(&self) -> SmidrxsR {
        SmidrxsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - PHY Link Mode"]
    #[inline(always)]
    pub fn lnkmod(&self) -> LnkmodR {
        LnkmodR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - PHY Link Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - PHY Link Status"]
    #[inline(always)]
    pub fn lnksts(&self) -> LnkstsR {
        LnkstsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PHY Jabber TO"]
    #[inline(always)]
    pub fn jabto(&self) -> JabtoR {
        JabtoR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PHY False CARDET"]
    #[inline(always)]
    pub fn falsecardet(&self) -> FalsecardetR {
        FalsecardetR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PHY TC"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<PhyifCtrlStatusSpec> {
        TcW::new(self, 0)
    }
    #[doc = "Bit 1 - PHY LUD"]
    #[inline(always)]
    #[must_use]
    pub fn lud(&mut self) -> LudW<PhyifCtrlStatusSpec> {
        LudW::new(self, 1)
    }
    #[doc = "Bit 4 - PHY SMID RXS"]
    #[inline(always)]
    #[must_use]
    pub fn smidrxs(&mut self) -> SmidrxsW<PhyifCtrlStatusSpec> {
        SmidrxsW::new(self, 4)
    }
    #[doc = "Bit 16 - PHY Link Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lnkmod(&mut self) -> LnkmodW<PhyifCtrlStatusSpec> {
        LnkmodW::new(self, 16)
    }
    #[doc = "Bits 17:18 - PHY Link Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<PhyifCtrlStatusSpec> {
        SpeedW::new(self, 17)
    }
    #[doc = "Bit 19 - PHY Link Status"]
    #[inline(always)]
    #[must_use]
    pub fn lnksts(&mut self) -> LnkstsW<PhyifCtrlStatusSpec> {
        LnkstsW::new(self, 19)
    }
    #[doc = "Bit 20 - PHY Jabber TO"]
    #[inline(always)]
    #[must_use]
    pub fn jabto(&mut self) -> JabtoW<PhyifCtrlStatusSpec> {
        JabtoW::new(self, 20)
    }
    #[doc = "Bit 21 - PHY False CARDET"]
    #[inline(always)]
    #[must_use]
    pub fn falsecardet(&mut self) -> FalsecardetW<PhyifCtrlStatusSpec> {
        FalsecardetW::new(self, 21)
    }
}
#[doc = "PHY Interface Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyif_ctrl_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phyif_ctrl_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyifCtrlStatusSpec;
impl crate::RegisterSpec for PhyifCtrlStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phyif_ctrl_status::R`](R) reader structure"]
impl crate::Readable for PhyifCtrlStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`phyif_ctrl_status::W`](W) writer structure"]
impl crate::Writable for PhyifCtrlStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets phyif_ctrl_status to value 0"]
impl crate::Resettable for PhyifCtrlStatusSpec {
    const RESET_VALUE: u32 = 0;
}
