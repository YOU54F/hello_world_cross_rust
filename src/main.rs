fn main() {
    println!("Hello, world!");

    // Print OS and Architecture
    println!("OS: {}", std::env::consts::OS);
    println!("Architecture: {}", std::env::consts::ARCH);
}


// // For panic=unwind
// // #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64", all(target_os = "windows", target_arch = "x86")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_GetDataRelBase() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "riscv64", all(target_os = "windows", target_arch = "x86")))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_GetIPInfo() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64", all(target_os = "windows", target_arch = "x86")))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_GetLanguageSpecificData() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64", all(target_os = "windows", target_arch = "x86")))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_GetRegionStart() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64", all(target_os = "windows", target_arch = "x86")))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_GetTextRelBase() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64", all(target_os = "windows", target_arch = "x86")))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_Resume() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64",all(target_os = "windows", target_arch = "x86")))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_SetGR() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64", all(target_os = "windows", target_arch = "x86")))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_SetIP() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64"))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_RaiseException() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64"))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_DeleteException() {}
// // #[cfg(any(target_arch = "powerpc64"))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_FindEnclosingFunction() {}
// // #[cfg(any(target_arch = "powerpc64"))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_GetCFA() {}
// // #[cfg(any(target_arch = "powerpc64"))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]

// // For panic=abort
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64"))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_GetIP() {}
// // #[cfg(any(target_arch = "s390x", target_arch = "powerpc64", target_arch = "riscv64"))]
// #[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
// #[no_mangle]
// #[allow(non_snake_case)]
// fn _Unwind_Backtrace() {}