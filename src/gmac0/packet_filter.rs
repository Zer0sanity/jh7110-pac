#[doc = "Register `packet_filter` reader"]
pub type R = crate::R<PacketFilterSpec>;
#[doc = "Register `packet_filter` writer"]
pub type W = crate::W<PacketFilterSpec>;
#[doc = "Field `pr` reader - PR"]
pub type PrR = crate::BitReader;
#[doc = "Field `pr` writer - PR"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hmc` reader - HMC"]
pub type HmcR = crate::BitReader;
#[doc = "Field `hmc` writer - HMC"]
pub type HmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pm` reader - PM"]
pub type PmR = crate::BitReader;
#[doc = "Field `pm` writer - PM"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pcf` reader - PCF"]
pub type PcfR = crate::BitReader;
#[doc = "Field `pcf` writer - PCF"]
pub type PcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hpf` reader - HPF"]
pub type HpfR = crate::BitReader;
#[doc = "Field `hpf` writer - HPF"]
pub type HpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vtfe` reader - VTFE"]
pub type VtfeR = crate::BitReader;
#[doc = "Field `vtfe` writer - VTFE"]
pub type VtfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ipfe` reader - IPFE"]
pub type IpfeR = crate::BitReader;
#[doc = "Field `ipfe` writer - IPFE"]
pub type IpfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ra` reader - RA"]
pub type RaR = crate::BitReader;
#[doc = "Field `ra` writer - RA"]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PR"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - HMC"]
    #[inline(always)]
    pub fn hmc(&self) -> HmcR {
        HmcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - PCF"]
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        PcfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - HPF"]
    #[inline(always)]
    pub fn hpf(&self) -> HpfR {
        HpfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    pub fn vtfe(&self) -> VtfeR {
        VtfeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - IPFE"]
    #[inline(always)]
    pub fn ipfe(&self) -> IpfeR {
        IpfeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PR"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<PacketFilterSpec> {
        PrW::new(self, 0)
    }
    #[doc = "Bit 2 - HMC"]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HmcW<PacketFilterSpec> {
        HmcW::new(self, 2)
    }
    #[doc = "Bit 4 - PM"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PmW<PacketFilterSpec> {
        PmW::new(self, 4)
    }
    #[doc = "Bit 7 - PCF"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PcfW<PacketFilterSpec> {
        PcfW::new(self, 7)
    }
    #[doc = "Bit 10 - HPF"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HpfW<PacketFilterSpec> {
        HpfW::new(self, 10)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    #[must_use]
    pub fn vtfe(&mut self) -> VtfeW<PacketFilterSpec> {
        VtfeW::new(self, 16)
    }
    #[doc = "Bit 20 - IPFE"]
    #[inline(always)]
    #[must_use]
    pub fn ipfe(&mut self) -> IpfeW<PacketFilterSpec> {
        IpfeW::new(self, 20)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<PacketFilterSpec> {
        RaW::new(self, 31)
    }
}
#[doc = "MAC Packet Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`packet_filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`packet_filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PacketFilterSpec;
impl crate::RegisterSpec for PacketFilterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`packet_filter::R`](R) reader structure"]
impl crate::Readable for PacketFilterSpec {}
#[doc = "`write(|w| ..)` method takes [`packet_filter::W`](W) writer structure"]
impl crate::Writable for PacketFilterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets packet_filter to value 0"]
impl crate::Resettable for PacketFilterSpec {
    const RESET_VALUE: u32 = 0;
}
