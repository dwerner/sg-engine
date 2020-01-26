use game_state::sdl2;
use sdl2::sys::SDL_Window;
use sdl2::sys::SDL_bool::SDL_FALSE;
use sdl2::sys::SDL_SYSWM_TYPE;
use sdl2::sys::{SDL_GetError, SDL_GetWindowWMInfo, SDL_SysWMinfo};
use sdl2::sys::{SDL_MAJOR_VERSION, SDL_MINOR_VERSION, SDL_PATCHLEVEL};
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::sync::Arc;
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::{Surface, SurfaceCreationError};

#[derive(Copy, Clone)]
pub struct WinPtr {
    pub raw: *const SDL_Window,
}

unsafe impl Send for WinPtr {}
unsafe impl Sync for WinPtr {}

#[derive(Debug)]
pub enum ErrorType {
    Unknown,
    PlatformNotSupported,
    OutOfMemory,
    MissingExtension(String),
    Generic(String),
}

pub fn required_extensions(window: WinPtr) -> Result<InstanceExtensions, ErrorType> {
    let wm_info = get_wminfo(window.raw)?;
    let mut extensions = InstanceExtensions {
        khr_surface: true,
        ..InstanceExtensions::none()
    };
    let ideal = InstanceExtensions {
        khr_surface: true,
        khr_xlib_surface: true,
        khr_xcb_surface: true,
        khr_wayland_surface: true,
        khr_android_surface: true,
        khr_win32_surface: true,
        mvk_ios_surface: true,
        mvk_macos_surface: true,
        ..InstanceExtensions::none()
    };
    match wm_info.subsystem {
        SDL_SYSWM_TYPE::SDL_SYSWM_X11 => extensions.khr_xlib_surface = true,
        SDL_SYSWM_TYPE::SDL_SYSWM_WAYLAND => extensions.khr_wayland_surface = true,
        SDL_SYSWM_TYPE::SDL_SYSWM_WINDOWS => extensions.khr_win32_surface = true,
        SDL_SYSWM_TYPE::SDL_SYSWM_ANDROID => extensions.khr_android_surface = true,
        _ => return Err(ErrorType::PlatformNotSupported),
    }
    let supported = extensions;
    match InstanceExtensions::supported_by_core() {
        Ok(supported) => Ok(supported.intersection(&ideal)),
        Err(_) => Ok(InstanceExtensions::none()),
    }
}

pub fn build_vk_surface(
    window: WinPtr,
    instance: Arc<Instance>,
) -> Result<Arc<Surface<WinPtr>>, ErrorType> {
    let wm_info = get_wminfo(window.raw)?;
    unsafe { sdl2_to_surface(&wm_info, instance, window) }
}

#[cfg(target_os = "android")]
unsafe fn sdl2_to_surface(
    wm_info: &SDL_SysWMinfo,
    instance: Arc<Instance>,
    win: *mut SDL_Window,
) -> Result<Arc<Surface<WinPtr>>, ErrorType> {
    let window = wm_info.info.android.window;
    translate_vk_result(Surface::from_anativewindow(instance, window, win))
}

#[cfg(all(unix, not(target_os = "android")))]
unsafe fn sdl2_to_surface(
    wm_info: &SDL_SysWMinfo,
    instance: Arc<Instance>,
    win: WinPtr,
) -> Result<Arc<Surface<WinPtr>>, ErrorType> {
    if wm_info.subsystem == SDL_SYSWM_TYPE::SDL_SYSWM_X11 {
        let display = wm_info.info.x11.display;
        let window = wm_info.info.x11.window;
        translate_vk_result(Surface::from_xlib(instance, display, window, win.clone()))
    } else if wm_info.subsystem == SDL_SYSWM_TYPE::SDL_SYSWM_WAYLAND {
        let display = wm_info.info.wl.display;
        let surface = wm_info.info.wl.surface;
        translate_vk_result(Surface::from_wayland(
            instance,
            display,
            surface,
            win.clone(),
        ))
    } else {
        unreachable!();
    }
}

#[cfg(target_os = "windows")]
unsafe fn sdl2_to_surface(
    wm_info: &SDL_SysWMinfo,
    instance: Arc<Instance>,
) -> Result<Arc<Surface<WinPtr>>, ErrorType> {
    let hinstance = wm_info.info.win.hinstance;
    let hwnd = wm_info.info.win.window;
    translate_vk_result(Surface::from_hwnd(instance, hinstance, hwnd))
}

fn translate_vk_result(
    obj: Result<Arc<Surface<WinPtr>>, SurfaceCreationError>,
) -> Result<Arc<Surface<WinPtr>>, ErrorType> {
    match obj {
        Ok(x) => Ok(x),
        Err(SurfaceCreationError::OomError(_)) => Err(ErrorType::OutOfMemory),
        Err(SurfaceCreationError::MissingExtension { name: x }) => {
            Err(ErrorType::MissingExtension(String::from(x)))
        }
    }
}

fn get_wminfo(window: *const SDL_Window) -> Result<SDL_SysWMinfo, ErrorType> {
    let mut wm_info: SDL_SysWMinfo;
    unsafe {
        wm_info = mem::zeroed();
    }
    wm_info.version.major = SDL_MAJOR_VERSION as u8;
    wm_info.version.minor = SDL_MINOR_VERSION as u8;
    wm_info.version.patch = SDL_PATCHLEVEL as u8;
    unsafe {
        let window = window as *mut _;
        if SDL_GetWindowWMInfo(window, &mut wm_info as *mut SDL_SysWMinfo) == SDL_FALSE {
            let error = CString::from_raw(SDL_GetError() as *mut c_char);
            match error.into_string() {
                Ok(x) => return Err(ErrorType::Generic(x)),
                Err(_) => return Err(ErrorType::Unknown),
            }
        }
    }
    Ok(wm_info)
}
