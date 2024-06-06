use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    FadeView = <CachedView> {
        draw_bg: {
            instance opacity: 1.0
            
            fn pixel(self) -> vec4 {
                let color = sample2d_rt(self.image, self.pos * self.scale + self.shift) + vec4(self.marked, 0.0, 0.0, 0.0);
                return Pal::premul(vec4(color.xyz, color.w * self.opacity))
            }
        }
    }

    CenteredScaledImage = <Image> {
        draw_bg: {
            texture image: texture2d
            instance opacity: 1.0
            instance image_scale: vec2(1.0, 1.0)

            fn get_color_scale(self, scale: vec2) -> vec4 {
                let traslation = vec2(0.0, 0.0);
                if scale.x > 1.0 {
                    traslation.x = 0.5 - 1.0 / (scale.x * 2.0);
                }
                if scale.y > 1.0 {
                    traslation.y = 0.5 - 1.0 / (scale.y * 2.0);
                }
                return sample2d(self.image, (self.pos - traslation) * scale).xyzw;
            }

            fn get_color(self) -> vec4 {
                return self.get_color_scale(self.image_scale)
            }

            fn pixel(self) -> vec4 {
                let color = self.get_color();
                return Pal::premul(vec4(color.xyz, color.w * self.opacity))
            }
        }
    }

    CenteredOnTop = <Image> {
        draw_bg: {
            instance radius: 90.
            instance opacity: 1.0
            instance image_scale: vec2(1.0, 1.0)

            fn get_opacity(self) -> float {
                return self.opacity;
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1,
                    1,
                    self.rect_size.x - 2.0,
                    // This calculation is to make sure the bottom part is not rounded
                    self.rect_size.y + self.radius * 2.0,
                    max(1.0, self.radius)
                );

                let color = self.get_color();
                sdf.fill_keep(Pal::premul(vec4(color.xyz, color.w * self.get_opacity())));
                return sdf.result
            }
        }
    }

    CenteredOnBottom = <Image> {
        draw_bg: {
            instance radius: 60.
            instance opacity: 1.0
            instance image_scale: vec2(1.0, 1.0)
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box_y(
                    1,
                    1,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    1.0,
                    max(1.0, self.radius)
                );

                let color = self.get_color();
                sdf.fill_keep(Pal::premul(vec4(color.xyz, color.w * self.opacity)));
                return sdf.result
            }
        }
    }

    Line = <View> {
        width: Fill,
        height: 1,
        show_bg: true,
        draw_bg: {
            color: #8b9e77
        }
    }

    VerticalLine = <View> {
        width: 1,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: #8b9e77
        }
    }

    IconButton = <Button> {
        draw_icon: {
            fn get_color(self) -> vec4 {
                return #fff
            }
        }
        icon_walk: {width: 7.5, height: Fit}
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        padding: 9.0
        text: ""
    }

    VerticalFadingBar = <RoundedYView> {
        width: 10.0
        height: 50.0

        show_bg: true
        draw_bg: {
            color: #fff
            instance opacity: 0.3

            fn get_color(self) -> vec4 {
                return mix(
                    vec4(self.color.rgb, 0.0),
                    vec4(self.color.rgb, self.opacity),
                    self.pos.y);
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box_y(
                    self.inset.x + self.border_width,
                    self.inset.y + self.border_width,
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                    self.radius.x,
                    self.radius.y
                )
                sdf.fill_keep(self.get_color())
                if self.border_width > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_width)
                }
                return sdf.result;
            }
        }
    }

    BlurStage = <ViewBase> {
        optimize: Texture,
        draw_bg: {
            texture image: texture2d

            uniform blursize: 0.0,
            uniform blurstd: 1.5,
            uniform blurx: 1.0,
            uniform blury: 0.0,
            varying g1: float,
            varying g2: float,
            varying g3: float,
            varying g4: float,
            varying g5: float,

            varying gaussscale: float,

            varying o0: vec2,

            varying o1a: vec2,
            varying o2a: vec2,
            varying o3a: vec2,
            varying o4a: vec2,
            varying o5a: vec2,

            varying o1b: vec2,
            varying o2b: vec2,
            varying o3b: vec2,
            varying o4b: vec2,
            varying o5b: vec2,

            fn vertex(self) -> vec4
            {
                let x = self.blurx;
                let y = self.blury;
                let dpi = self.dpi_factor;
                let ceil_size = ceil(self.rect_size * dpi) / dpi
                let floor_pos = floor(self.rect_pos * dpi) / dpi

                let offset = 0.003 * self.blursize / max(x,y);
                let standard_deviation = 0.0001 + self.blurstd *0.003;
                let st_dev_sqr = standard_deviation * standard_deviation;

                let off1 = offset;
                let off2 = 2.0*offset;
                let off3 = 3.0*offset;
                let off4 = 4.0*offset;
                let off5 = 5.0*offset;

                let mainscale = (1.0 / sqrt(2*PI*st_dev_sqr));
                let stddevscale = 1.0/ (2*st_dev_sqr);

                self.g1 =  pow(E, -((off1*off1)* stddevscale));
                self.g2 =  pow(E, -((off2*off2)* stddevscale));
                self.g3 =  pow(E, -((off3*off3)* stddevscale));
                self.g4 =  pow(E, -((off4*off4)* stddevscale));
                self.g5 =  pow(E, -((off5*off5)* stddevscale));

                self.gaussscale = 1.0/(1.0 +  (self.g1 + self.g2 + self.g3 + self.g4 + self.g5 )*2.0);

                let pos = self.clip_and_transform_vertex(self.rect_pos, self.rect_size);
                self.o0 = self.pos*0.5;

                self.o1a = self.o0 + vec2(off1*x,off1*y);
                self.o2a = self.o0 + vec2(off2*x,off2*y);
                self.o3a = self.o0 + vec2(off3*x,off3*y);
                self.o4a = self.o0 + vec2(off4*x,off4*y);
                self.o5a = self.o0 + vec2(off5*x,off5*y);

                self.o1b = self.o0 - vec2(off1*x,off1*y);
                self.o2b = self.o0 - vec2(off2*x,off2*y);
                self.o3b = self.o0 - vec2(off3*x,off3*y);
                self.o4b = self.o0 - vec2(off4*x,off4*y);
                self.o5b = self.o0 - vec2(off5*x,off5*y);

                return pos;
            }

            fn pixel(self) -> vec4
            {

                let col = sample2d_rt(self.image, self.o0) ;
                col +=  (sample2d_rt(self.image, self.o1a) + sample2d_rt(self.image, self.o1b)) * self.g1;
                col +=  (sample2d_rt(self.image, self.o2a) + sample2d_rt(self.image, self.o2b)) * self.g2 ;
                col +=  (sample2d_rt(self.image, self.o3a) + sample2d_rt(self.image, self.o3b)) * self.g3 ;
                col +=  (sample2d_rt(self.image, self.o4a) + sample2d_rt(self.image, self.o4b)) * self.g4 ;
                col +=  (sample2d_rt(self.image, self.o5a) + sample2d_rt(self.image, self.o5b)) * self.g5 ;
                col = col * self.gaussscale;

                return col ;
            }
        }
    }
}