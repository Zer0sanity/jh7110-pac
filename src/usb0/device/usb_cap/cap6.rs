#[doc = "Register `cap6` reader"]
pub type R = crate::R<Cap6Spec>;
#[doc = "USBSS-DEV Controller Internal build number.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DevBaseVersion {
    #[doc = "148738: NXP version 1"]
    NxpV1 = 148738,
    #[doc = "148745: TI version 1"]
    TiV1 = 148745,
    #[doc = "148748: Version 2"]
    V2 = 148748,
    #[doc = "148749: Version 3"]
    V3 = 148749,
}
impl From<DevBaseVersion> for u32 {
    #[inline(always)]
    fn from(variant: DevBaseVersion) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DevBaseVersion {
    type Ux = u32;
}
#[doc = "Field `dev_base_version` reader - USBSS-DEV Controller Internal build number."]
pub type DevBaseVersionR = crate::FieldReader<DevBaseVersion>;
impl DevBaseVersionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DevBaseVersion> {
        match self.bits {
            148738 => Some(DevBaseVersion::NxpV1),
            148745 => Some(DevBaseVersion::TiV1),
            148748 => Some(DevBaseVersion::V2),
            148749 => Some(DevBaseVersion::V3),
            _ => None,
        }
    }
    #[doc = "NXP version 1"]
    #[inline(always)]
    pub fn is_nxp_v1(&self) -> bool {
        *self == DevBaseVersion::NxpV1
    }
    #[doc = "TI version 1"]
    #[inline(always)]
    pub fn is_ti_v1(&self) -> bool {
        *self == DevBaseVersion::TiV1
    }
    #[doc = "Version 2"]
    #[inline(always)]
    pub fn is_v2(&self) -> bool {
        *self == DevBaseVersion::V2
    }
    #[doc = "Version 3"]
    #[inline(always)]
    pub fn is_v3(&self) -> bool {
        *self == DevBaseVersion::V3
    }
}
#[doc = "Field `dev_custom_version` reader - USBSS-DEV Controller version number."]
pub type DevCustomVersionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - USBSS-DEV Controller Internal build number."]
    #[inline(always)]
    pub fn dev_base_version(&self) -> DevBaseVersionR {
        DevBaseVersionR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - USBSS-DEV Controller version number."]
    #[inline(always)]
    pub fn dev_custom_version(&self) -> DevCustomVersionR {
        DevCustomVersionR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "USB3 Global capability 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap6Spec;
impl crate::RegisterSpec for Cap6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap6::R`](R) reader structure"]
impl crate::Readable for Cap6Spec {}
#[doc = "`reset()` method sets cap6 to value 0"]
impl crate::Resettable for Cap6Spec {
    const RESET_VALUE: u32 = 0;
}
