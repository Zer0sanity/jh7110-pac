#[repr(C)]
#[doc = "SYSCRG RESET registers"]
#[doc(alias = "rst")]
pub struct Rst {
    software_address_selector: SoftwareAddressSelector,
    syscrg_status: SyscrgStatus,
}
impl Rst {
    #[doc = "0x00..0x10 - Software RESET Address Selector"]
    #[inline(always)]
    pub const fn software_address_selector(&self) -> &SoftwareAddressSelector {
        &self.software_address_selector
    }
    #[doc = "0x10..0x20 - SYSCRG RESET Status"]
    #[inline(always)]
    pub const fn syscrg_status(&self) -> &SyscrgStatus {
        &self.syscrg_status
    }
}
#[doc = "Software RESET Address Selector"]
pub use self::software_address_selector::SoftwareAddressSelector;
#[doc = r"Cluster"]
#[doc = "Software RESET Address Selector"]
pub mod software_address_selector;
#[doc = "SYSCRG RESET Status"]
pub use self::syscrg_status::SyscrgStatus;
#[doc = r"Cluster"]
#[doc = "SYSCRG RESET Status"]
pub mod syscrg_status;
