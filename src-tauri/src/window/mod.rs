#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "windows")]
pub mod win32;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux::LinuxWindowHelper as WindowHelper;

#[cfg(target_os = "windows")]
pub use win32::Win32WindowHelper as WindowHelper;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub use linux::LinuxWindowHelper as WindowHelper;
