use anyhow::{Result, anyhow};
use futures::executor::block_on;
use serde::Deserialize;
use obws::Client;
use std::fs;
use toml;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::mem::size_of;
use std::ptr;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::UI::Input::*,
    Win32::System::LibraryLoader::*
};
mod router;
mod processes;
mod events;


#[derive(Debug, Deserialize)]
struct Config {
    obs_client: ObsClientConfig
}

#[derive(Debug, Deserialize)]
struct ObsClientConfig {
    host: Option<String>,
    port: Option<u16>,
    password: Option<String>
}

async fn init_client_from_config(path: &str) -> Result<Client> {
    let s = fs::read_to_string(path)?;
    let config: Config = toml::from_str(s.as_str())?;
    match config.obs_client {
        ObsClientConfig { host: Some(host), port: Some(port), password } => {
            Result::Ok(Client::connect(host, port, password).await?)
        }

        _ => {
            Result::Err(anyhow::anyhow!("Invalid config."))
        }
    }
}

fn init_router_and_table() -> (router::Router, events::EventTable) {
    let process_pairs = processes::init_processes();
    let router  = router::Router::init(&process_pairs);

    let process_indices = process_pairs.iter().map(|(i, _)| *i).collect::<Vec<usize>>();
    let table = events::EventTable::init(&process_indices);

    (router, table)
}

struct ObsProcessSystem {
    client: Client,
    router: router::Router,
    event_table: events::EventTable
}

impl ObsProcessSystem {
    fn handle_from_raw_event(&mut self, raw_event: events::RawEvent) {
        let client = &self.client;
        let router = &self.router;
        let event_table = &mut self.event_table;
        if let Some(index) = &event_table.handle_from_raw_event(raw_event) {
            let r = block_on(router.process(&client, *index)).unwrap();
        }
    }
}

static OBS_PROCESS_SYSTEM: Lazy<Mutex<ObsProcessSystem>> = Lazy::new(|| {
    let client = block_on(async {init_client_from_config("./config.toml").await}).unwrap();
    let (router, event_table) = init_router_and_table();
    Mutex::new(ObsProcessSystem { client, router, event_table })
});

#[tokio::main]
async fn main() -> Result<()> {
    unsafe { 
        // ウィンドウクラスの登録
        let h_instance = std::mem::transmute(GetModuleHandleA(None)?);
        let class_name = s!("my_window_class");

        let wnd_class = WNDCLASSA {
            hInstance: h_instance,
            lpszClassName: class_name,
            lpfnWndProc: Some(wnd_proc),
            ..Default::default()
        };

        RegisterClassA(&wnd_class);

        // メッセージを受け取るための不可視ウィンドウを作成
        let hwnd = CreateWindowExA(
            Default::default(),
            class_name,
            s!("Key Detection"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND(0),
            HMENU(0),
            h_instance,
            Some(ptr::null_mut()),
        );

        // RAWINPUTデバイスの登録
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01,
            usUsage: 0x06, // キーボード
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        };

        if RegisterRawInputDevices(&[rid], size_of::<RAWINPUTDEVICE>() as u32).is_ok() {
            println!("RAW input devices registered.");
        } else {
            eprintln!("Failed to register RAW input devices.");
            return Result::Err(anyhow!("windows api error"));
        }

        println!("Listening for keyboard input. Press 'Ctrl+c' on terminal to exit.");

        // メッセージループ
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
    Ok(())
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_INPUT => {
            let mut data: [u8; 48] = [0; 48];
            let mut data_size = data.len() as u32;

            if GetRawInputData(HRAWINPUT(lparam.0), RID_INPUT, Some(data.as_mut_ptr().cast()), &mut data_size, size_of::<RAWINPUTHEADER>() as u32) == u32::MAX {
                eprintln!("Failed to get raw input data.");
            } else {
                let raw: &RAWINPUT = &*(data.as_ptr() as *const _);
                if raw.header.dwType == RIM_TYPEKEYBOARD.0 {
                    let device_handle = raw.header.hDevice;

                    // 起動時、先にdevice_handleを調べる必要がある。
                    if device_handle == HANDLE(65611) {
                        let keyboard = raw.data.keyboard;
                        let vk = keyboard.VKey;
                        let flag = keyboard.Flags;
                        let mut obs = OBS_PROCESS_SYSTEM.lock().unwrap();
                        obs.handle_from_raw_event((vk, flag));
                    }
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}
