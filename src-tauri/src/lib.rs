use std::ffi::{CStr, CString};
use std::ptr;

use serde::{Deserialize, Serialize};

#[repr(C)]
struct PdfAnnot {
    _opaque: [u8; 0],
}

#[repr(C)]
struct PdfObj {
    _opaque: [u8; 0],
}

#[repr(C)]
struct FzFont {
    _opaque: [u8; 0],
}

extern "C" {
    fn fz_new_context_imp(
        alloc: *const std::ffi::c_void,
        locks: *const std::ffi::c_void,
        max_store: usize,
        version: *const std::os::raw::c_char,
    ) -> *mut std::ffi::c_void;
    fn fz_drop_context(ctx: *mut std::ffi::c_void);
    fn fz_register_document_handlers(ctx: *mut std::ffi::c_void);

    fn pdf_open_document(
        ctx: *mut std::ffi::c_void,
        filename: *const std::os::raw::c_char,
    ) -> *mut PdfDocument;
    fn pdf_create_document(ctx: *mut std::ffi::c_void) -> *mut PdfDocument;
    fn pdf_drop_document(ctx: *mut std::ffi::c_void, doc: *mut PdfDocument);
    fn pdf_count_pages(ctx: *mut std::ffi::c_void, doc: *mut PdfDocument) -> std::os::raw::c_int;
    fn pdf_delete_page(
        ctx: *mut std::ffi::c_void,
        doc: *mut PdfDocument,
        number: std::os::raw::c_int,
    );
    fn pdf_graft_page(
        ctx: *mut std::ffi::c_void,
        dst: *mut PdfDocument,
        page_to: std::os::raw::c_int,
        src: *mut PdfDocument,
        page_from: std::os::raw::c_int,
    );
    fn pdf_save_document(
        ctx: *mut std::ffi::c_void,
        doc: *mut PdfDocument,
        filename: *const std::os::raw::c_char,
        opts: *const PdfWriteOptions,
    );
    fn pdf_load_page(
        ctx: *mut std::ffi::c_void,
        doc: *mut PdfDocument,
        number: std::os::raw::c_int,
    ) -> *mut PdfPage;
    fn pdf_drop_page(ctx: *mut std::ffi::c_void, page: *mut PdfPage);
    fn pdf_bound_page(
        ctx: *mut std::ffi::c_void,
        page: *mut PdfPage,
        bbox: *mut FzRect,
    ) -> *mut FzRect;

    fn pdf_create_annot(
        ctx: *mut std::ffi::c_void,
        page: *mut PdfPage,
        typ: std::os::raw::c_int,
    ) -> *mut PdfAnnot;
    fn pdf_delete_annot(
        ctx: *mut std::ffi::c_void,
        page: *mut PdfPage,
        annot: *mut PdfAnnot,
    );
    fn pdf_first_annot(ctx: *mut std::ffi::c_void, page: *mut PdfPage) -> *mut PdfAnnot;
    fn pdf_next_annot(ctx: *mut std::ffi::c_void, annot: *mut PdfAnnot) -> *mut PdfAnnot;
    fn pdf_annot_type(ctx: *mut std::ffi::c_void, annot: *mut PdfAnnot) -> std::os::raw::c_int;
    fn pdf_annot_contents(ctx: *mut std::ffi::c_void, annot: *mut PdfAnnot) -> *const std::os::raw::c_char;
    fn pdf_annot_rect(ctx: *mut std::ffi::c_void, annot: *mut PdfAnnot) -> FzRect;
    fn pdf_annot_obj(ctx: *mut std::ffi::c_void, annot: *mut PdfAnnot) -> *mut PdfObj;
    fn pdf_set_annot_rect(ctx: *mut std::ffi::c_void, annot: *mut PdfAnnot, rect: FzRect);
    fn pdf_set_annot_contents(
        ctx: *mut std::ffi::c_void,
        annot: *mut PdfAnnot,
        text: *const std::os::raw::c_char,
    );
    fn pdf_set_annot_default_appearance(
        ctx: *mut std::ffi::c_void,
        annot: *mut PdfAnnot,
        font: *const std::os::raw::c_char,
        size: f32,
        n: std::os::raw::c_int,
        color: *const f32,
    );
    fn pdf_annot_default_appearance(
        ctx: *mut std::ffi::c_void,
        annot: *mut PdfAnnot,
        font: *mut *const std::os::raw::c_char,
        size: *mut f32,
        n: *mut std::os::raw::c_int,
        color: *mut f32,
    );
    fn pdf_update_annot(ctx: *mut std::ffi::c_void, annot: *mut PdfAnnot) -> std::os::raw::c_int;
    fn pdf_to_num(ctx: *mut std::ffi::c_void, obj: *mut PdfObj) -> std::os::raw::c_int;

    fn fz_new_pixmap(
        ctx: *mut std::ffi::c_void,
        cs: *mut std::ffi::c_void,
        w: std::os::raw::c_int,
        h: std::os::raw::c_int,
        seps: *mut std::ffi::c_void,
        alpha: std::os::raw::c_int,
    ) -> *mut FzPixmap;
    fn fz_drop_pixmap(ctx: *mut std::ffi::c_void, pix: *mut FzPixmap);
    fn fz_clear_pixmap_with_value(
        ctx: *mut std::ffi::c_void,
        pix: *mut FzPixmap,
        value: std::os::raw::c_int,
    );
    fn fz_new_draw_device(ctx: *mut std::ffi::c_void, mat: FzMatrix) -> *mut FzDevice;
    fn fz_close_device(ctx: *mut std::ffi::c_void, dev: *mut FzDevice);
    fn fz_drop_device(ctx: *mut std::ffi::c_void, dev: *mut FzDevice);
    fn fz_run_page(
        ctx: *mut std::ffi::c_void,
        page: *mut PdfPage,
        dev: *mut FzDevice,
        ctm: FzMatrix,
        cookie: *mut std::ffi::c_void,
    );
    fn fz_new_buffer_from_pixmap_as_png(
        ctx: *mut std::ffi::c_void,
        pix: *mut FzPixmap,
        color_params: *const std::ffi::c_void,
    ) -> *mut FzBuffer;
    fn fz_buffer_extract(
        ctx: *mut std::ffi::c_void,
        buf: *mut FzBuffer,
        data: *mut *mut u8,
    ) -> usize;
    fn fz_drop_buffer(ctx: *mut std::ffi::c_void, buf: *mut FzBuffer);
    fn fz_device_rgb(ctx: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn fz_free(ctx: *mut std::ffi::c_void, ptr: *mut std::ffi::c_void);

    fn fz_new_font_from_file(
        ctx: *mut std::ffi::c_void,
        name: *const std::os::raw::c_char,
        path: *const std::os::raw::c_char,
        index: std::os::raw::c_int,
        use_glyph_bbox: std::os::raw::c_int,
    ) -> *mut FzFont;
    fn fz_drop_font(ctx: *mut std::ffi::c_void, font: *mut FzFont);
    fn pdf_add_cid_font(
        ctx: *mut std::ffi::c_void,
        doc: *mut PdfDocument,
        font: *mut FzFont,
    ) -> *mut PdfObj;
    fn pdf_to_name(ctx: *mut std::ffi::c_void, obj: *mut PdfObj) -> *const std::os::raw::c_char;
    fn pdf_dict_puts(
        ctx: *mut std::ffi::c_void,
        dict: *mut PdfObj,
        key: *const std::os::raw::c_char,
        val: *mut PdfObj,
    );
    fn pdf_lookup_page_obj(
        ctx: *mut std::ffi::c_void,
        doc: *mut PdfDocument,
        number: std::os::raw::c_int,
    ) -> *mut PdfObj;
    fn pdf_new_int(
        ctx: *mut std::ffi::c_void,
        i: i64,
    ) -> *mut PdfObj;
}

#[repr(C)]
struct PdfDocument {
    _opaque: [u8; 0],
}

#[repr(C)]
struct PdfPage {
    _opaque: [u8; 0],
}

#[repr(C)]
struct FzPixmap {
    _opaque: [u8; 0],
}

#[repr(C)]
struct FzBuffer {
    _opaque: [u8; 0],
}

#[repr(C)]
struct FzDevice {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FzRect {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FzMatrix {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
}

impl FzMatrix {
    fn scale(sx: f32, sy: f32) -> Self {
        Self { a: sx, b: 0.0, c: 0.0, d: sy, e: 0.0, f: 0.0 }
    }
}

#[repr(C)]
struct PdfWriteOptions {
    do_incremental: std::os::raw::c_int,
    do_pretty: std::os::raw::c_int,
    do_ascii: std::os::raw::c_int,
    do_compress: std::os::raw::c_int,
    do_compress_images: std::os::raw::c_int,
    do_compress_fonts: std::os::raw::c_int,
    do_decompress: std::os::raw::c_int,
    do_garbage: std::os::raw::c_int,
    do_linear: std::os::raw::c_int,
    do_clean: std::os::raw::c_int,
    do_sanitize: std::os::raw::c_int,
    do_appearance: std::os::raw::c_int,
    do_encrypt: std::os::raw::c_int,
    dont_regenerate_id: std::os::raw::c_int,
    permissions: std::os::raw::c_int,
    opwd_utf8: [std::os::raw::c_char; 128],
    upwd_utf8: [std::os::raw::c_char; 128],
    do_snapshot: std::os::raw::c_int,
    do_preserve_metadata: std::os::raw::c_int,
    do_use_objstms: std::os::raw::c_int,
    compression_effort: std::os::raw::c_int,
}

struct MupdfContext(*mut std::ffi::c_void);

impl MupdfContext {
    fn new() -> Result<Self, String> {
        let version = CString::new("1.25.2").unwrap();
        let ctx = unsafe { fz_new_context_imp(ptr::null(), ptr::null(), 64 << 20, version.as_ptr()) };
        if ctx.is_null() {
            return Err("Failed to create MuPDF context".into());
        }
        unsafe { fz_register_document_handlers(ctx) };
        Ok(Self(ctx))
    }

    fn ptr(&self) -> *mut std::ffi::c_void {
        self.0
    }
}

impl Drop for MupdfContext {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { fz_drop_context(self.0) };
            self.0 = ptr::null_mut();
        }
    }
}

struct PdfDoc {
    ptr: *mut PdfDocument,
    ctx: *mut std::ffi::c_void,
}

impl PdfDoc {
    fn open(ctx: &MupdfContext, path: &str) -> Result<Self, String> {
        let c_path = CString::new(path).map_err(|_| "Invalid path")?;
        let doc = unsafe { pdf_open_document(ctx.ptr(), c_path.as_ptr()) };
        if doc.is_null() {
            return Err(format!("Failed to open: {}", path));
        }
        Ok(Self { ptr: doc, ctx: ctx.ptr() })
    }

    fn create(ctx: &MupdfContext) -> Result<Self, String> {
        let doc = unsafe { pdf_create_document(ctx.ptr()) };
        if doc.is_null() {
            return Err("Failed to create PDF document".into());
        }
        Ok(Self { ptr: doc, ctx: ctx.ptr() })
    }

    fn page_count(&self) -> i32 {
        unsafe { pdf_count_pages(self.ctx, self.ptr) }
    }

    fn delete_page(&self, page: i32) {
        unsafe { pdf_delete_page(self.ctx, self.ptr, page) };
    }

    fn graft_page(&self, page_to: i32, src: &PdfDoc, page_from: i32) {
        unsafe { pdf_graft_page(self.ctx, self.ptr, page_to, src.ptr, page_from) };
    }

    fn save(&self, path: &str, opts: &PdfWriteOptions) -> Result<(), String> {
        let c_path = CString::new(path).map_err(|_| "Invalid path")?;
        unsafe { pdf_save_document(self.ctx, self.ptr, c_path.as_ptr(), opts) };
        Ok(())
    }

    fn load_page(&self, page_num: i32) -> Result<Page, String> {
        let page = unsafe { pdf_load_page(self.ctx, self.ptr, page_num) };
        if page.is_null() {
            return Err(format!("Failed to load page {}", page_num));
        }
        Ok(Page { ptr: page, ctx: self.ctx })
    }
}

impl Drop for PdfDoc {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { pdf_drop_document(self.ctx, self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

struct Page {
    ptr: *mut PdfPage,
    ctx: *mut std::ffi::c_void,
}

impl Page {
    fn bound(&self) -> FzRect {
        let mut bbox = FzRect { x0: 0.0, y0: 0.0, x1: 0.0, y1: 0.0 };
        unsafe { pdf_bound_page(self.ctx, self.ptr, &mut bbox) };
        bbox
    }

    fn render(&self, scale: f32) -> Result<Vec<u8>, String> {
        let bbox = self.bound();
        let w = ((bbox.x1 - bbox.x0) * scale).ceil() as i32;
        let h = ((bbox.y1 - bbox.y0) * scale).ceil() as i32;

        let ctm = FzMatrix::scale(scale, scale);
        let cs = unsafe { fz_device_rgb(self.ctx) };
        let pixmap = unsafe { fz_new_pixmap(self.ctx, cs, w, h, ptr::null_mut(), 0) };
        if pixmap.is_null() {
            return Err("Failed to create pixmap".into());
        }

        unsafe {
            fz_clear_pixmap_with_value(self.ctx, pixmap, 0xff);
            let dev = fz_new_draw_device(self.ctx, ctm);
            if dev.is_null() {
                fz_drop_pixmap(self.ctx, pixmap);
                return Err("Failed to create draw device".into());
            }
            let translate = FzMatrix { a: scale, b: 0.0, c: 0.0, d: scale, e: -bbox.x0 * scale, f: -bbox.y0 * scale };
            fz_run_page(self.ctx, self.ptr, dev, translate, ptr::null_mut());
            fz_close_device(self.ctx, dev);
            fz_drop_device(self.ctx, dev);

            let buf = fz_new_buffer_from_pixmap_as_png(self.ctx, pixmap, ptr::null());
            fz_drop_pixmap(self.ctx, pixmap);

            if buf.is_null() {
                return Err("Failed to encode PNG".into());
            }

            let mut data_ptr: *mut u8 = ptr::null_mut();
            let len = fz_buffer_extract(self.ctx, buf, &mut data_ptr);
            let data = std::slice::from_raw_parts(data_ptr, len).to_vec();
            fz_free(self.ctx, data_ptr as *mut std::ffi::c_void);
            fz_drop_buffer(self.ctx, buf);
            Ok(data)
        }
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { pdf_drop_page(self.ctx, self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

fn render_page(doc_path: &str, page_num: i32, scale: f32) -> Result<Vec<u8>, String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, doc_path)?;
    let page = doc.load_page(page_num)?;
    page.render(scale)
}

fn new_compress_opts() -> PdfWriteOptions {
    let mut opts: PdfWriteOptions = unsafe { std::mem::zeroed() };
    opts.do_compress = 1;
    opts.do_compress_images = 1;
    opts.do_compress_fonts = 1;
    opts.do_garbage = 3;
    opts.do_clean = 1;
    opts.do_sanitize = 1;
    opts.do_use_objstms = 1;
    opts.compression_effort = 80;
    opts
}

#[derive(Serialize, Deserialize)]
struct PageRange {
    start: i32,
    end: i32,
}

#[derive(Serialize, Deserialize)]
struct MergeSource {
    path: String,
    pages: Vec<PageRange>,
}

#[tauri::command]
fn delete_pages(input: String, output: String, pages: Vec<i32>) -> Result<i32, String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    let total = doc.page_count();
    let mut sorted = pages;
    sorted.sort_unstable();
    sorted.dedup();
    for &p in sorted.iter().rev() {
        if p >= 0 && p < total {
            doc.delete_page(p);
        }
    }
    doc.save(&output, &new_compress_opts())?;
    Ok(total - sorted.len() as i32)
}

#[tauri::command]
fn extract_pages(input: String, output: String, pages: Vec<i32>) -> Result<i32, String> {
    let ctx = MupdfContext::new()?;
    let src = PdfDoc::open(&ctx, &input)?;
    let total = src.page_count();
    let dst = PdfDoc::create(&ctx)?;

    let mut to = 0;
    for &p in &pages {
        if p >= 0 && p < total {
            dst.graft_page(to, &src, p);
            to += 1;
        }
    }

    dst.save(&output, &new_compress_opts())?;
    Ok(to)
}

#[tauri::command]
fn merge_pdfs(output: String, sources: Vec<MergeSource>) -> Result<i32, String> {
    let ctx = MupdfContext::new()?;
    if sources.is_empty() {
        return Err("No sources provided".into());
    }

    let dst = PdfDoc::create(&ctx)?;
    let mut total_pages = 0i32;

    for source in &sources {
        let doc = PdfDoc::open(&ctx, &source.path)?;
        let count = doc.page_count();
        if source.pages.is_empty() {
            for p in 0..count {
                dst.graft_page(total_pages, &doc, p);
                total_pages += 1;
            }
        } else {
            for range in &source.pages {
                for p in range.start..=range.end.min(count - 1) {
                    dst.graft_page(total_pages, &doc, p);
                    total_pages += 1;
                }
            }
        }
    }

    dst.save(&output, &new_compress_opts())?;
    Ok(total_pages)
}

#[tauri::command]
fn compress_pdf(input: String, output: String, effort: i32) -> Result<String, String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;

    let mut opts = new_compress_opts();
    opts.compression_effort = effort.clamp(1, 100);

    doc.save(&output, &opts)?;

    let input_size = std::fs::metadata(&input).map(|m| m.len()).unwrap_or(0);
    let output_size = std::fs::metadata(&output).map(|m| m.len()).unwrap_or(0);
    let ratio = if input_size > 0 {
        format!("{:.1}%", (output_size as f64 / input_size as f64) * 100.0)
    } else {
        "N/A".to_string()
    };

    Ok(format!("{} -> {} ({})", input_size, output_size, ratio))
}

#[tauri::command]
fn pdf_info(input: String) -> Result<i32, String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    Ok(doc.page_count())
}

#[derive(Serialize, Deserialize)]
struct PdfOpenResult {
    page_count: i32,
    page_width: f32,
    page_height: f32,
}

#[tauri::command]
fn pdf_open(input: String) -> Result<PdfOpenResult, String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    let count = doc.page_count();
    let page = doc.load_page(0)?;
    let bbox = page.bound();
    Ok(PdfOpenResult {
        page_count: count,
        page_width: bbox.x1 - bbox.x0,
        page_height: bbox.y1 - bbox.y0,
    })
}

#[tauri::command]
fn pdf_page_info(input: String, page: i32) -> Result<String, String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    let p = doc.load_page(page)?;
    let bbox = p.bound();
    Ok(serde_json::json!({
        "width": bbox.x1 - bbox.x0,
        "height": bbox.y1 - bbox.y0,
    }).to_string())
}

const PDF_ANNOT_FREE_TEXT: std::os::raw::c_int = 38;

#[derive(Serialize, Deserialize)]
struct TextAnnotation {
    id: i32,
    page: i32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
    font_size: f32,
    color: [f32; 3],
}

#[derive(Serialize, Deserialize)]
struct TextAddParams {
    input: String,
    page: i32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
    font_size: f32,
    color: [f32; 3],
}

fn annot_id(ctx: *mut std::ffi::c_void, annot: *mut PdfAnnot) -> i32 {
    unsafe {
        let obj = pdf_annot_obj(ctx, annot);
        if obj.is_null() { 0 } else { pdf_to_num(ctx, obj) }
    }
}

fn contains_cjk(text: &str) -> bool {
    text.chars().any(|c| {
        let cp = c as u32;
        (0x4E00..=0x9FFF).contains(&cp)
            || (0x3400..=0x4DBF).contains(&cp)
            || (0x3000..=0x303F).contains(&cp)
            || (0xAC00..=0xD7AF).contains(&cp)
            || (0x3040..=0x309F).contains(&cp)
            || (0x30A0..=0x30FF).contains(&cp)
    })
}

fn get_cjk_font_path() -> Option<&'static str> {
    #[cfg(target_os = "macos")]
    {
        if std::path::Path::new("/System/Library/Fonts/PingFang.ttc").exists() {
            return Some("/System/Library/Fonts/PingFang.ttc");
        }
        if std::path::Path::new("/System/Library/Fonts/STHeiti Light.ttc").exists() {
            return Some("/System/Library/Fonts/STHeiti Light.ttc");
        }
    }
    #[cfg(target_os = "windows")]
    {
        if std::path::Path::new("C:\\Windows\\Fonts\\msyh.ttc").exists() {
            return Some("C:\\Windows\\Fonts\\msyh.ttc");
        }
        if std::path::Path::new("C:\\Windows\\Fonts\\simsun.ttc").exists() {
            return Some("C:\\Windows\\Fonts\\simsun.ttc");
        }
    }
    None
}

struct FontHandle {
    font: *mut FzFont,
    ctx: *mut std::ffi::c_void,
}

impl Drop for FontHandle {
    fn drop(&mut self) {
        if !self.font.is_null() {
            unsafe { fz_drop_font(self.ctx, self.font) };
        }
    }
}

fn load_cid_font(ctx: *mut std::ffi::c_void, doc: *mut PdfDocument) -> Result<(CString, FontHandle), String> {
    let font_path = get_cjk_font_path().ok_or("No CJK font found on system")?;
    let c_name = CString::new("CJKFont").unwrap();
    let c_path = CString::new(font_path).map_err(|_| "Invalid font path")?;

    let font = unsafe { fz_new_font_from_file(ctx, c_name.as_ptr(), c_path.as_ptr(), 0, 1) };
    if font.is_null() {
        return Err("Failed to load CJK font".into());
    }

    let font_obj = unsafe { pdf_add_cid_font(ctx, doc, font) };
    if font_obj.is_null() {
        unsafe { fz_drop_font(ctx, font) };
        return Err("Failed to add CID font to document".into());
    }

    let font_name_ptr = unsafe { pdf_to_name(ctx, font_obj) };
    if font_name_ptr.is_null() {
        unsafe { fz_drop_font(ctx, font) };
        return Err("Failed to get font name".into());
    }

    let font_name = unsafe { CStr::from_ptr(font_name_ptr) }.to_string_lossy().into_owned();
    let c_font_name = CString::new(font_name).unwrap();

    Ok((c_font_name, FontHandle { font, ctx }))
}

#[tauri::command]
fn text_add(params: String) -> Result<i32, String> {
    let p: TextAddParams = serde_json::from_str(&params).map_err(|e| e.to_string())?;
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &p.input)?;
    let page = doc.load_page(p.page)?;

    let annot = unsafe { pdf_create_annot(ctx.ptr(), page.ptr, PDF_ANNOT_FREE_TEXT) };
    if annot.is_null() {
        return Err("Failed to create annotation".into());
    }

    let rect = FzRect {
        x0: p.x,
        y0: p.y,
        x1: p.x + p.width,
        y1: p.y + p.height,
    };
    let c_text = CString::new(p.text.as_str()).map_err(|_| "Invalid text")?;
    let color = [p.color[0], p.color[1], p.color[2]];

    let c_font = if contains_cjk(&p.text) {
        match load_cid_font(ctx.ptr(), doc.ptr) {
            Ok((name, _handle)) => name,
            Err(_) => CString::new("Helv").unwrap(),
        }
    } else {
        CString::new("Helv").unwrap()
    };

    unsafe {
        pdf_set_annot_rect(ctx.ptr(), annot, rect);
        pdf_set_annot_contents(ctx.ptr(), annot, c_text.as_ptr());
        pdf_set_annot_default_appearance(ctx.ptr(), annot, c_font.as_ptr(), p.font_size, 3, color.as_ptr());
        pdf_update_annot(ctx.ptr(), annot);
    }

    let id = annot_id(ctx.ptr(), annot);
    drop(page);
    doc.save(&p.input, &new_compress_opts())?;
    Ok(id)
}

#[tauri::command]
fn text_edit(input: String, page: i32, id: i32, text: String, font_size: f32, color: Vec<f32>) -> Result<(), String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    let pg = doc.load_page(page)?;

    unsafe {
        let mut annot = pdf_first_annot(ctx.ptr(), pg.ptr);
        while !annot.is_null() {
            if pdf_annot_type(ctx.ptr(), annot) == PDF_ANNOT_FREE_TEXT && annot_id(ctx.ptr(), annot) == id {
                let c_text = CString::new(text.as_str()).map_err(|_| "Invalid text")?;
                let clr = [color.get(0).copied().unwrap_or(0.0), color.get(1).copied().unwrap_or(0.0), color.get(2).copied().unwrap_or(0.0)];

                let c_font = if contains_cjk(&text) {
                    match load_cid_font(ctx.ptr(), doc.ptr) {
                        Ok((name, _handle)) => name,
                        Err(_) => CString::new("Helv").unwrap(),
                    }
                } else {
                    CString::new("Helv").unwrap()
                };

                pdf_set_annot_contents(ctx.ptr(), annot, c_text.as_ptr());
                pdf_set_annot_default_appearance(ctx.ptr(), annot, c_font.as_ptr(), font_size, 3, clr.as_ptr());
                pdf_update_annot(ctx.ptr(), annot);
                drop(pg);
                doc.save(&input, &new_compress_opts())?;
                return Ok(());
            }
            annot = pdf_next_annot(ctx.ptr(), annot);
        }
    }
    Err("Annotation not found".into())
}

#[tauri::command]
fn text_delete(input: String, page: i32, id: i32) -> Result<(), String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    let pg = doc.load_page(page)?;

    unsafe {
        let mut annot = pdf_first_annot(ctx.ptr(), pg.ptr);
        while !annot.is_null() {
            if pdf_annot_type(ctx.ptr(), annot) == PDF_ANNOT_FREE_TEXT && annot_id(ctx.ptr(), annot) == id {
                pdf_delete_annot(ctx.ptr(), pg.ptr, annot);
                drop(pg);
                doc.save(&input, &new_compress_opts())?;
                return Ok(());
            }
            annot = pdf_next_annot(ctx.ptr(), annot);
        }
    }
    Err("Annotation not found".into())
}

#[tauri::command]
fn text_list(input: String, page: i32) -> Result<String, String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    let pg = doc.load_page(page)?;

    let mut annotations = Vec::new();

    unsafe {
        let mut annot = pdf_first_annot(ctx.ptr(), pg.ptr);
        while !annot.is_null() {
            if pdf_annot_type(ctx.ptr(), annot) == PDF_ANNOT_FREE_TEXT {
                let rect = pdf_annot_rect(ctx.ptr(), annot);
                let contents_ptr = pdf_annot_contents(ctx.ptr(), annot);
                let text = if contents_ptr.is_null() {
                    String::new()
                } else {
                    CStr::from_ptr(contents_ptr).to_string_lossy().into_owned()
                };

                let mut font_ptr: *const std::os::raw::c_char = ptr::null();
                let mut size: f32 = 12.0;
                let mut n: std::os::raw::c_int = 0;
                let mut color = [0.0f32; 4];
                pdf_annot_default_appearance(ctx.ptr(), annot, &mut font_ptr, &mut size, &mut n, color.as_mut_ptr());

                annotations.push(TextAnnotation {
                    id: annot_id(ctx.ptr(), annot),
                    page,
                    x: rect.x0,
                    y: rect.y0,
                    width: rect.x1 - rect.x0,
                    height: rect.y1 - rect.y0,
                    text,
                    font_size: size,
                    color: [color[0], color[1], color[2]],
                });
            }
            annot = pdf_next_annot(ctx.ptr(), annot);
        }
    }

    serde_json::to_string(&annotations).map_err(|e| e.to_string())
}

#[tauri::command]
fn rotate_page(input: String, page: i32, rotation: i32) -> Result<(), String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    let page_obj = unsafe { pdf_lookup_page_obj(ctx.ptr(), doc.ptr, page) };
    if page_obj.is_null() {
        return Err("Failed to get page object".into());
    }
    let c_key = CString::new("Rotate").unwrap();
    let rot_val = unsafe { pdf_new_int(ctx.ptr(), rotation as i64) };
    unsafe { pdf_dict_puts(ctx.ptr(), page_obj, c_key.as_ptr(), rot_val) };
    doc.save(&input, &new_compress_opts())?;
    Ok(())
}

#[tauri::command]
fn save_as(input: String, output: String) -> Result<(), String> {
    let ctx = MupdfContext::new()?;
    let doc = PdfDoc::open(&ctx, &input)?;
    let opts = new_compress_opts();
    doc.save(&output, &opts)?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            delete_pages,
            extract_pages,
            merge_pdfs,
            compress_pdf,
            pdf_info,
            pdf_open,
            pdf_page_info,
            text_add,
            text_edit,
            text_delete,
            text_list,
            rotate_page,
            save_as,
        ])
        .register_asynchronous_uri_scheme_protocol("pdfpage", |_ctx, request, responder| {
            let url = request.uri().to_string();
            std::thread::spawn(move || {
                let response = parse_pdf_url(&url)
                    .and_then(|(path, page_num, scale)| render_page(&path, page_num, scale));

                match response {
                    Ok(data) => {
                        responder.respond(
                            http::Response::builder()
                                .header("Content-Type", "image/png")
                                .header("Access-Control-Allow-Origin", "*")
                                .body(data)
                                .unwrap(),
                        );
                    }
                    Err(e) => {
                        responder.respond(
                            http::Response::builder()
                                .status(500)
                                .header("Access-Control-Allow-Origin", "*")
                                .body(e.into_bytes())
                                .unwrap(),
                        );
                    }
                }
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn parse_pdf_url(url: &str) -> Result<(String, i32, f32), String> {
    let stripped = url.strip_prefix("pdfpage://localhost/").unwrap_or(url);
    let (path_part, query) = match stripped.split_once('?') {
        Some((p, q)) => (p, q),
        None => (stripped, ""),
    };

    let page_str = path_part.strip_prefix("page/").ok_or("Invalid page path")?;
    let page_num: i32 = page_str.parse().map_err(|_| "Invalid page number")?;

    let mut scale = 1.0f32;
    let mut doc_path = String::new();
    for param in query.split('&').filter(|s| !s.is_empty()) {
        if let Some(v) = param.strip_prefix("scale=") {
            scale = v.parse().unwrap_or(1.0);
        } else if let Some(v) = param.strip_prefix("doc=") {
            doc_path = percent_decode(v);
        }
    }

    if doc_path.is_empty() {
        return Err("Missing doc parameter".into());
    }

    Ok((doc_path, page_num, scale))
}

fn percent_decode(s: &str) -> String {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(
                std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""),
                16,
            ) {
                result.push(byte);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(result).unwrap_or_default()
}
