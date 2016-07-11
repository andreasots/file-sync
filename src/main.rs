extern crate clap;
extern crate kernel32;
extern crate notify;
extern crate winapi;

mod winsvc;
mod guard;

use guard::Guard;
use std::os::windows::ffi::OsStrExt;

fn encode<S: AsRef<std::ffi::OsStr> + ?Sized>(s: &S) -> Vec<winapi::WCHAR> {
    s.as_ref().encode_wide().chain(Some(0)).collect()
}

fn install(dir1: &std::ffi::OsStr, dir2: &std::ffi::OsStr) {
    unsafe {
        let scm = winsvc::OpenSCManagerW(std::ptr::null_mut(), std::ptr::null_mut(), winapi::SC_MANAGER_CONNECT | winapi::SC_MANAGER_CREATE_SERVICE);
        if scm.is_null() {
            panic!("Failed to connect to service manager: {:#x}", kernel32::GetLastError());
        }
        let scm = Guard::new(scm, |scm| { winsvc::CloseServiceHandle(scm); });

        let mut service_name = encode("file-sync");

        let mut exe_path = std::env::current_exe().unwrap().as_os_str().encode_wide()
            .chain([' ', '"'].into_iter().map(|&c| c as winapi::WCHAR))
            .chain(dir1.encode_wide())
            .chain([' ', '"', ' '].into_iter().map(|&c| c as winapi::WCHAR))
            .chain(dir2.encode_wide())
            .chain(['"', '\0'].into_iter().map(|&c| c as winapi::WCHAR))
            .collect::<Vec<winapi::WCHAR>>();

        let mut deps = encode("");
        let mut user = encode("NT AUTHORITY\\LocalService");

        let srv = winsvc::CreateServiceW(
            *scm,
            service_name.as_mut_ptr(),
            service_name.as_mut_ptr(),
            winapi::SERVICE_QUERY_STATUS,
            winapi::SERVICE_WIN32_OWN_PROCESS,
            winsvc::SERVICE_AUTO_START,
            winsvc::SERVICE_ERROR_NORMAL,
            exe_path.as_mut_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            deps.as_mut_ptr(),
            user.as_mut_ptr(),
            std::ptr::null_mut(),
        );
        if srv.is_null() {
            panic!("Failed to create service: {:#x}", kernel32::GetLastError());
        }
        let _srv = Guard::new(srv, |srv| { winsvc::CloseServiceHandle(srv); });
    }
}

fn uninstall() {
    unimplemented!()
}

fn run(dir1: &std::ffi::OsStr, dir2: &std::ffi::OsStr) {
    unimplemented!()
}

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Keep two directories in sync.")
        .arg(clap::Arg::with_name("install")
            .long("install")
            .help("Install service.")
        )
        .arg(clap::Arg::with_name("uninstall")
            .long("uninstall")
            .help("Uninstall service.")
        )
        .arg(clap::Arg::with_name("DIR1")
            .help("First directory to watch.")
            .required_unless("uninstall")
        )
        .arg(clap::Arg::with_name("DIR2")
            .help("Second directory to watch.")
            .required_unless("uninstall")
        )
        .get_matches();

    if matches.is_present("install") {
        install(matches.value_of_os("DIR1").unwrap(), matches.value_of_os("DIR2").unwrap());
    } else if matches.is_present("uninstall") {
        uninstall();
    } else {
        run(matches.value_of_os("DIR1").unwrap(), matches.value_of_os("DIR2").unwrap());
    }
}
