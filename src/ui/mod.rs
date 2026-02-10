//! UI module for the CatShare application
//! Optimized for a high-performance "speed" feel with Electric Indigo & Cyan palette

pub mod app;

pub use app::CatShareApp;

use crate::i18n::{t, Language, TranslationKey};
use egui::{Color32, Context, Pos2, Rect, RichText, Rounding, Stroke, Ui, Vec2};

/// Modern high-performance color palette
/// Optimized for speed with softer Electric Blue & Cyan accents for dynamic energy
pub struct Colors;

impl Colors {
    // Primary Brand - Softer Electric Blue
    pub const PRIMARY: Color32 = Color32::from_rgb(79, 114, 205); // Softer Electric Blue
    pub const PRIMARY_LIGHT: Color32 = Color32::from_rgb(129, 163, 223);
    pub const PRIMARY_GLOW: Color32 = Color32::from_rgba_premultiplied(79, 114, 205, 40);

    // Speed Accent - Softer Cyan for high-speed feel
    pub const ACCENT: Color32 = Color32::from_rgb(56, 189, 248); // Softer Cyan
    pub const ACCENT_LIGHT: Color32 = Color32::from_rgb(148, 210, 245);

    // Status - Softer for comfortable viewing
    pub const SUCCESS: Color32 = Color32::from_rgb(52, 178, 125); // Softer Emerald
    pub const ERROR: Color32 = Color32::from_rgb(220, 70, 70); // Softer Red
    pub const WARNING: Color32 = Color32::from_rgb(245, 158, 11); // Amber (original)
    pub const INFO: Color32 = Color32::from_rgb(96, 165, 250); // Softer Cyan

    // Neutral / Background - Lighter for speed perception
    pub const BACKGROUND: Color32 = Color32::from_rgb(249, 250, 251); // Lighter background
    pub const SURFACE: Color32 = Color32::from_rgb(255, 255, 255); // Pure white
    pub const SURFACE_SUBTLE: Color32 = Color32::from_rgb(248, 250, 251); // Very light blue-grey

    // Text - Optimized for quick readability
    pub const TEXT_HEADING: Color32 = Color32::from_rgb(30, 41, 59); // Dark for contrast
    pub const TEXT_BODY: Color32 = Color32::from_rgb(55, 65, 81); // Softer dark
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(107, 114, 128); // Softer muted
    pub const TEXT_ON_PRIMARY: Color32 = Color32::WHITE;

    // Borders & Shadows - Subtle for minimal distraction
    pub const BORDER: Color32 = Color32::from_rgb(229, 231, 235); // Softer grey
    pub const SHADOW: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 6);
}

/// Setup global custom theme for egui context to ensure Speed-focused palette is applied everywhere
pub fn setup_custom_theme(ctx: &Context, theme: &str) {
    let mut visuals = if theme == "dark_theme" {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };

    // Determine colors based on theme
    let colors: Box<dyn ThemeColors> = match theme {
        "orange_cat" => Box::new(OrangeCatThemeColors),
        "classic_blue" => Box::new(ClassicBlueColors),
        "dark_theme" => Box::new(DarkThemeColors),
        "light_theme" => Box::new(LightThemeColors),
        _ => Box::new(ElectricIndigoColors), // Default to electric indigo
    };

    // Background and Panels - Optimized for speed perception
    visuals.panel_fill = colors.background();
    visuals.window_fill = colors.surface();
    visuals.extreme_bg_color = colors.surface_subtle();
    visuals.faint_bg_color = colors.surface_subtle();

    // Widget styling (Buttons, Checkboxes, etc.) - Enhanced for speed
    let rounding = Rounding::same(Sizes::RADIUS_SM); // Smaller radius for sharper, faster feel

    // Non-interactive state
    visuals.widgets.noninteractive.bg_fill = colors.surface();
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, colors.border());
    visuals.widgets.noninteractive.rounding = rounding;

    // Inactive state (default buttons)
    visuals.widgets.inactive.bg_fill = colors.surface();
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, colors.border());
    visuals.widgets.inactive.rounding = rounding;

    // Hovered state - Enhanced for speed feedback
    visuals.widgets.hovered.bg_fill = colors.surface_subtle();
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5, colors.accent()); // Use accent for hover
    visuals.widgets.hovered.rounding = rounding;

    // Active/Pressed state - More dynamic for speed
    visuals.widgets.active.bg_fill = colors.primary_glow();
    visuals.widgets.active.fg_stroke = Stroke::new(2.0, colors.accent()); // Use accent for active
    visuals.widgets.active.rounding = rounding;

    // Selection and Focus - Speed-focused
    visuals.selection.bg_fill = colors.accent();
    visuals.selection.stroke = Stroke::new(1.0, colors.text_on_primary());

    ctx.set_visuals(visuals);

    // Adjust spacing for a "faster" and more compact feel
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(6.0, 6.0); // Tighter spacing for speed
    style.spacing.button_padding = Vec2::new(8.0, 5.0); // More compact buttons
    ctx.set_style(style);
}

/// Color theme implementations
pub trait ThemeColors {
    fn primary(&self) -> Color32;
    fn primary_light(&self) -> Color32;
    fn primary_glow(&self) -> Color32;
    fn accent(&self) -> Color32;
    fn accent_light(&self) -> Color32;
    fn success(&self) -> Color32;
    fn error(&self) -> Color32;
    fn warning(&self) -> Color32;
    fn info(&self) -> Color32;
    fn background(&self) -> Color32;
    fn surface(&self) -> Color32;
    fn surface_subtle(&self) -> Color32;
    fn text_heading(&self) -> Color32;
    fn text_body(&self) -> Color32;
    fn text_muted(&self) -> Color32;
    fn text_on_primary(&self) -> Color32;
    fn border(&self) -> Color32;
    fn shadow(&self) -> Color32;
}

pub struct ElectricIndigoColors;

impl ThemeColors for ElectricIndigoColors {
    fn primary(&self) -> Color32 { Color32::from_rgb(79, 114, 205) } // Softer Electric Blue
    fn primary_light(&self) -> Color32 { Color32::from_rgb(129, 163, 223) }
    fn primary_glow(&self) -> Color32 { Color32::from_rgba_premultiplied(79, 114, 205, 40) }
    fn accent(&self) -> Color32 { Color32::from_rgb(56, 189, 248) } // Softer Cyan
    fn accent_light(&self) -> Color32 { Color32::from_rgb(148, 210, 245) }
    fn success(&self) -> Color32 { Color32::from_rgb(52, 178, 125) } // Softer Emerald
    fn error(&self) -> Color32 { Color32::from_rgb(220, 70, 70) } // Softer Red
    fn warning(&self) -> Color32 { Color32::from_rgb(245, 158, 11) } // Amber
    fn info(&self) -> Color32 { Color32::from_rgb(96, 165, 250) } // Softer Cyan
    fn background(&self) -> Color32 { Color32::from_rgb(249, 250, 251) } // Lighter background
    fn surface(&self) -> Color32 { Color32::from_rgb(255, 255, 255) } // Pure white
    fn surface_subtle(&self) -> Color32 { Color32::from_rgb(248, 250, 251) } // Very light blue-grey
    fn text_heading(&self) -> Color32 { Color32::from_rgb(30, 41, 59) } // Dark for contrast
    fn text_body(&self) -> Color32 { Color32::from_rgb(55, 65, 81) } // Softer dark
    fn text_muted(&self) -> Color32 { Color32::from_rgb(107, 114, 128) } // Softer muted
    fn text_on_primary(&self) -> Color32 { Color32::WHITE }
    fn border(&self) -> Color32 { Color32::from_rgb(229, 231, 235) } // Softer grey
    fn shadow(&self) -> Color32 { Color32::from_rgba_premultiplied(0, 0, 0, 6) }
}

pub struct ClassicBlueColors;

impl ThemeColors for ClassicBlueColors {
    fn primary(&self) -> Color32 { Color32::from_rgb(59, 130, 246) } // Classic blue
    fn primary_light(&self) -> Color32 { Color32::from_rgb(99, 158, 246) }
    fn primary_glow(&self) -> Color32 { Color32::from_rgba_premultiplied(59, 130, 246, 40) }
    fn accent(&self) -> Color32 { Color32::from_rgb(100, 200, 255) } // Light blue accent
    fn accent_light(&self) -> Color32 { Color32::from_rgb(150, 220, 255) }
    fn success(&self) -> Color32 { Color32::from_rgb(34, 197, 94) } // Green
    fn error(&self) -> Color32 { Color32::from_rgb(239, 68, 68) } // Red
    fn warning(&self) -> Color32 { Color32::from_rgb(245, 158, 11) } // Amber
    fn info(&self) -> Color32 { Color32::from_rgb(59, 130, 246) } // Blue
    fn background(&self) -> Color32 { Color32::from_rgb(241, 245, 249) } // Light blue background
    fn surface(&self) -> Color32 { Color32::from_rgb(255, 255, 255) } // White
    fn surface_subtle(&self) -> Color32 { Color32::from_rgb(241, 245, 249) } // Light blue
    fn text_heading(&self) -> Color32 { Color32::from_rgb(15, 23, 42) } // Dark
    fn text_body(&self) -> Color32 { Color32::from_rgb(30, 41, 59) } // Darker
    fn text_muted(&self) -> Color32 { Color32::from_rgb(100, 116, 139) } // Muted blue
    fn text_on_primary(&self) -> Color32 { Color32::WHITE }
    fn border(&self) -> Color32 { Color32::from_rgb(219, 234, 254) } // Light blue border
    fn shadow(&self) -> Color32 { Color32::from_rgba_premultiplied(0, 0, 0, 8) }
}

pub struct DarkThemeColors;

impl ThemeColors for DarkThemeColors {
    fn primary(&self) -> Color32 { Color32::from_rgb(99, 102, 241) } // Indigo
    fn primary_light(&self) -> Color32 { Color32::from_rgb(129, 140, 248) }
    fn primary_glow(&self) -> Color32 { Color32::from_rgba_premultiplied(99, 102, 241, 40) }
    fn accent(&self) -> Color32 { Color32::from_rgb(139, 92, 246) } // Purple
    fn accent_light(&self) -> Color32 { Color32::from_rgb(168, 85, 247) }
    fn success(&self) -> Color32 { Color32::from_rgb(74, 222, 128) } // Bright green
    fn error(&self) -> Color32 { Color32::from_rgb(248, 113, 113) } // Light red
    fn warning(&self) -> Color32 { Color32::from_rgb(251, 191, 36) } // Yellow
    fn info(&self) -> Color32 { Color32::from_rgb(59, 130, 246) } // Blue
    fn background(&self) -> Color32 { Color32::from_rgb(15, 15, 15) } // Almost black background
    fn surface(&self) -> Color32 { Color32::from_rgb(30, 30, 30) } // Dark surface
    fn surface_subtle(&self) -> Color32 { Color32::from_rgb(40, 40, 40) } // Subtle surface
    fn text_heading(&self) -> Color32 { Color32::from_rgb(240, 240, 240) } // Bright white-gray
    fn text_body(&self) -> Color32 { Color32::from_rgb(220, 220, 220) } // Light gray
    fn text_muted(&self) -> Color32 { Color32::from_rgb(160, 160, 160) } // Muted gray
    fn text_on_primary(&self) -> Color32 { Color32::WHITE }
    fn border(&self) -> Color32 { Color32::from_rgb(60, 60, 60) } // Dark gray border
    fn shadow(&self) -> Color32 { Color32::from_rgba_premultiplied(0, 0, 0, 50) }
}

pub struct LightThemeColors;

impl ThemeColors for LightThemeColors {
    fn primary(&self) -> Color32 { Color32::from_rgb(107, 114, 128) } // Gray
    fn primary_light(&self) -> Color32 { Color32::from_rgb(156, 163, 175) }
    fn primary_glow(&self) -> Color32 { Color32::from_rgba_premultiplied(107, 114, 128, 40) }
    fn accent(&self) -> Color32 { Color32::from_rgb(156, 163, 175) } // Gray accent
    fn accent_light(&self) -> Color32 { Color32::from_rgb(209, 213, 219) }
    fn success(&self) -> Color32 { Color32::from_rgb(34, 197, 94) } // Green
    fn error(&self) -> Color32 { Color32::from_rgb(239, 68, 68) } // Red
    fn warning(&self) -> Color32 { Color32::from_rgb(245, 158, 11) } // Amber
    fn info(&self) -> Color32 { Color32::from_rgb(59, 130, 246) } // Blue
    fn background(&self) -> Color32 { Color32::from_rgb(249, 250, 251) } // Light gray
    fn surface(&self) -> Color32 { Color32::from_rgb(255, 255, 255) } // White
    fn surface_subtle(&self) -> Color32 { Color32::from_rgb(249, 250, 251) } // Light gray
    fn text_heading(&self) -> Color32 { Color32::from_rgb(17, 24, 39) } // Dark
    fn text_body(&self) -> Color32 { Color32::from_rgb(55, 65, 81) } // Medium gray
    fn text_muted(&self) -> Color32 { Color32::from_rgb(107, 114, 128) } // Gray
    fn text_on_primary(&self) -> Color32 { Color32::WHITE }
    fn border(&self) -> Color32 { Color32::from_rgb(229, 231, 235) } // Light gray border
    fn shadow(&self) -> Color32 { Color32::from_rgba_premultiplied(0, 0, 0, 5) }
}

pub struct OrangeCatThemeColors;

impl ThemeColors for OrangeCatThemeColors {
    fn primary(&self) -> Color32 { Color32::from_rgb(255, 140, 0) } // Vibrant orange
    fn primary_light(&self) -> Color32 { Color32::from_rgb(255, 165, 79) } // Lighter orange
    fn primary_glow(&self) -> Color32 { Color32::from_rgba_premultiplied(255, 140, 0, 40) }
    fn accent(&self) -> Color32 { Color32::from_rgb(255, 215, 0) } // Gold/yellow accent
    fn accent_light(&self) -> Color32 { Color32::from_rgb(255, 235, 107) } // Light gold
    fn success(&self) -> Color32 { Color32::from_rgb(34, 197, 94) } // Green
    fn error(&self) -> Color32 { Color32::from_rgb(239, 68, 68) } // Red
    fn warning(&self) -> Color32 { Color32::from_rgb(245, 158, 11) } // Amber
    fn info(&self) -> Color32 { Color32::from_rgb(59, 130, 246) } // Blue
    fn background(&self) -> Color32 { Color32::from_rgb(255, 248, 235) } // Warm cream background
    fn surface(&self) -> Color32 { Color32::from_rgb(255, 255, 255) } // White
    fn surface_subtle(&self) -> Color32 { Color32::from_rgb(255, 248, 235) } // Same as background
    fn text_heading(&self) -> Color32 { Color32::from_rgb(100, 65, 16) } // Dark brown/orange
    fn text_body(&self) -> Color32 { Color32::from_rgb(120, 75, 25) } // Medium brown/orange
    fn text_muted(&self) -> Color32 { Color32::from_rgb(160, 120, 60) } // Muted orange-brown
    fn text_on_primary(&self) -> Color32 { Color32::WHITE }
    fn border(&self) -> Color32 { Color32::from_rgb(255, 215, 175) } // Light orange border
    fn shadow(&self) -> Color32 { Color32::from_rgba_premultiplied(0, 0, 0, 8) }
}

pub struct Sizes;

impl Sizes {
    pub const RADIUS_SM: f32 = 4.0; // Sharper, more compact for speed
    pub const RADIUS_MD: f32 = 8.0;  // More compact for speed
    pub const RADIUS_LG: f32 = 16.0; // More compact for speed

    pub const PADDING_SM: f32 = 6.0; // Tighter spacing for speed
    pub const PADDING_MD: f32 = 12.0; // Tighter spacing for speed
    pub const PADDING_LG: f32 = 18.0; // Tighter spacing for speed

    pub const BUTTON_HEIGHT: f32 = 40.0; // More compact for speed
}

/// Primary action button with a sleek "fast" gradient-like feel
pub fn primary_button(ui: &mut Ui, text: &str) -> egui::Response {
    let desired_size = Vec2::new(ui.available_width().min(240.0), Sizes::BUTTON_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let hovered = response.hovered();
        let active = response.is_pointer_button_down_on();

        // Background color logic - Enhanced for speed but softer
        let bg = if active {
            Color32::from_rgb(59, 95, 175) // Softer deeper electric blue when pressed
        } else if hovered {
            Color32::from_rgb(79, 114, 205) // Softer electric blue when hovered
        } else {
            Color32::from_rgb(69, 104, 195) // Softer primary for static state
        };

        // Draw dynamic glow when hovered for speed effect
        if hovered {
            painter.rect_filled(
                rect.translate(Vec2::new(0.0, 2.0)),
                Rounding::same(Sizes::RADIUS_SM),
                Color32::from_rgba_premultiplied(79, 114, 205, 80), // Softer glow
            );
        }

        painter.rect_filled(rect, Rounding::same(Sizes::RADIUS_SM), bg);

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(15.0),
            Colors::TEXT_ON_PRIMARY,
        );

        if hovered {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }

    response
}

/// Secondary button for neutral actions - Enhanced for speed
pub fn secondary_button(ui: &mut Ui, text: &str) -> egui::Response {
    let desired_size = Vec2::new(ui.available_width().min(120.0), Sizes::BUTTON_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let hovered = response.hovered();

        let bg = if hovered {
            Color32::from_rgb(245, 246, 248) // Softer hover background
        } else {
            Colors::SURFACE
        };
        let stroke_color = if hovered {
            Color32::from_rgb(100, 150, 220) // Softer accent color for speed feel
        } else {
            Colors::BORDER
        };

        painter.rect_filled(rect, Rounding::same(Sizes::RADIUS_SM), bg);
        painter.rect_stroke(
            rect,
            Rounding::same(Sizes::RADIUS_SM),
            Stroke::new(1.5, stroke_color),
        );

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(14.0),
            if hovered {
                Color32::from_rgb(100, 150, 220) // Softer accent color for speed feel
            } else {
                Colors::TEXT_BODY
            },
        );
    }

    response
}

/// Danger action button - Enhanced for speed
pub fn danger_button(ui: &mut Ui, text: &str) -> egui::Response {
    let desired_size = Vec2::new(ui.available_width().min(120.0), Sizes::BUTTON_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let hovered = response.hovered();

        let bg = if hovered {
            Color32::from_rgb(200, 60, 60) // Softer intense red for speed
        } else {
            Colors::ERROR
        };

        // Add subtle glow effect for danger button
        if hovered {
            painter.rect_filled(
                rect.translate(Vec2::new(0.0, 1.0)),
                Rounding::same(Sizes::RADIUS_SM),
                Color32::from_rgba_premultiplied(220, 70, 70, 30), // Softer subtle glow
            );
        }

        painter.rect_filled(rect, Rounding::same(Sizes::RADIUS_SM), bg);

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(14.0),
            Colors::TEXT_ON_PRIMARY,
        );
    }

    response
}

/// Small icon-only button
pub fn icon_button(ui: &mut Ui, icon: &str, tooltip: &str) -> egui::Response {
    let (rect, response) = ui.allocate_at_least(Vec2::splat(32.0), egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if response.hovered() {
            painter.circle_filled(rect.center(), 16.0, Colors::SURFACE_SUBTLE);
        }
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::proportional(18.0),
            Colors::TEXT_BODY,
        );
    }

    response.on_hover_text(tooltip)
}

/// Content card with modern soft shadow
pub fn card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::none()
        .fill(Colors::SURFACE)
        .rounding(Rounding::same(Sizes::RADIUS_MD))
        .inner_margin(Sizes::PADDING_MD)
        .shadow(egui::epaint::Shadow {
            offset: Vec2::new(0.0, 4.0),
            blur: 12.0,
            spread: 0.0,
            color: Colors::SHADOW,
        })
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add_contents(ui);
        });
}

/// Section header with accent line
pub fn section_header(ui: &mut Ui, icon: &str, text: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(icon).size(18.0).color(Colors::ACCENT));
        ui.add_space(Sizes::PADDING_SM);
        ui.label(
            RichText::new(text)
                .size(16.0)
                .strong()
                .color(Colors::TEXT_HEADING),
        );
    });
    ui.add_space(Sizes::PADDING_SM);
}

/// High-performance progress bar with dynamic speed accent and glow
pub fn progress_bar_with_text(ui: &mut Ui, progress: f32, text: &str) {
    let progress = progress.clamp(0.0, 1.0);
    let height = 24.0;
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), height),
        egui::Sense::hover(),
    );

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // Background track - More subtle for speed focus
        painter.rect_filled(rect, Rounding::same(height / 2.0), Color32::from_rgb(245, 247, 249));

        if progress > 0.0 {
            let progress_width = rect.width() * progress;
            let progress_rect =
                Rect::from_min_size(rect.min, Vec2::new(progress_width, rect.height()));

            // Dynamic Speed Cyan Gradient Effect - Enhanced for speed but softer
            painter.rect_filled(progress_rect, Rounding::same(height / 2.0), Colors::ACCENT);

            // Animated "speed shine" overlay for dynamic feel
            let shine_rect = Rect::from_min_max(
                Pos2::new(progress_rect.min.x + progress_width * 0.7, progress_rect.min.y),
                Pos2::new(progress_rect.max.x, progress_rect.min.y + height / 2.0),
            );
            painter.rect_filled(
                shine_rect,
                Rounding::same(height / 2.0),
                Color32::from_rgba_unmultiplied(255, 255, 255, 60), // Softer shine
            );
            
            // Additional speed effect: gradient from start to end
            if progress > 0.05 {
                let gradient_start = Pos2::new(progress_rect.min.x, progress_rect.min.y);
                let gradient_end = Pos2::new(progress_rect.min.x + progress_width * 0.3, progress_rect.min.y);
                
                painter.line_segment(
                    [gradient_start, gradient_end],
                    egui::Stroke::new(1.5, Color32::from_rgba_premultiplied(255, 255, 255, 80)),
                );
            }
        }

        // Percentage text - Enhanced for speed readability
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(12.0),
            if progress > 0.5 {
                Colors::TEXT_ON_PRIMARY
            } else {
                Color32::from_rgb(75, 85, 99) // Softer contrast for better readability
            },
        );
    }
}

/// Badge for status display - Enhanced for speed
pub fn status_badge_with_icon(ui: &mut Ui, icon: &str, text: &str, color: Color32) {
    egui::Frame::none()
        .fill(color.linear_multiply(0.15)) // Slightly more prominent for speed
        .rounding(Rounding::same(Sizes::RADIUS_SM))
        .stroke(egui::Stroke::new(1.0, color.linear_multiply(0.3))) // Subtle border
        .inner_margin(Vec2::new(10.0, 5.0)) // Slightly larger padding
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 5.0; // Slightly more space
                ui.label(RichText::new(icon).color(color).size(13.0)); // Slightly larger icon
                ui.label(RichText::new(text).color(color).size(13.0).strong());
            });
        });
}

/// Identity display with large circular avatar
pub fn id_display_large(ui: &mut Ui, id: u8) {
    let size = 64.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let center = rect.center();

        // Glow effect
        painter.circle_filled(center, size / 2.0 + 4.0, Colors::PRIMARY_GLOW);

        // Primary circle
        painter.circle_filled(center, size / 2.0, Colors::PRIMARY);

        // ID text
        painter.text(
            center,
            egui::Align2::CENTER_CENTER,
            id.to_string(),
            egui::FontId::proportional(28.0),
            Colors::TEXT_ON_PRIMARY,
        );
    }
}

/// Language selector dropdown
pub fn language_selector(ui: &mut Ui, current: &mut Language) {
    egui::ComboBox::from_id_salt("lang_sel")
        .selected_text(
            RichText::new(format!("{} {}", current.flag(), current.native_name())).size(13.0),
        )
        .width(130.0)
        .show_ui(ui, |ui| {
            for lang in Language::all() {
                ui.selectable_value(
                    current,
                    *lang,
                    format!("{} {}", lang.flag(), lang.native_name()),
                );
            }
        });
}

/// File item in the transfer list
pub fn file_list_item(ui: &mut Ui, icon: &str, name: &str, size: &str, show_remove: bool) -> bool {
    let mut removed = false;
    ui.horizontal(|ui| {
        ui.label(RichText::new(icon).size(16.0).color(Colors::ACCENT));
        ui.add_space(4.0);
        ui.label(RichText::new(name).size(13.0).color(Colors::TEXT_BODY));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if show_remove {
                if ui.button("✕").clicked() {
                    removed = true;
                }
            }
            ui.label(RichText::new(size).size(12.0).color(Colors::TEXT_MUTED));
        });
    });
    removed
}

/// Statistics display (e.g., speed, time)
pub fn stat_display(ui: &mut Ui, label: &str, value: &str, icon: Option<&str>) {
    ui.vertical(|ui| {
        ui.label(RichText::new(label).size(11.0).color(Colors::TEXT_MUTED));
        ui.horizontal(|ui| {
            if let Some(i) = icon {
                ui.label(RichText::new(i).size(14.0).color(Colors::ACCENT));
            }
            ui.label(
                RichText::new(value)
                    .size(14.0)
                    .strong()
                    .color(Colors::TEXT_HEADING),
            );
        });
    });
}

/// Zone for file dragging
pub fn drop_zone(ui: &mut Ui, text: &str, hint: &str, is_hovering: bool) -> egui::Response {
    let desired_size = Vec2::new(ui.available_width(), 100.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let border_color = if is_hovering {
            Colors::ACCENT
        } else {
            Colors::BORDER
        };
        let fill_color = if is_hovering {
            Colors::ACCENT.linear_multiply(0.05)
        } else {
            Colors::SURFACE_SUBTLE
        };

        painter.rect_filled(rect, Rounding::same(Sizes::RADIUS_MD), fill_color);
        painter.rect_stroke(
            rect,
            Rounding::same(Sizes::RADIUS_MD),
            Stroke::new(2.0, border_color),
        );

        painter.text(
            rect.center() - Vec2::new(0.0, 10.0),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(15.0),
            Colors::TEXT_HEADING,
        );
        painter.text(
            rect.center() + Vec2::new(0.0, 15.0),
            egui::Align2::CENTER_CENTER,
            hint,
            egui::FontId::proportional(12.0),
            Colors::TEXT_MUTED,
        );
    }
    response
}

/// Modern tab-style mode selector - Enhanced for speed
pub fn mode_tab(ui: &mut Ui, icon: &str, text: &str, selected: bool) -> egui::Response {
    let desired_size = Vec2::new(ui.available_width() / 2.0 - 4.0, 40.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let bg = if selected {
            Colors::PRIMARY
        } else {
            Colors::SURFACE_SUBTLE
        };
        let fg = if selected {
            Colors::TEXT_ON_PRIMARY
        } else {
            Colors::TEXT_BODY
        };

        // Add subtle animation effect when selected for speed feel
        if selected {
            painter.rect_filled(
                rect.translate(Vec2::new(0.0, 1.0)),
                Rounding::same(Sizes::RADIUS_SM),
                Color32::from_rgba_premultiplied(59, 130, 246, 30), // Subtle glow
            );
        }

        painter.rect_filled(rect, Rounding::same(Sizes::RADIUS_SM), bg);
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{} {}", icon, text),
            egui::FontId::proportional(14.0),
            fg,
        );
    }
    response
}

/// Full width horizontal divider
pub fn divider(ui: &mut Ui) {
    ui.add_space(4.0);
    let rect = ui.available_rect_before_wrap();
    ui.painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, Colors::BORDER));
    ui.add_space(8.0);
}

/// Alert message display
pub fn inline_alert(ui: &mut Ui, icon: &str, message: &str, color: Color32) {
    egui::Frame::none()
        .fill(color.linear_multiply(0.08))
        .rounding(Rounding::same(Sizes::RADIUS_SM))
        .inner_margin(Vec2::new(12.0, 8.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(icon).color(color).size(16.0));
                ui.add_space(8.0);
                ui.label(RichText::new(message).color(color).size(13.0));
            });
        });
}

/// Generic text input
pub fn styled_text_input(
    ui: &mut Ui,
    value: &mut String,
    hint: &str,
    width: f32,
) -> egui::Response {
    ui.add(
        egui::TextEdit::singleline(value)
            .hint_text(RichText::new(hint).color(Colors::TEXT_MUTED))
            .desired_width(width)
            .margin(Vec2::new(8.0, 8.0)),
    )
}

/// Styled checkbox
pub fn styled_checkbox(ui: &mut Ui, checked: &mut bool, text: &str) -> egui::Response {
    ui.checkbox(
        checked,
        RichText::new(text).color(Colors::TEXT_BODY).size(13.0),
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransferStatus {
    Idle,
    Connecting,
    WaitingConfirmation,
    Transferring,
    Compressing,
    Decompressing,
    Complete,
    Failed,
    Cancelled,
}

impl TransferStatus {
    pub fn color(&self) -> Color32 {
        match self {
            TransferStatus::Idle => Colors::TEXT_MUTED,
            TransferStatus::Connecting => Color32::from_rgb(79, 114, 205), // Softer electric blue for activity
            TransferStatus::WaitingConfirmation => Colors::WARNING,
            TransferStatus::Transferring => Color32::from_rgb(56, 189, 248), // Softer cyan for active transfer
            TransferStatus::Compressing => Color32::from_rgb(129, 163, 223), // Softer blue for processing
            TransferStatus::Decompressing => Color32::from_rgb(129, 163, 223), // Softer blue for processing
            TransferStatus::Complete => Color32::from_rgb(52, 178, 125), // Softer green for success
            TransferStatus::Failed => Color32::from_rgb(220, 70, 70), // Softer red for failure
            TransferStatus::Cancelled => Colors::WARNING,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            TransferStatus::Idle => "⏸",
            TransferStatus::Connecting => "🔗",
            TransferStatus::WaitingConfirmation => "⏳",
            TransferStatus::Transferring => "🚀",
            TransferStatus::Compressing => "🗜",
            TransferStatus::Decompressing => "📦",
            TransferStatus::Complete => "✅",
            TransferStatus::Failed => "❌",
            TransferStatus::Cancelled => "🚫",
        }
    }

    pub fn text(&self, lang: Language) -> &'static str {
        match self {
            TransferStatus::Idle => t(TranslationKey::WaitingForConnection, lang),
            TransferStatus::Connecting => t(TranslationKey::Connecting, lang),
            TransferStatus::WaitingConfirmation => t(TranslationKey::WaitingConfirmation, lang),
            TransferStatus::Transferring => t(TranslationKey::Transferring, lang),
            TransferStatus::Compressing => t(TranslationKey::Compressing, lang),
            TransferStatus::Decompressing => t(TranslationKey::Decompressing, lang),
            TransferStatus::Complete => t(TranslationKey::Complete, lang),
            TransferStatus::Failed => t(TranslationKey::Failed, lang),
            TransferStatus::Cancelled => t(TranslationKey::Cancelled, lang),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    Send,
    Receive,
}

#[derive(Debug)]
pub struct Particle {
    pub pos: Pos2,
    pub vel: Vec2,
    pub size: f32,
    pub alpha: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl Particle {
    pub fn new(pos: Pos2) -> Self {
        Self {
            pos,
            vel: Vec2::new((rand::random::<f32>() - 0.5) * 20.0, (rand::random::<f32>() - 0.5) * 20.0),
            size: rand::random::<f32>() * 3.0 + 1.0,
            alpha: 1.0,
            lifetime: 0.0,
            max_lifetime: rand::random::<f32>() * 2.0 + 1.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
        self.lifetime += dt;
        self.alpha = 1.0 - (self.lifetime / self.max_lifetime).min(1.0);
        self.vel *= 0.98; // friction
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime < self.max_lifetime && self.alpha > 0.0
    }
}
