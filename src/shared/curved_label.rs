use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    DrawRotatedText = {{DrawRotatedText}} {
        color: #fff
        
        uniform brightness: float
        uniform curve: float
        uniform rotation: float
        uniform rotation_origin_inside_char: vec2
        
        texture tex: texture2d
        
        varying tex_coord1: vec2
        varying tex_coord2: vec2
        varying tex_coord3: vec2
        varying pos: vec2
        
        fn vertex(self) -> vec4 {
            let min_pos = vec2(self.rect_pos.x, self.rect_pos.y)
            let max_pos = vec2(self.rect_pos.x + self.rect_size.x, self.rect_pos.y - self.rect_size.y)
            
            self.clipped = clamp(
                mix(min_pos, max_pos, self.geom_pos),
                self.draw_clip.xy,
                self.draw_clip.zw
            )
            
            let normalized: vec2 = (self.clipped - min_pos) / vec2(self.rect_size.x, -self.rect_size.y)
            
            self.tex_coord1 = mix(
                self.font_t1.xy,
                self.font_t2.xy,
                normalized.xy
            )
            self.pos = normalized;

            let rot_translation = self.rect_pos + self.rotation_origin_inside_char;
            
            let rot_transform = mat4(
                vec4(1., 0., 0., 0.),
                vec4(0., 1., 0., 0.),
                vec4(0., 0., 1., 0.),
                vec4(rot_translation.x, rot_translation.y, 0., 1.)
            ) * mat4(
                vec4(cos(self.rotation), -sin(self.rotation), 0., 0.),
                vec4(sin(self.rotation), cos(self.rotation), 0., 0.),
                vec4(0., 0., 1., 0.),
                vec4(0., 0., 0., 1.)
            ) * mat4(
                vec4(1., 0., 0., 0.),
                vec4(0., 1., 0., 0.),
                vec4(0., 0., 1., 0.),
                vec4(-rot_translation.x, -rot_translation.y, 0., 1.)
            );

            return self.camera_projection * (self.camera_view * (self.view_transform  * rot_transform  * vec4(
                self.clipped.x,
                self.clipped.y,
                self.char_depth + self.draw_zbias,
                1.
            )));
        }
        
        fn get_color(self) -> vec4 {
            return self.color;
        }
        fn blend_color(self, incol:vec4)->vec4{
            return incol
        }
        fn pixel(self) -> vec4 {
            let s = sample2d_rt(self.tex, self.tex_coord1.xy).x;
            s = pow(s, self.curve);
            let col = self.get_color();
            return self.blend_color(vec4(s * col.rgb * self.brightness * col.a, s * col.a));
        }
    }

    CurvedLabel = {{CurvedLabel}} {
        text: "HELLO WORLD",

        draw_bg: {
            color: #fff
        }

        draw_text: {
            color: #000,
            text_style: <MONO_TEXT> {},
        }
    }
}

#[derive(Live, LiveHook)]#[repr(C)]
struct DrawRotatedText {
    #[deref] draw_super: DrawText,
}

#[derive(Live)]
pub struct CurvedLabel {
    #[live]
    text: String,

    #[walk]
    walk: Walk,

    #[layout]
    layout: Layout,

    #[live(0.0)]
    rotation: f64,

    #[live(100.0)]
    radius: f64,

    #[live(std::f64::consts::PI * 0.5)]
    total_angle: f64,

    #[rust(20.0)]
    extra_horizontal_padding: f64,

    #[live] draw_bg: DrawColor,
    #[live] draw_text: DrawRotatedText,
}

impl LiveHook for CurvedLabel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, CurvedLabel);
    }
}

impl Widget for CurvedLabel {
    fn redraw(&mut self, cx: &mut Cx) {
        self.draw_bg.redraw(cx);
        self.draw_text.redraw(cx);
    }

    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl CurvedLabel {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        self.draw_bg.begin(cx, walk, self.layout);
        let abs_pos = cx.turtle().pos();

        let width = cx.turtle().size().x;
        let radius = if walk.width.is_fit() || width.is_nan() {
            self.radius
        } else {
            ((width - self.layout.padding.left - self.layout.padding.right) / 2.0) / sin(self.total_angle / 2.0)
        } - self.extra_horizontal_padding;

        let len = self.text.len();
        for (index, char) in self.text.chars().enumerate() {
            let slice_angle = self.total_angle / (len as f64);
            let angle = -(index as f64 - (len / 2) as f64) * slice_angle + self.rotation;

            let width_reduction = walk.margin.left + walk.margin.right + self.layout.padding.left + self.layout.padding.right;
            let offset_pos = if width.is_nan() {
                dvec2(
                    -radius * angle.sin() + radius - width_reduction,
                    -radius * angle.cos() + radius
                )
            } else {
                dvec2(
                    -radius * angle.sin() + (width - width_reduction) / 2.0,
                    -radius * angle.cos() + radius
                )
            };

            self.draw_text.draw_vars.set_uniform(cx, id!(rotation), &[angle as f32]);

            // This is an approximate calculation of the rotation origin,
            // that has to be in the center of the character.
            let font_width = self.draw_text.text_style.font_size;
            self.draw_text.draw_vars.set_uniform(
                cx,
                id!(rotation_origin_inside_char),
                &[font_width as f32 * 0.6666 / 2.0 , font_width as f32 / 2.0]
            );

            let new_pos = abs_pos + offset_pos;
            self.draw_text.draw_walk(cx, Walk{abs_pos: Some(new_pos), ..walk}, Align::default(), &char.to_string());
        }

        self.draw_bg.end(cx);
    }
}