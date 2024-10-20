#[doc = "Register `features0` reader"]
pub type R = crate::R<Features0Spec>;
#[doc = "Register `features0` writer"]
pub type W = crate::W<Features0Spec>;
#[doc = "Field `miisel` reader - MII Select"]
pub type MiiselR = crate::BitReader;
#[doc = "Field `miisel` writer - MII Select"]
pub type MiiselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `gmiisel` reader - GMII Select"]
pub type GmiiselR = crate::BitReader;
#[doc = "Field `gmiisel` writer - GMII Select"]
pub type GmiiselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hdsel` reader - HD Select"]
pub type HdselR = crate::BitReader;
#[doc = "Field `hdsel` writer - HD Select"]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pcssel` reader - PCS Select"]
pub type PcsselR = crate::BitReader;
#[doc = "Field `pcssel` writer - PCS Select"]
pub type PcsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vlhash` reader - VLAN Hash"]
pub type VlhashR = crate::BitReader;
#[doc = "Field `vlhash` writer - VLAN Hash"]
pub type VlhashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `smasel` reader - SMA Select"]
pub type SmaselR = crate::BitReader;
#[doc = "Field `smasel` writer - SMA Select"]
pub type SmaselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rwksel` reader - RWK Select"]
pub type RwkselR = crate::BitReader;
#[doc = "Field `rwksel` writer - RWK Select"]
pub type RwkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mgksel` reader - MGK Select"]
pub type MgkselR = crate::BitReader;
#[doc = "Field `mgksel` writer - MGK Select"]
pub type MgkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mmcsel` reader - MMC Select"]
pub type MmcselR = crate::BitReader;
#[doc = "Field `mmcsel` writer - MMC Select"]
pub type MmcselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `arpoffsel` reader - ARP Off Select"]
pub type ArpoffselR = crate::BitReader;
#[doc = "Field `arpoffsel` writer - ARP Off Select"]
pub type ArpoffselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tssel` reader - TS Select"]
pub type TsselR = crate::BitReader;
#[doc = "Field `tssel` writer - TS Select"]
pub type TsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eeesel` reader - Energy Efficient Ethernet Select"]
pub type EeeselR = crate::BitReader;
#[doc = "Field `eeesel` writer - Energy Efficient Ethernet Select"]
pub type EeeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txcosel` reader - TX CO Select"]
pub type TxcoselR = crate::BitReader;
#[doc = "Field `txcosel` writer - TX CO Select"]
pub type TxcoselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxcoesel` reader - RX COE Select"]
pub type RxcoeselR = crate::BitReader;
#[doc = "Field `rxcoesel` writer - RX COE Select"]
pub type RxcoeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `addmac` reader - ADD MAC"]
pub type AddmacR = crate::BitReader;
#[doc = "Field `addmac` writer - ADD MAC"]
pub type AddmacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `savlanins` reader - SAVLANINS"]
pub type SavlaninsR = crate::BitReader;
#[doc = "Field `savlanins` writer - SAVLANINS"]
pub type SavlaninsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MII Select"]
    #[inline(always)]
    pub fn miisel(&self) -> MiiselR {
        MiiselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMII Select"]
    #[inline(always)]
    pub fn gmiisel(&self) -> GmiiselR {
        GmiiselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HD Select"]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCS Select"]
    #[inline(always)]
    pub fn pcssel(&self) -> PcsselR {
        PcsselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VLAN Hash"]
    #[inline(always)]
    pub fn vlhash(&self) -> VlhashR {
        VlhashR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMA Select"]
    #[inline(always)]
    pub fn smasel(&self) -> SmaselR {
        SmaselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RWK Select"]
    #[inline(always)]
    pub fn rwksel(&self) -> RwkselR {
        RwkselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MGK Select"]
    #[inline(always)]
    pub fn mgksel(&self) -> MgkselR {
        MgkselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Select"]
    #[inline(always)]
    pub fn mmcsel(&self) -> MmcselR {
        MmcselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ARP Off Select"]
    #[inline(always)]
    pub fn arpoffsel(&self) -> ArpoffselR {
        ArpoffselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - TS Select"]
    #[inline(always)]
    pub fn tssel(&self) -> TsselR {
        TsselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Energy Efficient Ethernet Select"]
    #[inline(always)]
    pub fn eeesel(&self) -> EeeselR {
        EeeselR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TX CO Select"]
    #[inline(always)]
    pub fn txcosel(&self) -> TxcoselR {
        TxcoselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - RX COE Select"]
    #[inline(always)]
    pub fn rxcoesel(&self) -> RxcoeselR {
        RxcoeselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ADD MAC"]
    #[inline(always)]
    pub fn addmac(&self) -> AddmacR {
        AddmacR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 27 - SAVLANINS"]
    #[inline(always)]
    pub fn savlanins(&self) -> SavlaninsR {
        SavlaninsR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII Select"]
    #[inline(always)]
    #[must_use]
    pub fn miisel(&mut self) -> MiiselW<Features0Spec> {
        MiiselW::new(self, 0)
    }
    #[doc = "Bit 1 - GMII Select"]
    #[inline(always)]
    #[must_use]
    pub fn gmiisel(&mut self) -> GmiiselW<Features0Spec> {
        GmiiselW::new(self, 1)
    }
    #[doc = "Bit 2 - HD Select"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HdselW<Features0Spec> {
        HdselW::new(self, 2)
    }
    #[doc = "Bit 3 - PCS Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcssel(&mut self) -> PcsselW<Features0Spec> {
        PcsselW::new(self, 3)
    }
    #[doc = "Bit 4 - VLAN Hash"]
    #[inline(always)]
    #[must_use]
    pub fn vlhash(&mut self) -> VlhashW<Features0Spec> {
        VlhashW::new(self, 4)
    }
    #[doc = "Bit 5 - SMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn smasel(&mut self) -> SmaselW<Features0Spec> {
        SmaselW::new(self, 5)
    }
    #[doc = "Bit 6 - RWK Select"]
    #[inline(always)]
    #[must_use]
    pub fn rwksel(&mut self) -> RwkselW<Features0Spec> {
        RwkselW::new(self, 6)
    }
    #[doc = "Bit 7 - MGK Select"]
    #[inline(always)]
    #[must_use]
    pub fn mgksel(&mut self) -> MgkselW<Features0Spec> {
        MgkselW::new(self, 7)
    }
    #[doc = "Bit 8 - MMC Select"]
    #[inline(always)]
    #[must_use]
    pub fn mmcsel(&mut self) -> MmcselW<Features0Spec> {
        MmcselW::new(self, 8)
    }
    #[doc = "Bit 9 - ARP Off Select"]
    #[inline(always)]
    #[must_use]
    pub fn arpoffsel(&mut self) -> ArpoffselW<Features0Spec> {
        ArpoffselW::new(self, 9)
    }
    #[doc = "Bit 12 - TS Select"]
    #[inline(always)]
    #[must_use]
    pub fn tssel(&mut self) -> TsselW<Features0Spec> {
        TsselW::new(self, 12)
    }
    #[doc = "Bit 13 - Energy Efficient Ethernet Select"]
    #[inline(always)]
    #[must_use]
    pub fn eeesel(&mut self) -> EeeselW<Features0Spec> {
        EeeselW::new(self, 13)
    }
    #[doc = "Bit 14 - TX CO Select"]
    #[inline(always)]
    #[must_use]
    pub fn txcosel(&mut self) -> TxcoselW<Features0Spec> {
        TxcoselW::new(self, 14)
    }
    #[doc = "Bit 16 - RX COE Select"]
    #[inline(always)]
    #[must_use]
    pub fn rxcoesel(&mut self) -> RxcoeselW<Features0Spec> {
        RxcoeselW::new(self, 16)
    }
    #[doc = "Bit 18 - ADD MAC"]
    #[inline(always)]
    #[must_use]
    pub fn addmac(&mut self) -> AddmacW<Features0Spec> {
        AddmacW::new(self, 18)
    }
    #[doc = "Bit 27 - SAVLANINS"]
    #[inline(always)]
    #[must_use]
    pub fn savlanins(&mut self) -> SavlaninsW<Features0Spec> {
        SavlaninsW::new(self, 27)
    }
}
#[doc = "Hardware Features 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`features0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`features0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Features0Spec;
impl crate::RegisterSpec for Features0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`features0::R`](R) reader structure"]
impl crate::Readable for Features0Spec {}
#[doc = "`write(|w| ..)` method takes [`features0::W`](W) writer structure"]
impl crate::Writable for Features0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets features0 to value 0"]
impl crate::Resettable for Features0Spec {
    const RESET_VALUE: u32 = 0;
}
