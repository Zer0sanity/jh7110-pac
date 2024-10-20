#[doc = "Register `syscfg1` reader"]
pub type R = crate::R<Syscfg1Spec>;
#[doc = "Field `mposv(0-31)` reader - MPOSV: u0_mipitx_dphy_MPOSVn"]
pub type MposvR = crate::BitReader;
impl R {
    #[doc = "MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `mposv0` field"]
    #[inline(always)]
    pub fn mposv(&self, n: u8) -> MposvR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        MposvR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv_iter(&self) -> impl Iterator<Item = MposvR> + '_ {
        (0..32).map(move |n| MposvR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv0(&self) -> MposvR {
        MposvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv1(&self) -> MposvR {
        MposvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv2(&self) -> MposvR {
        MposvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv3(&self) -> MposvR {
        MposvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv4(&self) -> MposvR {
        MposvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv5(&self) -> MposvR {
        MposvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv6(&self) -> MposvR {
        MposvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv7(&self) -> MposvR {
        MposvR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv8(&self) -> MposvR {
        MposvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv9(&self) -> MposvR {
        MposvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv10(&self) -> MposvR {
        MposvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv11(&self) -> MposvR {
        MposvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv12(&self) -> MposvR {
        MposvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv13(&self) -> MposvR {
        MposvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv14(&self) -> MposvR {
        MposvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv15(&self) -> MposvR {
        MposvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv16(&self) -> MposvR {
        MposvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv17(&self) -> MposvR {
        MposvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv18(&self) -> MposvR {
        MposvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv19(&self) -> MposvR {
        MposvR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv20(&self) -> MposvR {
        MposvR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv21(&self) -> MposvR {
        MposvR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv22(&self) -> MposvR {
        MposvR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv23(&self) -> MposvR {
        MposvR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv24(&self) -> MposvR {
        MposvR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv25(&self) -> MposvR {
        MposvR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv26(&self) -> MposvR {
        MposvR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv27(&self) -> MposvR {
        MposvR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv28(&self) -> MposvR {
        MposvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv29(&self) -> MposvR {
        MposvR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv30(&self) -> MposvR {
        MposvR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MPOSV: u0_mipitx_dphy_MPOSVn"]
    #[inline(always)]
    pub fn mposv31(&self) -> MposvR {
        MposvR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "MIPITX DPHY SYSCFG 1: mipitx_apbifsaif_syscfg_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg1Spec;
impl crate::RegisterSpec for Syscfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg1::R`](R) reader structure"]
impl crate::Readable for Syscfg1Spec {}
#[doc = "`reset()` method sets syscfg1 to value 0"]
impl crate::Resettable for Syscfg1Spec {
    const RESET_VALUE: u32 = 0;
}
