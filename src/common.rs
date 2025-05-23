use std::{
    future::Future,
    sync::{Arc, Mutex},
};

#[derive(Debug, Eq, PartialEq)]
pub enum GrabState {
    Ready,
    Run,
    Wait,
    Exit,
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub use arboard::Clipboard as ClipboardContext;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use hbb_common::compress::decompress;
use hbb_common::{
    allow_err,
    anyhow::bail,
    compress::compress as compress_func,
    config::{self, Config, COMPRESS_LEVEL, RENDEZVOUS_TIMEOUT},
    get_version_number, log,
    message_proto::*,
    protobuf::Enum,
    protobuf::Message as _,
    rendezvous_proto::*,
    sleep, socket_client, tokio, ResultType,
};
// #[cfg(any(target_os = "android", target_os = "ios", feature = "cli"))]
use hbb_common::{config::RENDEZVOUS_PORT, futures::future::join_all};

pub type NotifyMessageBox = fn(String, String, String, String) -> dyn Future<Output = ()>;

pub const CLIPBOARD_NAME: &'static str = "clipboard";
pub const CLIPBOARD_INTERVAL: u64 = 333;

// the executable name of the portable version
pub const PORTABLE_APPNAME_RUNTIME_ENV_KEY: &str = "RUSTDESK_APPNAME";

lazy_static::lazy_static! {
    pub static ref CONTENT: Arc<Mutex<String>> = Default::default();
    pub static ref SOFTWARE_UPDATE_URL: Arc<Mutex<String>> = Default::default();
}

lazy_static::lazy_static! {
    pub static ref DEVICE_ID: Arc<Mutex<String>> = Default::default();
    pub static ref DEVICE_NAME: Arc<Mutex<String>> = Default::default();
}

pub fn global_init() -> bool {
    #[cfg(target_os = "linux")]
    {
        if !scrap::is_x11() {
            crate::server::wayland::set_wayland_scrap_map_err();
        }
    }
    true
}

pub fn global_clean() {}

#[inline]
pub fn valid_for_numlock(evt: &KeyEvent) -> bool {
    if let Some(key_event::Union::ControlKey(ck)) = evt.union {
        let v = ck.value();
        (v >= ControlKey::Numpad0.value() && v <= ControlKey::Numpad9.value())
            || v == ControlKey::Decimal.value()
    } else {
        false
    }
}

pub fn create_clipboard_msg(content: String) -> Message {
    let bytes = content.into_bytes();
    let compressed = compress_func(&bytes, COMPRESS_LEVEL);
    let compress = compressed.len() < bytes.len();
    let content = if compress { compressed } else { bytes };
    let mut msg = Message::new();
    msg.set_clipboard(Clipboard {
        compress,
        content: content.into(),
        ..Default::default()
    });
    msg
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn check_clipboard(
    ctx: &mut ClipboardContext,
    old: Option<&Arc<Mutex<String>>>,
) -> Option<Message> {
    let side = if old.is_none() { "host" } else { "client" };
    let old = if let Some(old) = old { old } else { &CONTENT };
    if let Ok(content) = ctx.get_text() {
        if content.len() < 2_000_000 && !content.is_empty() {
            let changed = content != *old.lock().unwrap();
            if changed {
                log::info!("{} update found on {}", CLIPBOARD_NAME, side);
                *old.lock().unwrap() = content.clone();
                return Some(create_clipboard_msg(content));
            }
        }
    }
    None
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn update_clipboard(clipboard: Clipboard, old: Option<&Arc<Mutex<String>>>) {
    let content = if clipboard.compress {
        decompress(&clipboard.content)
    } else {
        clipboard.content.into()
    };
    if let Ok(content) = String::from_utf8(content) {
        if content.is_empty() {
            // ctx.set_text may crash if content is empty
            return;
        }
        match ClipboardContext::new() {
            Ok(mut ctx) => {
                let side = if old.is_none() { "host" } else { "client" };
                let old = if let Some(old) = old { old } else { &CONTENT };
                *old.lock().unwrap() = content.clone();
                allow_err!(ctx.set_text(content));
                log::debug!("{} updated on {}", CLIPBOARD_NAME, side);
            }
            Err(err) => {
                log::error!("Failed to create clipboard context: {}", err);
            }
        }
    }
}

pub async fn send_opts_after_login(
    config: &crate::client::LoginConfigHandler,
    peer: &mut hbb_common::tcp::FramedStream,
) {
    if let Some(opts) = config.get_option_message_after_login() {
        let mut misc = Misc::new();
        misc.set_option(opts);
        let mut msg_out = Message::new();
        msg_out.set_misc(misc);
        allow_err!(peer.send(&msg_out).await);
    }
}

#[cfg(feature = "use_rubato")]
pub fn resample_channels(
    data: &[f32],
    sample_rate0: u32,
    sample_rate: u32,
    channels: u16,
) -> Vec<f32> {
    use rubato::{
        InterpolationParameters, InterpolationType, Resampler, SincFixedIn, WindowFunction,
    };
    let params = InterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: InterpolationType::Nearest,
        oversampling_factor: 160,
        window: WindowFunction::BlackmanHarris2,
    };
    let mut resampler = SincFixedIn::<f64>::new(
        sample_rate as f64 / sample_rate0 as f64,
        params,
        data.len() / (channels as usize),
        channels as _,
    );
    let mut waves_in = Vec::new();
    if channels == 2 {
        waves_in.push(
            data.iter()
                .step_by(2)
                .map(|x| *x as f64)
                .collect::<Vec<_>>(),
        );
        waves_in.push(
            data.iter()
                .skip(1)
                .step_by(2)
                .map(|x| *x as f64)
                .collect::<Vec<_>>(),
        );
    } else {
        waves_in.push(data.iter().map(|x| *x as f64).collect::<Vec<_>>());
    }
    if let Ok(x) = resampler.process(&waves_in) {
        if x.is_empty() {
            Vec::new()
        } else if x.len() == 2 {
            x[0].chunks(1)
                .zip(x[1].chunks(1))
                .flat_map(|(a, b)| a.into_iter().chain(b))
                .map(|x| *x as f32)
                .collect()
        } else {
            x[0].iter().map(|x| *x as f32).collect()
        }
    } else {
        Vec::new()
    }
}

#[cfg(feature = "use_dasp")]
pub fn resample_channels(
    data: &[f32],
    sample_rate0: u32,
    sample_rate: u32,
    channels: u16,
) -> Vec<f32> {
    use dasp::{interpolate::linear::Linear, signal, Signal};
    let n = data.len() / (channels as usize);
    let n = n * sample_rate as usize / sample_rate0 as usize;
    if channels == 2 {
        let mut source = signal::from_interleaved_samples_iter::<_, [_; 2]>(data.iter().cloned());
        let a = source.next();
        let b = source.next();
        let interp = Linear::new(a, b);
        let mut data = Vec::with_capacity(n << 1);
        for x in source
            .from_hz_to_hz(interp, sample_rate0 as _, sample_rate as _)
            .take(n)
        {
            data.push(x[0]);
            data.push(x[1]);
        }
        data
    } else {
        let mut source = signal::from_iter(data.iter().cloned());
        let a = source.next();
        let b = source.next();
        let interp = Linear::new(a, b);
        source
            .from_hz_to_hz(interp, sample_rate0 as _, sample_rate as _)
            .take(n)
            .collect()
    }
}

#[cfg(feature = "use_samplerate")]
pub fn resample_channels(
    data: &[f32],
    sample_rate0: u32,
    sample_rate: u32,
    channels: u16,
) -> Vec<f32> {
    use samplerate::{convert, ConverterType};
    convert(
        sample_rate0 as _,
        sample_rate as _,
        channels as _,
        ConverterType::SincBestQuality,
        data,
    )
    .unwrap_or_default()
}

pub fn test_nat_type() {
    let mut i = 0;
    std::thread::spawn(move || loop {
        match test_nat_type_() {
            Ok(true) => break,
            Err(err) => {
                log::error!("test nat: {}", err);
            }
            _ => {}
        }
        if Config::get_nat_type() != 0 {
            break;
        }
        i = i * 2 + 1;
        if i > 300 {
            i = 300;
        }
        std::thread::sleep(std::time::Duration::from_secs(i));
    });
}

#[tokio::main(flavor = "current_thread")]
async fn test_nat_type_() -> ResultType<bool> {
    log::info!("Testing nat ...");
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let is_direct = crate::ipc::get_socks_async(1_000).await.is_none(); // sync socks BTW
    #[cfg(any(target_os = "android", target_os = "ios"))]
    let is_direct = Config::get_socks().is_none(); // sync socks BTW
    if !is_direct {
        Config::set_nat_type(NatType::SYMMETRIC as _);
        return Ok(true);
    }
    let start = std::time::Instant::now();
    let (rendezvous_server, _, _) = get_rendezvous_server(1_000).await;
    let server1 = rendezvous_server;
    let tmp: Vec<&str> = server1.split(":").collect();
    if tmp.len() != 2 {
        bail!("Invalid server address: {}", server1);
    }
    let port: u16 = tmp[1].parse()?;
    if port == 0 {
        bail!("Invalid server address: {}", server1);
    }
    let server2 = format!("{}:{}", tmp[0], port - 1);
    let mut msg_out = RendezvousMessage::new();
    let serial = Config::get_serial();
    msg_out.set_test_nat_request(TestNatRequest {
        serial,
        ..Default::default()
    });
    let mut port1 = 0;
    let mut port2 = 0;
    let server1 = socket_client::get_target_addr(&server1)?;
    let server2 = socket_client::get_target_addr(&server2)?;
    let mut addr = Config::get_any_listen_addr();
    for i in 0..2 {
        let mut socket = socket_client::connect_tcp(
            if i == 0 {
                server1.clone()
            } else {
                server2.clone()
            },
            addr,
            RENDEZVOUS_TIMEOUT,
        )
        .await?;
        addr = socket.local_addr();
        socket.send(&msg_out).await?;
        if let Some(Ok(bytes)) = socket.next_timeout(RENDEZVOUS_TIMEOUT).await {
            if let Ok(msg_in) = RendezvousMessage::parse_from_bytes(&bytes) {
                if let Some(rendezvous_message::Union::TestNatResponse(tnr)) = msg_in.union {
                    if i == 0 {
                        port1 = tnr.port;
                    } else {
                        port2 = tnr.port;
                    }
                    if let Some(cu) = tnr.cu.as_ref() {
                        Config::set_option(
                            "rendezvous-servers".to_owned(),
                            cu.rendezvous_servers.join(","),
                        );
                        Config::set_serial(cu.serial);
                    }
                }
            }
        } else {
            break;
        }
    }
    Config::set_option("local-ip-addr".to_owned(), addr.ip().to_string());
    let ok = port1 > 0 && port2 > 0;
    if ok {
        let t = if port1 == port2 {
            NatType::ASYMMETRIC
        } else {
            NatType::SYMMETRIC
        };
        Config::set_nat_type(t as _);
        log::info!("Tested nat type: {:?} in {:?}", t, start.elapsed());
    }
    Ok(ok)
}

pub async fn get_rendezvous_server(ms_timeout: u64) -> (String, Vec<String>, bool) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    let (mut a, mut b) = get_rendezvous_server_(ms_timeout);
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let (mut a, mut b) = get_rendezvous_server_(ms_timeout).await;
    let mut b: Vec<String> = b
        .drain(..)
        .map(|x| {
            if !x.contains(":") {
                format!("{}:{}", x, config::RENDEZVOUS_PORT)
            } else {
                x
            }
        })
        .collect();
    let c = if b.contains(&a) {
        b = b.drain(..).filter(|x| x != &a).collect();
        true
    } else {
        a = b.pop().unwrap_or(a);
        false
    };
    (a, b, c)
}

#[inline]
#[cfg(any(target_os = "android", target_os = "ios"))]
fn get_rendezvous_server_(_ms_timeout: u64) -> (String, Vec<String>) {
    (
        Config::get_rendezvous_server(),
        Config::get_rendezvous_servers(),
    )
}

#[inline]
#[cfg(not(any(target_os = "android", target_os = "ios")))]
async fn get_rendezvous_server_(ms_timeout: u64) -> (String, Vec<String>) {
    crate::ipc::get_rendezvous_server(ms_timeout).await
}

#[inline]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub async fn get_nat_type(_ms_timeout: u64) -> i32 {
    Config::get_nat_type()
}

#[inline]
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub async fn get_nat_type(ms_timeout: u64) -> i32 {
    crate::ipc::get_nat_type(ms_timeout).await
}

// #[cfg(any(target_os = "android", target_os = "ios", feature = "cli"))]
#[tokio::main(flavor = "current_thread")]
async fn test_rendezvous_server_() {
    let servers = Config::get_rendezvous_servers();
    Config::reset_online();
    let mut futs = Vec::new();
    for host in servers {
        futs.push(tokio::spawn(async move {
            let tm = std::time::Instant::now();
            if socket_client::connect_tcp(
                crate::check_port(&host, RENDEZVOUS_PORT),
                Config::get_any_listen_addr(),
                RENDEZVOUS_TIMEOUT,
            )
            .await
            .is_ok()
            {
                let elapsed = tm.elapsed().as_micros();
                Config::update_latency(&host, elapsed as _);
            } else {
                Config::update_latency(&host, -1);
            }
        }));
    }
    join_all(futs).await;
}

// #[cfg(any(target_os = "android", target_os = "ios", feature = "cli"))]
pub fn test_rendezvous_server() {
    std::thread::spawn(test_rendezvous_server_);
}

pub fn refresh_rendezvous_server() {
    #[cfg(any(target_os = "android", target_os = "ios", feature = "cli"))]
    test_rendezvous_server();
    #[cfg(not(any(target_os = "android", target_os = "ios", feature = "cli")))]
    std::thread::spawn(|| {
        if crate::ipc::test_rendezvous_server().is_err() {
            test_rendezvous_server();
        }
    });
}

pub fn run_me<T: AsRef<std::ffi::OsStr>>(args: Vec<T>) -> std::io::Result<std::process::Child> {
    #[cfg(not(feature = "appimage"))]
    {
        let cmd = std::env::current_exe()?;
        return std::process::Command::new(cmd).args(&args).spawn();
    }
    #[cfg(feature = "appimage")]
    {
        let appdir = std::env::var("APPDIR").unwrap();
        let appimage_cmd = std::path::Path::new(&appdir).join("AppRun");
        log::info!("path: {:?}", appimage_cmd);
        return std::process::Command::new(appimage_cmd).args(&args).spawn();
    }
}

pub fn username() -> String {
    // fix bug of whoami
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    return whoami::username().trim_end_matches('\0').to_owned();
    #[cfg(any(target_os = "android", target_os = "ios"))]
    return DEVICE_NAME.lock().unwrap().clone();
}

#[inline]
pub fn check_port<T: std::string::ToString>(host: T, port: i32) -> String {
    let host = host.to_string();
    if !host.contains(":") {
        return format!("{}:{}", host, port);
    }
    return host;
}

pub const POSTFIX_SERVICE: &'static str = "_service";

#[inline]
pub fn is_control_key(evt: &KeyEvent, key: &ControlKey) -> bool {
    if let Some(key_event::Union::ControlKey(ck)) = evt.union {
        ck.value() == key.value()
    } else {
        false
    }
}

#[inline]
pub fn is_modifier(evt: &KeyEvent) -> bool {
    if let Some(key_event::Union::ControlKey(ck)) = evt.union {
        let v = ck.value();
        v == ControlKey::Alt.value()
            || v == ControlKey::Shift.value()
            || v == ControlKey::Control.value()
            || v == ControlKey::Meta.value()
            || v == ControlKey::RAlt.value()
            || v == ControlKey::RShift.value()
            || v == ControlKey::RControl.value()
            || v == ControlKey::RWin.value()
    } else {
        false
    }
}

pub fn check_software_update() {
    std::thread::spawn(move || allow_err!(check_software_update_()));
}

#[tokio::main(flavor = "current_thread")]
async fn check_software_update_() -> hbb_common::ResultType<()> {
    sleep(3.).await;

    let rendezvous_server =
        socket_client::get_target_addr(&format!("remote.scanntech.com:{}", config::RENDEZVOUS_PORT))?;
    let mut socket =
        socket_client::new_udp(Config::get_any_listen_addr(), RENDEZVOUS_TIMEOUT).await?;

    let mut msg_out = RendezvousMessage::new();
    msg_out.set_software_update(SoftwareUpdate {
        url: crate::VERSION.to_owned(),
        ..Default::default()
    });
    socket.send(&msg_out, rendezvous_server).await?;
    use hbb_common::protobuf::Message;
    if let Some(Ok((bytes, _))) = socket.next_timeout(30_000).await {
        if let Ok(msg_in) = RendezvousMessage::parse_from_bytes(&bytes) {
            if let Some(rendezvous_message::Union::SoftwareUpdate(su)) = msg_in.union {
                let version = hbb_common::get_version_from_url(&su.url);
                if get_version_number(&version) > get_version_number(crate::VERSION) {
                    *SOFTWARE_UPDATE_URL.lock().unwrap() = su.url;
                }
            }
        }
    }
    Ok(())
}

#[cfg(not(any(target_os = "android", target_os = "ios", feature = "cli")))]
pub fn get_icon() -> String {
    hbb_common::config::ICON.to_owned()
}

pub fn get_app_name() -> String {
    hbb_common::config::APP_NAME.read().unwrap().clone()
}

#[cfg(target_os = "macos")]
pub fn get_full_name() -> String {
    format!(
        "{}.{}",
        hbb_common::config::ORG.read().unwrap(),
        hbb_common::config::APP_NAME.read().unwrap(),
    )
}

pub fn is_ip(id: &str) -> bool {
    hbb_common::regex::Regex::new(r"^\d+\.\d+\.\d+\.\d+(:\d+)?$")
        .unwrap()
        .is_match(id)
}

pub fn is_setup(name: &str) -> bool {
    name.to_lowercase().ends_with("install.exe")
}

pub fn get_custom_rendezvous_server(custom: String) -> String {
    if !custom.is_empty() {
        return custom;
    }
    #[cfg(windows)]
    if let Some(lic) = crate::platform::windows::get_license() {
        if !lic.host.is_empty() {
            return lic.host.clone();
        }
    }
    if !config::PROD_RENDEZVOUS_SERVER.read().unwrap().is_empty() {
        return config::PROD_RENDEZVOUS_SERVER.read().unwrap().clone();
    }
    "".to_owned()
}

pub fn get_api_server(api: String, custom: String) -> String {
    if !api.is_empty() {
        return api.to_owned();
    }
    #[cfg(windows)]
    if let Some(lic) = crate::platform::windows::get_license() {
        if !lic.api.is_empty() {
            return lic.api.clone();
        }
    }
    let s = get_custom_rendezvous_server(custom);
    if !s.is_empty() {
        if s.contains(':') {
            let tmp: Vec<&str> = s.split(":").collect();
            if tmp.len() == 2 {
                let port: u16 = tmp[1].parse().unwrap_or(0);
                if port > 2 {
                    return format!("http://{}:{}", tmp[0], port - 2);
                }
            }
        } else {
            return format!("http://{}:{}", s, config::RENDEZVOUS_PORT - 2);
        }
    }
    "https://admin.rustdesk.com".to_owned()
}

pub fn get_audit_server(api: String, custom: String) -> String {
    let url = get_api_server(api, custom);
    if url.is_empty() || url.contains("rustdesk.com") {
        return "".to_owned();
    }
    format!("{}/api/audit", url)
}

pub async fn post_request(url: String, body: String, header: &str) -> ResultType<String> {
    #[cfg(not(target_os = "linux"))]
    {
        let mut req = reqwest::Client::new().post(url);
        if !header.is_empty() {
            let tmp: Vec<&str> = header.split(": ").collect();
            if tmp.len() == 2 {
                req = req.header(tmp[0], tmp[1]);
            }
        }
        req = req.header("Content-Type", "application/json");
        let to = std::time::Duration::from_secs(12);
        Ok(req.body(body).timeout(to).send().await?.text().await?)
    }
    #[cfg(target_os = "linux")]
    {
        let mut data = vec![
            "curl",
            "-sS",
            "-X",
            "POST",
            &url,
            "-H",
            "Content-Type: application/json",
            "-d",
            &body,
            "--connect-timeout",
            "12",
        ];
        if !header.is_empty() {
            data.push("-H");
            data.push(header);
        }
        let output = async_process::Command::new("curl")
            .args(&data)
            .output()
            .await?;
        let res = String::from_utf8_lossy(&output.stdout).to_string();
        if !res.is_empty() {
            return Ok(res);
        }
        bail!(String::from_utf8_lossy(&output.stderr).to_string());
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn post_request_sync(url: String, body: String, header: &str) -> ResultType<String> {
    post_request(url, body, header).await
}

#[inline]
pub fn make_privacy_mode_msg(state: back_notification::PrivacyModeState) -> Message {
    let mut misc = Misc::new();
    let mut back_notification = BackNotification::new();
    back_notification.set_privacy_mode_state(state);
    misc.set_back_notification(back_notification);
    let mut msg_out = Message::new();
    msg_out.set_misc(misc);
    msg_out
}

#[cfg(not(target_os = "linux"))]
lazy_static::lazy_static! {
    pub static ref IS_X11: Mutex<bool> = Mutex::new(false);

}

#[cfg(target_os = "linux")]
lazy_static::lazy_static! {
    pub static ref IS_X11: Mutex<bool> = Mutex::new("x11" == hbb_common::platform::linux::get_display_server());
}
