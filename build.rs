fn main() {
    // let path = "/home/maratworker/Downloads/game/SFML-2.5.1-linux-gcc-64-bit/SFML-2.5.1/lib";
    let path = "/home/maratworker/Downloads/game/CSFML-2.5-sources/CSFML-2.5/build/lib";
    println!("cargo:rustc-link-search=native={}", path);
    // println!("cargo:rustc-link-lib=dylib=csfml-audio.so");
    // println!("cargo:rustc-link-lib={}/libcsfml-system.so", path);
    // println!("cargo:rustc-link-lib={}/libcsfml-window.so", path);
    // println!("cargo:rustc-link-lib={}/libcsfml-graphics.so", path);
    // println!("cargo:rustc-link-search={}", cpath);
    // println!("cargo:rustc-link-lib={}/libdisplay.so", path);
    

}