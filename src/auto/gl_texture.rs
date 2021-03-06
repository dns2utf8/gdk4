// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::translate::*;
use std::fmt;
use Paintable;
use Texture;

glib_wrapper! {
    pub struct GLTexture(Object<gdk_sys::GdkGLTexture, gdk_sys::GdkGLTextureClass, GLTextureClass>) @extends Texture, @implements Paintable;

    match fn {
        get_type => || gdk_sys::gdk_gl_texture_get_type(),
    }
}

impl GLTexture {
    //pub fn new(context: &GLContext, id: u32, width: i32, height: i32, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> GLTexture {
    //    unsafe { TODO: call gdk_sys:gdk_gl_texture_new() }
    //}

    pub fn release(&self) {
        unsafe {
            gdk_sys::gdk_gl_texture_release(self.to_glib_none().0);
        }
    }
}

impl fmt::Display for GLTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GLTexture")
    }
}
