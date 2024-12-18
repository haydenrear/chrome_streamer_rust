use std::{env, fs};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;
use std::path::Path;
use std::process::Command;
use std::ptr::write;
use syn::__private::{Span, ToTokens};
use syn::{braced, Fields, Ident, Item, ItemMod, ItemStruct, Token, token, Visibility, VisPublic};
use syn::__private::quote::__private::push_div_eq_spanned;
use syn::parse::{ParseBuffer, ParseStream};
use syn::token::Brace;
use build_lib::replace_modules;
use codegen_utils::env::{get_project_base_build_dir, get_build_project_dir};
use codegen_utils::project_directory;
use crate_gen::CrateWriter;

fn main() {
    // replace_modules(
    //     Some(get_build_project_dir("chrome_streamer_rdev_filter/src").as_str()),
    //     vec![get_project_base_build_dir().as_str()]
    // );
    // let _ = Command::new("brew install pkg-config libavutil libavcodec libavformat libswscale libswresample libavdevice freetype")
    //     .spawn();
}