shader_type canvas_item;

#include "res://resources/shaders/colorconv.gdshaderinc"
instance uniform vec4 current : source_color;
instance uniform int mode : hint_enum(
	"HSV",
	"HSL",
	"OkHSV",
	"OkHSL"
) = 0;

void fragment() {
	switch (mode) {
		case 0:
			vec3 hsv = rgb_to_hsv(current.r, current.g, current.b);
			COLOR = vec4(hsv_to_rgb(hsv.x, UV.x, UV.y), 1.0);
			break;

		case 1:
			vec3 hsl = rgb_to_hsl(current.r, current.g, current.b);
			COLOR = vec4(hsl_to_rgb(hsl.x, UV.x, UV.y), 1.0);
			break;

		case 2:
			vec3 okhsv = srgb_to_okhsv(current.r, current.g, current.b);
			COLOR = vec4(okhsv_to_srgb(okhsv.x, UV.x, UV.y), 1.0);
			break;

		case 3:
			vec3 okhsl = srgb_to_okhsl(current.r, current.g, current.b);
			COLOR = vec4(okhsl_to_srgb(okhsl.x, UV.x, UV.y), 1.0);
			break;
	}
}

//void light() {
//	// Called for every pixel for every light affecting the CanvasItem.
//	// Uncomment to replace the default light processing function with this one.
//}
