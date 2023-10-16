use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    
    
    FadeView = <CachedView> {
        dpi_factor: 2.0,
        
        draw_bg: {
            instance opacity: 1.0
            
            fn pixel(self) -> vec4 {
                let color = sample2d_rt(self.image, self.pos);
                return Pal::premul(vec4(color.xyz, color.w * self.opacity))
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
}