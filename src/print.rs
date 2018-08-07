use super::get_state;
use gcn_fonts::get_coords;
use gcn::gx;

pub unsafe fn setup_draw() {
    let state = get_state();
    gx::set_projection(&mut state.perspective, gx::ORTHOGRAPHIC);

    gx::set_cull_mode(gx::CULL_BACK);

    gx::set_blend_mode(
        gx::BM_BLEND,
        gx::BL_SRCALPHA,
        gx::BL_INVSRCALPHA,
        gx::LO_SET,
    );
    gx::clear_vtx_desc();
    gx::set_vtx_desc(gx::VA_POS as u8, gx::DIRECT);
    gx::set_vtx_desc(gx::VA_CLR0 as u8, gx::DIRECT);
    gx::set_vtx_desc(gx::VA_TEX0 as u8, gx::DIRECT);

    gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_POS, gx::POS_XY, gx::F32, 0);
    gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_CLR0, gx::CLR_RGBA, gx::RGBA8, 0);
    gx::set_vtx_attr_fmt(gx::VTXFMT0, gx::VA_TEX0, gx::TEX_ST, gx::F32, 0);

    gx::set_num_tex_gens(1);
    gx::set_tex_coord_gen(
        gx::TEXCOORD0 as u16,
        gx::TG_MTX2X4,
        gx::TG_TEX0,
        gx::IDENTITY,
    );

    // gx::set_tev_op(gx::TEVSTAGE0, gx::REPLACE);
    gx::set_tev_color_in(
        gx::TEVSTAGE0,
        gx::CC_ZERO,
        gx::CC_ZERO,
        gx::CC_ZERO,
        gx::CC_RASC,
    );
    gx::set_tev_alpha_in(
        gx::TEVSTAGE0,
        gx::CA_ZERO,
        gx::CA_RASA,
        gx::CA_TEXA,
        gx::CA_ZERO,
    );
    gx::set_tev_color_op(
        gx::TEVSTAGE0,
        gx::TEV_ADD,
        gx::TB_ZERO,
        gx::CS_SCALE_1,
        gx::TRUE,
        gx::TEVPREV,
    );
    gx::set_tev_alpha_op(
        gx::TEVSTAGE0,
        gx::TEV_ADD,
        gx::TB_ZERO,
        gx::CS_SCALE_1,
        gx::TRUE,
        gx::TEVPREV,
    );
    gx::set_tev_order(gx::TEVSTAGE0, gx::TEXCOORD0, gx::TEXMAP0, gx::COLOR0A0);
    gx::load_tex_obj(state.font.texture.get(), gx::TEXMAP0 as u8);
}

pub unsafe fn printf(s: &str, mut x: f32, y: f32, top_color: u32, bottom_color: u32) {
    s.chars().for_each(|c| {
        let adv_x = print_char(c, x, y, top_color, bottom_color);
        x += adv_x + 0.5;
    });
}

pub unsafe fn print_char(c: char, x: f32, y: f32, top_color: u32, bottom_color: u32) -> f32 {
    let state = get_state();
    if let Some((coords, width, height, descender)) = get_coords(&state.font.font, c) {
        let y = y + state.font.font.size + descender;
        let shift = 1.5;
            gx::begin(gx::QUADS, gx::VTXFMT0, 4);
            {
                let x = x + shift;
                let y = y + shift;
                gx::submit_f32s(&[x, y - height]);
                gx::submit_u32(0x00_00_00_D0);
                gx::submit_f32s(&coords[0]);

                gx::submit_f32s(&[x + width, y - height]);
                gx::submit_u32(0x00_00_00_D0);
                gx::submit_f32s(&coords[1]);

                gx::submit_f32s(&[x + width, y]);
                gx::submit_u32(0x00_00_00_D0);
                gx::submit_f32s(&coords[3]);

                gx::submit_f32s(&[x, y]);
                gx::submit_u32(0x00_00_00_D0);
                gx::submit_f32s(&coords[2]);
            }
            gx::end();

        gx::begin(gx::QUADS, gx::VTXFMT0, 4);
        {
            gx::submit_f32s(&[x, y - height]);
            gx::submit_u32(top_color);
            gx::submit_f32s(&coords[0]);

            gx::submit_f32s(&[x + width, y - height]);
            gx::submit_u32(top_color);
            gx::submit_f32s(&coords[1]);

            gx::submit_f32s(&[x + width, y]);
            gx::submit_u32(bottom_color);
            gx::submit_f32s(&coords[3]);

            gx::submit_f32s(&[x, y]);
            gx::submit_u32(bottom_color);
            gx::submit_f32s(&coords[2]);
        }
        gx::end();
        width
    } else {
        state.font.font.space_advance
    }
}
