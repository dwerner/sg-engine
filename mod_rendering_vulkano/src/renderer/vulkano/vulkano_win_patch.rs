use game_state::winit;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::swapchain::Surface;
use vulkano::swapchain::SurfaceCreationError;

use std::sync::Arc;

use std::ptr;

pub fn required_extensions() -> InstanceExtensions {
    let ideal = InstanceExtensions {
        khr_surface: true,
        khr_xlib_surface: true,
        khr_xcb_surface: true,
        khr_wayland_surface: true,
        khr_android_surface: true,
        khr_win32_surface: true,
        mvk_ios_surface: true,
        mvk_macos_surface: true,
        ext_debug_report: true,
        ..InstanceExtensions::none()
    };

    match InstanceExtensions::supported_by_core() {
        Ok(supported) => supported.intersection(&ideal),
        Err(_) => InstanceExtensions::none(),
    }
}

type AMWin = Arc<winit::Window>;

#[cfg(target_os = "android")]
pub unsafe fn winit_to_surface(
    instance: Arc<Instance>,
    win: AMWin,
) -> Result<Arc<Surface<AMWin>>, SurfaceCreationError> {
    use winit::os::android::WindowExt;
    Surface::from_anativewindow(instance, &win.get_native_window(), win.clone())
}

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub unsafe fn winit_to_surface(
    instance: Arc<Instance>,
    win: AMWin,
) -> Result<Arc<Surface<AMWin>>, SurfaceCreationError> {
    use winit::os::unix::WindowExt;
    match (&win.get_wayland_display(), win.get_wayland_surface()) {
        (Some(display), Some(surface)) => {
            Surface::from_wayland(instance, display, surface, win.clone())
        }
        _ => {
            // No wayland display found, check if we can use xlib.
            // If not, we use xcb.

            if instance.loaded_extensions().khr_xlib_surface {
                Surface::from_xlib(
                    instance,
                    win.get_xlib_display().unwrap(),
                    win.get_xlib_window().unwrap() as _,
                    win.clone(),
                )
            } else {
                Surface::from_xcb(
                    instance,
                    win.get_xcb_connection().unwrap(),
                    win.get_xlib_window().unwrap() as _,
                    win.clone(),
                )
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub unsafe fn winit_to_surface(
    instance: Arc<Instance>,
    win: AMWin,
) -> Result<Arc<Surface<AMWin>>, SurfaceCreationError> {
    use winit::os::windows::WindowExt;
    let hwnd = win.get_hwnd();
    Surface::from_hwnd(
        instance,
        ptr::null() as *const (), // FIXME
        hwnd,
        win.clone(),
    )
}

#[cfg(target_os = "macos")]
pub unsafe fn winit_to_surface(
    instance: Arc<Instance>,
    win: AMWin,
) -> Result<Arc<Surface<AMWin>>, SurfaceCreationError> {
    use winit::os::macos::WindowExt;

    let wnd: cocoa_id = mem::transmute(&win.get_nswindow());

    let layer = CoreAnimationLayer::new();

    layer.set_edge_antialiasing_mask(0);
    layer.set_presents_with_transaction(false);
    layer.remove_all_animations();

    let view = wnd.contentView();

    layer.set_contents_scale(view.backingScaleFactor());
    view.setLayer(mem::transmute(layer.as_ref())); // Bombs here with out of memory
    view.setWantsLayer(YES);

    Surface::from_macos_moltenvk(instance, win.get_nsview() as *const (), win.clone())
}
