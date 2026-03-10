shader_type canvas_item;

#include "res://resources/shaders/colorconv.gdshaderinc"

instance uniform vec4 current : source_color;
instance uniform int type : hint_enum(
	"R",
	"G",
	"B",
	"HSV H",
	"HSV S",
	"HSV V",
	"HSL H",
	"HSL S",
	"HSL L",
	"OkHSV H",
	"OkHSV S",
	"OkHSV V",
	"OkHSL H",
	"OkHSL S",
	"OkHSL L",
	"A"
) = 0;

void fragment() {
	float value = UV.x;
	
	switch (type) {
		case 0:
			COLOR = vec4(value, current.g, current.b, 1.0);
			break;
		case 1:
			COLOR = vec4(current.r, value, current.b, 1.0);
			break;
		case 2:
			COLOR = vec4(current.r, current.g, value, 1.0);
			break;
			
		case 3:
			vec3 hsv = rgb_to_hsv(current.r, current.g, current.b);
			COLOR = vec4(hsv_to_rgb(value, hsv.y, hsv.z), 1.0);
			break;
		case 4:
			vec3 hsv = rgb_to_hsv(current.r, current.g, current.b);
			COLOR = vec4(hsv_to_rgb(hsv.x, value, hsv.z), 1.0);
			break;
		case 5:
			vec3 hsv = rgb_to_hsv(current.r, current.g, current.b);
			COLOR = vec4(hsv_to_rgb(hsv.x, hsv.y, value), 1.0);
			break;
			
		case 6:
			vec3 hsl = rgb_to_hsl(current.r, current.g, current.b);
			COLOR = vec4(hsl_to_rgb(value, hsl.y, hsl.z), 1.0);
			break;
		case 7:
			vec3 hsl = rgb_to_hsl(current.r, current.g, current.b);
			COLOR = vec4(hsl_to_rgb(hsl.x, value, hsl.z), 1.0);
			break;
		case 8:
			vec3 hsl = rgb_to_hsl(current.r, current.g, current.b);
			COLOR = vec4(hsl_to_rgb(hsl.x, hsl.y, value), 1.0);
			break;
			
		case 9:
			vec3 okhsv = srgb_to_okhsv(current.r, current.g, current.b);
			COLOR = vec4(okhsv_to_srgb(value, okhsv.y, okhsv.z), 1.0);
			break;
		case 10:
			vec3 okhsv = srgb_to_okhsv(current.r, current.g, current.b);
			COLOR = vec4(okhsv_to_srgb(okhsv.x, value, okhsv.z), 1.0);
			break;
		case 11:
			vec3 okhsv = srgb_to_okhsv(current.r, current.g, current.b);
			COLOR = vec4(okhsv_to_srgb(okhsv.x, okhsv.y, value), 1.0);
			break;
			
		case 12:
			vec3 okhsl = srgb_to_okhsl(current.r, current.g, current.b);
			COLOR = vec4(okhsl_to_srgb(value, okhsl.y, okhsl.z), 1.0);
			break;
		case 13:
			vec3 okhsl = srgb_to_okhsl(current.r, current.g, current.b);
			COLOR = vec4(okhsl_to_srgb(okhsl.x, value, okhsl.z), 1.0);
			break;
		case 14:
			vec3 okhsl = srgb_to_okhsl(current.r, current.g, current.b);
			COLOR = vec4(okhsl_to_srgb(okhsl.x, okhsl.y, value), 1.0);
			break;
			
		case 15:
			COLOR = vec4(current.rgb, value);
			break;
	}

}
