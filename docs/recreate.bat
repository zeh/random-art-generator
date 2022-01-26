@rem To recreate this file:
@rem 1. Go to regexr.com
@rem 2. Paste the contents of running.md into it
@rem 3. Use the expression:
@rem      ^.*`rag (mandrill.png.*)`.*src="([^"]*)".*$
@rem    With flags:
@rem      /gm
@rem    With List output of:
@rem      cargo run --release -- $1 --output $2\n
@rem 4. Copy the output to this file, replacing the existing content
@rem 5. Replace all "%" with "%%" in the content
@rem 6. Run recreate.bat (this file)

cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter-alpha 0.5 --output out_bg_000000.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter-alpha 0.5 --background-color ff00ff --output out_bg_ff00ff.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter-alpha 0.5 --background-color yellow --output out_bg_yellow.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --output out_blend_normal.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode multiply --output out_blend_multiply.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode screen --output out_blend_screen.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode overlay --output out_blend_overlay.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode darken --output out_blend_darken.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode lighten --output out_blend_lighten.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode color-dodge --output out_blend_dodge.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode color-burn --output out_blend_burn.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode hard-light --output out_blend_hard_light.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode soft-light --output out_blend_soft_light.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode difference --output out_blend_difference.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.7 --background-color 906050 --blending-mode exclusion --output out_blend_exclusion.png
cargo run --release -- mandrill.png --generations 200 --rng-seed 1 --painter-alpha 0.9 --background-color white --blending-mode normal multiply color-burn darken --output out_blend_mixed_dark.png
cargo run --release -- mandrill.png --generations 200 --rng-seed 1 --painter-alpha 0.9 --blending-mode normal screen@10 color-dodge@10 --output out_blend_mixed.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.5 --candidates 1 --output out_c_1.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.5 --candidates 10 --output out_c_10.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.5 --candidates 100 --output out_c_100.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-alpha 0.9 --color-seed 0 --output out_seed_00.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-alpha 0.9 --color-seed 0.5 --output out_seed_05.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-alpha 0.9 --color-seed 1 --output out_seed_10.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.8 --painter circles --color-seed 0.7 --diff 0.3 --output out_diff_30.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.8 --painter circles --color-seed 0.7 --diff 0.25 --output out_diff_25.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.8 --painter circles --color-seed 0.7 --diff 0.2 --output out_diff_20.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.8 --painter circles --color-seed 0.7 --diff 0.15 --output out_diff_15.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.8 --painter circles --color-seed 0.7 --diff 0.10 --output out_diff_10.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.9 --painter-width 5%% --background-color white --margins 10%% --generations 10 --output out_g_10.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.9 --painter-width 5%% --background-color white --margins 10%% --generations 25 --output out_g_25.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.9 --painter-width 5%% --background-color white --margins 10%% --generations 50 --output out_g_50.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.9 --painter-width 5%% --background-color white --margins 10%% --generations 100 --output out_g_100.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.9 --painter-width 5%% --background-color white --margins 10%% --generations 250 --output out_g_250.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.5 --painter circles --background-color beige --generations 100 --output out_margins_1.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.5 --painter circles --background-color beige --generations 100 --margins 10%% --output out_margins_2.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.5 --painter circles --background-color beige --generations 100 --margins -20%% --output out_margins_3.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.5 --painter circles --background-color beige --generations 100 --margins 20,25%% --output out_margins_4.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.5 --painter circles --background-color beige --generations 100 --margins 0,40,80,120 --output out_margins_5.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.2 --painter circles --background-color purple --max-tries 10 --output out_tries_10.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.2 --painter circles --background-color purple --max-tries 100 --output out_tries_100.png
cargo run --release -- mandrill.png --rng-seed 1 --painter-alpha 0.2 --painter circles --background-color purple --max-tries 1000 --output out_tries_1000.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.9 --background-color beige --painter rects --output out_painter_rects.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.9 --background-color beige --painter circles --output out_painter_circles.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter-alpha 0.9 --background-color beige --painter strokes --output out_painter_strokes.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --color-seed 0.5 --painter strokes --background-color white --margins 5%% --output out_alpha_100.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --color-seed 0.5 --painter strokes --background-color white --margins 5%% --painter-alpha 0.1 --output out_alpha_010.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --color-seed 0.5 --painter strokes --background-color white --margins 5%% --painter-alpha 0.6 --output out_alpha_060.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --color-seed 0.5 --painter strokes --background-color white --margins 5%% --painter-alpha 0-1 --output out_alpha_0_1.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --color-seed 0.5 --painter strokes --background-color white --margins 5%% --painter-alpha 0.1@5 0.5@4 1 --output out_alpha_list_1.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --color-seed 0.5 --painter strokes --background-color white --margins 5%% --painter-alpha 0-0.25 0.75-1 --output out_alpha_list_2.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter strokes --painter-width 5%% --painter-rotation 88-92 --background-color beige --margins 5%% --painter-alpha 0-1 --output out_alpha_bias_id.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter strokes --painter-width 5%% --painter-rotation 88-92 --background-color beige --margins 5%% --painter-alpha 0-1 --painter-alpha-bias 2 --output out_alpha_bias_p2.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter strokes --painter-width 5%% --painter-rotation 88-92 --background-color beige --margins 5%% --painter-alpha 0-1 --painter-alpha-bias -2 --output out_alpha_bias_m2.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter strokes --painter-width 5%% --painter-rotation 88-92 --background-color beige --margins 5%% --painter-alpha 0-1 --painter-alpha-bias -16 --output out_alpha_bias_m16.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --margins 4%% --background-color beige --painter-alpha 0.01-0.3 --painter rects --painter-rotation 355-365 --painter-height 1-10%% --output out_corner_id.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --margins 4%% --background-color beige --painter-alpha 0.01-0.3 --painter rects --painter-rotation 355-365 --painter-height 1-10%% --painter-corner-radius 4 --output out_corner_4.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --margins 4%% --background-color beige --painter-alpha 0.01-0.3 --painter rects --painter-rotation 355-365 --painter-height 1-10%% --painter-corner-radius 20 --output out_corner_20.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --margins 4%% --background-color beige --painter-alpha 0.01-0.3 --painter rects --painter-rotation 355-365 --painter-height 1-10%% --painter-corner-radius 0 2%% 20 --output out_corner_m.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --scale 0.1 --painter circles --output out_antialias_yes.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --scale 0.1 --painter circles --painter-disable-anti-alias --output out_antialias_no.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --output out_height.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-height 10%% --output out_height_10.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-height 100 --output out_height_100.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-height 5%% 10%% 15%% 20%% --output out_height_m1.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-height 5%% 100-200@3 --output out_height_m2.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-width 10 --output out_height_bias_id.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-width 10 --painter-height-bias 16 --output out_height_bias_16.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-width 10 --painter-height-bias -16 --output out_height_bias_m16.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-width 10 --painter-height 10%%-50%% --output out_height_bias_hid.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --painter-width 10 --painter-height 10%%-50%% --painter-height-bias -4 --output out_height_bias_hm4.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 6%% --output out_length_id.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 6%% --painter-length 10%% --output out_length_10.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 6%% --painter-length 100 --output out_length_100.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 6%% --painter-length 5%% 10%% 20%% --output out_length_m1.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 6%% --painter-length 5%% 100-200@3 --output out_length_m2.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 10 --output out_length_bias_id.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 10 --painter-length-bias 16 --output out_length_bias_16.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 10 --painter-length-bias -16 --output out_length_bias_m16.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 10 --painter-length 10%%-50%% --output out_length_bias_hid.png
cargo run --release -- mandrill.png --painter strokes --generations 20 --rng-seed 1 --painter-width 10 --painter-length 10%%-50%% --painter-length-bias -4 --output out_length_bias_hm4.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color pink --painter-alpha 0.5 --painter circles --output out_radius_id.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color pink --painter-alpha 0.5 --painter circles --painter-radius 10%% --output out_radius_10.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color pink --painter-alpha 0.5 --painter circles --painter-radius 16 32 --output out_radius_1632.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color pink --painter-alpha 0.5 --painter circles --painter-radius 200-50%% --output out_radius_20050.png
cargo run --release -- mandrill.png --generations 200 --rng-seed 1 --painter circles --output out_radius_bias_id.png
cargo run --release -- mandrill.png --generations 200 --rng-seed 1 --painter circles --painter-radius-bias 16 --output out_radius_bias_16.png
cargo run --release -- mandrill.png --generations 200 --rng-seed 1 --painter circles --painter-radius-bias -16 --output out_radius_bias_m16.png
cargo run --release -- mandrill.png --generations 200 --rng-seed 1 --painter circles --painter-radius 2-10%% --painter-radius-bias -2 --output out_radius_bias_m2.png
cargo run --release -- mandrill.png --generations 400 --rng-seed 1 --background-color beige --painter-alpha 0.5-0.9 --painter rects --painter-height 4%% --output out_rotation_id.png
cargo run --release -- mandrill.png --generations 400 --rng-seed 1 --background-color beige --painter-alpha 0.5-0.9 --painter rects --painter-height 4%% --painter-rotation 45 --output out_rotation_45.png
cargo run --release -- mandrill.png --generations 400 --rng-seed 1 --background-color beige --painter-alpha 0.5-0.9 --painter rects --painter-height 4%% --painter-rotation 350-370 --output out_rotation_mp10.png
cargo run --release -- mandrill.png --generations 400 --rng-seed 1 --background-color beige --painter-alpha 0.5-0.9 --painter rects --painter-height 4%% --painter-rotation 0 90 --output out_rotation_090.png
cargo run --release -- mandrill.png --generations 400 --rng-seed 1 --background-color beige --painter-alpha 0.5-0.9 --painter rects --painter-height 4%% --painter-rotation 0-360 --output out_rotation_all.png
cargo run --release -- mandrill.png --generations 2000 --rng-seed 1 --background-color beige --painter-alpha 0.9 --painter rects --painter-height 3 --painter-rotation 0-360 --output out_rotation_all_thin.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter strokes --background-color snow --painter-alpha 0.9 --painter-width 8-16%% --painter-rotation 89-91 --margins 8%% --painter-smear-scale 2 --output out_smear_id.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter strokes --background-color snow --painter-alpha 0.9 --painter-width 8-16%% --painter-rotation 89-91 --margins 8%% --painter-smear-scale 2 --painter-smear 1 --output out_smear_1.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter strokes --background-color snow --painter-alpha 0.9 --painter-width 8-16%% --painter-rotation 89-91 --margins 8%% --painter-smear-scale 2 --painter-smear 0 0.5 1 --output out_smear_05_1.png
cargo run --release -- mandrill.png --generations 10 --rng-seed 1 --painter strokes --background-color snow --painter-alpha 0.9 --painter-width 8-16%% --painter-rotation 89-91 --margins 8%% --painter-smear-scale 2 --painter-smear 0@3 0.5-0.6 --output out_smear_m1.png
cargo run --release -- mandrill.png --generations 6 --rng-seed 1 --painter strokes --background-color gainsboro --painter-width 8-16%% --painter-rotation 42-48 312-318 --margins 8%% --output out_smear_scale_id.png
cargo run --release -- mandrill.png --generations 6 --rng-seed 1 --painter strokes --background-color gainsboro --painter-width 8-16%% --painter-rotation 42-48 312-318 --margins 8%% --painter-smear-scale 0.5 --output out_smear_scale_05.png
cargo run --release -- mandrill.png --generations 6 --rng-seed 1 --painter strokes --background-color gainsboro --painter-width 8-16%% --painter-rotation 42-48 312-318 --margins 8%% --painter-smear-scale 2 --output out_smear_scale_2.png
cargo run --release -- mandrill.png --generations 6 --rng-seed 1 --painter strokes --background-color gainsboro --painter-width 8-16%% --painter-rotation 42-48 312-318 --margins 8%% --painter-smear-scale 8 --output out_smear_scale_8.png
cargo run --release -- mandrill.png --generations 6 --rng-seed 1 --painter strokes --background-color gainsboro --painter-width 8-16%% --painter-rotation 42-48 312-318 --margins 8%% --painter-smear-scale 0.5 2 --output out_smear_scale_m1.png
cargo run --release -- mandrill.png --generations 6 --rng-seed 1 --painter strokes --background-color gainsboro --painter-width 8-16%% --painter-rotation 42-48 312-318 --margins 8%% --painter-smear-scale 1-4 --output out_smear_scale_m2.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color darkgreen --painter strokes --output out_wave_height_id.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color darkgreen --painter strokes --painter-wave-height 2%% --output out_wave_height_2.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color darkgreen --painter strokes --painter-wave-height 10-20 --output out_wave_height_10.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color darkgreen --painter strokes --painter-wave-height 50 90 1-4%% --output out_wave_height_m.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color darkred --painter strokes --painter-wave-height 0%%-10%% --output out_wave_height_bias_id.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color darkred --painter strokes --painter-wave-height 0%%-10%% --painter-wave-height-bias -16 --output out_wave_height_bias_m16.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color darkred --painter strokes --painter-wave-height 0%%-10%% --painter-wave-height-bias 16 --output out_wave_height_bias_16.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color wheat --painter strokes --output out_wave_length_id.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color wheat --painter strokes --painter-wave-length 200%% --output out_wave_length_200.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color wheat --painter strokes --painter-wave-length 4000%% --output out_wave_length_4000.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color wheat --painter strokes --painter-wave-length 100 --output out_wave_length_100.png
cargo run --release -- mandrill.png --generations 20 --rng-seed 1 --background-color wheat --painter strokes --painter-wave-height 2%% --painter-wave-length 10%% 400%% 8000%%@2 --output out_wave_length_m.png
cargo run --release -- mandrill.png --generations 40 --rng-seed 1 --background-color snow --painter strokes --painter-width 4%% --painter-length 90%%-100%% --painter-alpha 0.5-1 --painter-wave-length 1%%-4000%% --output out_wave_length_bias_id.png
cargo run --release -- mandrill.png --generations 40 --rng-seed 1 --background-color snow --painter strokes --painter-width 4%% --painter-length 90%%-100%% --painter-alpha 0.5-1 --painter-wave-length 1%%-4000%% --painter-wave-length-bias -16 --output out_wave_length_bias_m16.png
cargo run --release -- mandrill.png --generations 40 --rng-seed 1 --background-color snow --painter strokes --painter-width 4%% --painter-length 90%%-100%% --painter-alpha 0.5-1 --painter-wave-length 1%%-4000%% --painter-wave-length-bias 16 --output out_wave_length_bias_16.png
cargo run --release -- mandrill.png --generations 20 --background-color whitesmoke --rng-seed 1 --painter-height 5%% --margins 4%% --output out_width_id.png
cargo run --release -- mandrill.png --generations 20 --background-color whitesmoke --rng-seed 1 --painter-height 5%% --margins 4%% --painter-width 50%% --output out_width_50.png
cargo run --release -- mandrill.png --generations 20 --background-color whitesmoke --rng-seed 1 --painter-height 5%% --margins 4%% --painter-width 80 --output out_width_80.png
cargo run --release -- mandrill.png --generations 20 --background-color whitesmoke --rng-seed 1 --painter-height 5%% --margins 4%% --painter-width 25%% 50%% 75%% 100%% --output out_width_m.png
cargo run --release -- mandrill.png --generations 30 --background-color gainsboro --rng-seed 1 --painter-height 4%% --margins 4%% --output out_width_bias_id.png
cargo run --release -- mandrill.png --generations 30 --background-color gainsboro --rng-seed 1 --painter-height 4%% --margins 4%% --painter-width-bias 16 --output out_width_bias_16.png
cargo run --release -- mandrill.png --generations 30  --background-color gainsboro --rng-seed 1 --painter-height 4%% --margins 4%% --painter-width-bias -16 --output out_width_bias_m16.png
cargo run --release -- mandrill.png --generations 30 --background-color gainsboro --rng-seed 1 --painter-height 4%% --margins 4%% --painter-width 10-40%% --painter-width-bias 4 --output out_width_bias_m.png
cargo run --release -- mandrill.png --generations 100 --painter-alpha 0.7 --margins -10%% --output out_rng_id.png
cargo run --release -- mandrill.png --generations 100 --painter-alpha 0.7 --margins -10%% --rng-seed 1 --output out_rng_1.png
cargo run --release -- mandrill.png --generations 100 --painter-alpha 0.7 --margins -10%% --rng-seed 2 --output out_rng_2.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter strokes --painter-alpha 0.7 --painter-width 5%% --painter-rotation 89-91 --margins 8%% --output out_scale_id.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter strokes --painter-alpha 0.7 --painter-width 5%% --painter-rotation 89-91 --margins 8%% --scale 4 --output out_scale_4.png
cargo run --release -- mandrill.png --generations 100 --rng-seed 1 --painter strokes --painter-alpha 0.7 --painter-width 5%% --painter-rotation 89-91 --margins 8%% --scale 0.1 --output out_scale_01.png
cargo run --release -- mandrill.png --generations 300 --rng-seed 1 --color-seed 1 --painter strokes --painter-alpha 0.1-0.7 --painter-width 2%%-4%% --margins 8%% --background-color beige --output out_matrix_id.png
cargo run --release -- mandrill.png --generations 300 --rng-seed 1 --color-seed 1 --painter strokes --painter-alpha 0.1-0.7 --painter-width 2%%-4%% --margins 8%% --background-color beige --target-color-matrix 0.33,0.59,0.11,0,0.33,0.59,0.11,0,0.33,0.59,0.11,0 --output out_matrix_gray.png
cargo run --release -- mandrill.png --generations 300 --rng-seed 1 --color-seed 1 --painter strokes --painter-alpha 0.1-0.7 --painter-width 2%%-4%% --margins 8%% --background-color beige --target-color-matrix 0.393,0.769,0.686,0,0.349,0.686,0.168,0,0.272,0.534,0.131,0 --output out_matrix_sepia.png
cargo run --release -- mandrill.png --generations 300 --rng-seed 1 --color-seed 1 --painter strokes --painter-alpha 0.1-0.7 --painter-width 2%%-4%% --margins 8%% --background-color beige --target-color-matrix 1.438,0.122,-0.016,-8,-0.062,1.378,-0.016,-13,-0.062,-0.122,1.483,-5 --output out_matrix_polaroid.png

for %%i in (*.png) do (optipng -o7 %%i)

