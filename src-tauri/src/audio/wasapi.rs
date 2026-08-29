use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use serde::{Deserialize, Serialize};
use windows::core::Interface;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Media::Audio::Endpoints::IAudioMeterInformation;
use windows::Win32::Media::Audio::{
    eConsole, eRender, AudioSessionStateActive, IAudioSessionControl2,
    IAudioSessionEnumerator, IAudioSessionManager2, IMMDevice, IMMDeviceEnumerator,
    MMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
};
use windows::Win32::System::ProcessStatus::K32GetProcessImageFileNameW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessAudioInfo {
    pub name: String,
    pub pid: u32,
    pub peak: f32,
}

pub struct WasapiMonitor {
    initialized: bool,
}

impl WasapiMonitor {
    pub fn new() -> Self {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        }
        Self { initialized: true }
    }

    pub fn get_active_sessions(&self) -> (f32, Vec<ProcessAudioInfo>) {
        let mut max_overall_peak = 0.0f32;
        let mut active_list = Vec::new();

        unsafe {
            let enumerator_res: Result<IMMDeviceEnumerator, _> =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL);

            let enumerator = match enumerator_res {
                Ok(e) => e,
                Err(_) => return (0.0, active_list),
            };

            let device_res = enumerator.GetDefaultAudioEndpoint(eRender, eConsole);
            let device: IMMDevice = match device_res {
                Ok(d) => d,
                Err(_) => return (0.0, active_list),
            };

            let session_manager_res: Result<IAudioSessionManager2, _> =
                device.Activate(CLSCTX_ALL, None);
            let session_manager = match session_manager_res {
                Ok(sm) => sm,
                Err(_) => return (0.0, active_list),
            };

            let session_enum_res = session_manager.GetSessionEnumerator();
            let session_enum: IAudioSessionEnumerator = match session_enum_res {
                Ok(se) => se,
                Err(_) => return (0.0, active_list),
            };

            let count = match session_enum.GetCount() {
                Ok(c) => c,
                Err(_) => return (0.0, active_list),
            };

            for i in 0..count {
                if let Ok(control) = session_enum.GetSession(i) {
                    let state = control.GetState().unwrap_or(windows::Win32::Media::Audio::AudioSessionStateInactive);

                    let meter_res: Result<IAudioMeterInformation, _> = control.cast();
                    let peak = if let Ok(meter) = meter_res {
                        meter.GetPeakValue().unwrap_or(0.0)
                    } else {
                        0.0
                    };

                    if peak > max_overall_peak {
                        max_overall_peak = peak;
                    }

                    let control2_res: Result<IAudioSessionControl2, _> = control.cast();
                    if let Ok(control2) = control2_res {
                        if let Ok(pid) = control2.GetProcessId() {
                            if pid > 0 {
                                let is_active = state == AudioSessionStateActive || peak > 0.001;
                                if is_active {
                                    let process_name = get_process_name(pid);
                                    // Avoid duplicates
                                    if let Some(existing) = active_list.iter_mut().find(|p: &&mut ProcessAudioInfo| p.pid == pid) {
                                        if peak > existing.peak {
                                            existing.peak = peak;
                                        }
                                    } else {
                                        active_list.push(ProcessAudioInfo {
                                            name: process_name,
                                            pid,
                                            peak,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        (max_overall_peak, active_list)
    }
}

impl Drop for WasapiMonitor {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

fn get_process_name(pid: u32) -> String {
    unsafe {
        let handle: Result<HANDLE, _> = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);
        if let Ok(h) = handle {
            if !h.is_invalid() {
                let mut buffer = [0u16; 512];
                let len = K32GetProcessImageFileNameW(h, &mut buffer);
                let _ = CloseHandle(h);

                if len > 0 {
                    let path_str = OsString::from_wide(&buffer[..len as usize]);
                    if let Some(path_lossy) = path_str.to_str() {
                        if let Some(file_name) = Path::new(path_lossy).file_name() {
                            return file_name.to_string_lossy().to_string();
                        }
                    }
                }
            }
        }
    }
    format!("App ({})", pid)
}
