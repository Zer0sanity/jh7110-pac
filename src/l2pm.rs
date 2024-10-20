#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cache_control: CacheControl,
    _reserved1: [u8; 0x1728],
    event_control: EventControl,
    _reserved2: [u8; 0x07f8],
    event_counter: [EventCounter; 6],
}
impl RegisterBlock {
    #[doc = "0x00..0x8d8 - L2 Cache Control registers."]
    #[inline(always)]
    pub const fn cache_control(&self) -> &CacheControl {
        &self.cache_control
    }
    #[doc = "0x2000..0x2808 - L2PM Event Control registers."]
    #[inline(always)]
    pub const fn event_control(&self) -> &EventControl {
        &self.event_control
    }
    #[doc = "0x3000..0x3030 - L2PM Event Control Event Select configuration."]
    #[inline(always)]
    pub const fn event_counter(&self, n: usize) -> &EventCounter {
        &self.event_counter[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3000..0x3030 - L2PM Event Control Event Select configuration."]
    #[inline(always)]
    pub fn event_counter_iter(&self) -> impl Iterator<Item = &EventCounter> {
        self.event_counter.iter()
    }
}
#[doc = "L2 Cache Control registers."]
pub use self::cache_control::CacheControl;
#[doc = r"Cluster"]
#[doc = "L2 Cache Control registers."]
pub mod cache_control;
#[doc = "L2PM Event Control registers."]
pub use self::event_control::EventControl;
#[doc = r"Cluster"]
#[doc = "L2PM Event Control registers."]
pub mod event_control;
#[doc = "event_counter (r) register accessor: L2PM Event Control Event Select configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_counter::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_counter`]
module"]
#[doc(alias = "event_counter")]
pub type EventCounter = crate::Reg<event_counter::EventCounterSpec>;
#[doc = "L2PM Event Control Event Select configuration."]
pub mod event_counter;
