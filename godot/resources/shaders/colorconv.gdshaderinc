



vec3 rgb_to_hsl(float r, float g, float b) {
    float max = max(r, max(g, b));
    float min = min(r, min(g, b));
    float h, s;
    float l = (max + min) / 2.0;

    if (max == min) {
        h = s = 0.0;
    } else {
        float d = max - min;
        s = l > 0.5 ? d / (2.0 - max - min) : d / (max + min);
        if (max == r) {
            h = (g - b) / d + (g < b ? 6.0 : 0.0);
        } else if (max == g) {
            h = (b - r) / d + 2.0;
        } else if (max == b) {
            h = (r - g) / d + 4.0;
        }

        h /= 6.0;
    }

    return vec3(h, s, l);
}

float hue_to_rgb(float p, float q, float t) {
    if (t < 0.0)
        t += 1.0;
    if (t > 1.0)
        t -= 1.0;
    if (t < 1.0 / 6.0)
        return p + (q - p) * 6.0 * t;
    if (t < 1.0 / 2.0)
        return q;
    if (t < 2.0 / 3.0)
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    return p;
}

vec3 hsl_to_rgb(float h, float s, float l) {
    float r, g, b;

    if (s == 0.0) {
        r = g = b = l;
    } else {
        float q = l < 0.5 ? l * (1.0 + s) : l + s - l * s;
        float p = 2.0 * l - q;
        r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1.0 / 3.0);
    }

    return vec3(r, g, b);
}

vec3 rgb_to_hsv(float r, float g, float b) {
    float max = max(r, max(g, b));
    float min = min(r, min(g, b));
    float h, s;
    float v = max;

    float d = max - min;
    s = max == 0.0 ? 0.0 : d / max;

    if (max == min) {
        h = 0.0; // achromatic
    } else {
        if (max == r) {
            h = (g - b) / d + (g < b ? 6.0 : 0.0);

        } else if (max == g) {
            h = (b - r) / d + 2.0;

        } else if (max == b) {
            h = (r - g) / d + 4.0;
        }
        h /= 6.0;
    }

    return vec3(h, s, v);
}

vec3 hsv_to_rgb(float h, float s, float v) {
    float r, g, b;

    float i = floor(h * 6.0);
    float f = h * 6.0 - i;
    float p = v * (1.0 - s);
    float q = v * (1.0 - f * s);
    float t = v * (1.0 - (1.0 - f) * s);

    switch (int(i) % 6) {
        case 0:
            r = v;
            g = t;
            b = p;
            break;
        case 1:
            r = q;
            g = v;
            b = p;
            break;
        case 2:
            r = p;
            g = v;
            b = t;
            break;
        case 3:
            r = p;
            g = q;
            b = v;
            break;
        case 4:
            r = t;
            g = p;
            b = v;
            break;
        case 5:
            r = v;
            g = p;
            b = q;
            break;
    }

    return vec3(r, g, b);
}

float srgb_transfer_function(float a) {
    return .0031308 >= a ? 12.92 * a : 1.055 * pow(a, .4166666666666667) - .055;
}

float srgb_transfer_function_inv(float a) {
    return .04045 < a ? pow((a + .055) / 1.055, 2.4) : a / 12.92;
}

vec3 linear_srgb_to_oklab(float r, float g, float b) {
    float l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    float m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    float s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

    float l_ = pow(l, 1.0 / 3.0);
    float m_ = pow(m, 1.0 / 3.0);
    float s_ = pow(s, 1.0 / 3.0);

    return vec3(
        0.2104542553f * l_ + 0.7936177850f * m_ - 0.0040720468f * s_,
        1.9779984951f * l_ - 2.4285922050f * m_ + 0.4505937099f * s_,
        0.0259040371f * l_ + 0.7827717662f * m_ - 0.8086757660f * s_
    );
}

vec3 oklab_to_linear_srgb(float L, float a, float b) {
    float l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    float m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    float s_ = L - 0.0894841775 * a - 1.2914855480 * b;

    float l = l_ * l_ * l_;
    float m = m_ * m_ * m_;
    float s = s_ * s_ * s_;

    return vec3(
        (+4.0767416621f * l - 3.3077115913f * m + 0.2309699292f * s),
        (-1.2684380046f * l + 2.6097574011f * m - 0.3413193965f * s),
        (-0.0041960863f * l - 0.7034186147f * m + 1.7076147010f * s)
    );
}

float toe(float x) {
    float k_1 = 0.206;
    float k_2 = 0.03;
    float k_3 = (1.0 + k_1) / (1.0 + k_2);

    return 0.5 * (k_3 * x - k_1 + sqrt((k_3 * x - k_1) * (k_3 * x - k_1) + 4.0 * k_2 * k_3 * x));
}

float toe_inv(float x) {
    float k_1 = 0.206;
    float k_2 = 0.03;
    float k_3 = (1.0 + k_1) / (1.0 + k_2);
    return (x * x + k_1 * x) / (k_3 * (x + k_2));
}

float compute_max_saturation(float a, float b) {
    // Max saturation will be when one of r, g or b goes below zero.

    // Select different coefficients depending on which component goes below zero first
    float k0, k1, k2, k3, k4, wl, wm, ws;

    if (-1.88170328 * a - 0.80936493 * b > 1.0) {
        // Red component
        k0 = +1.19086277;
        k1 = +1.76576728;
        k2 = +0.59662641;
        k3 = +0.75515197;
        k4 = +0.56771245;
        wl = +4.0767416621;
        wm = -3.3077115913;
        ws = +0.2309699292;
    } else if (1.81444104 * a - 1.19445276 * b > 1.0) {
        // Green component
        k0 = +0.73956515;
        k1 = -0.45954404;
        k2 = +0.08285427;
        k3 = +0.12541070;
        k4 = +0.14503204;
        wl = -1.2684380046;
        wm = +2.6097574011;
        ws = -0.3413193965;
    } else {
        // Blue component
        k0 = +1.35733652;
        k1 = -0.00915799;
        k2 = -1.15130210;
        k3 = -0.50559606;
        k4 = +0.00692167;
        wl = -0.0041960863;
        wm = -0.7034186147;
        ws = +1.7076147010;
    }

    // Approximate max saturation using a polynomial:
    float S = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

    // Do one step Halley's method to get closer
    // this gives an error less than 10e6, except for some blue hues where the dS/dh is close to infinite
    // this should be sufficient for most applications, otherwise do two/three steps

    float k_l = +0.3963377774 * a + 0.2158037573 * b;
    float k_m = -0.1055613458 * a - 0.0638541728 * b;
    float k_s = -0.0894841775 * a - 1.2914855480 * b;

    {
        float l_ = 1.0 + S * k_l;
        float m_ = 1.0 + S * k_m;
        float s_ = 1.0 + S * k_s;

        float l = l_ * l_ * l_;
        float m = m_ * m_ * m_;
        float s = s_ * s_ * s_;

        float l_dS = 3.0 * k_l * l_ * l_;
        float m_dS = 3.0 * k_m * m_ * m_;
        float s_dS = 3.0 * k_s * s_ * s_;

        float l_dS2 = 6.0 * k_l * k_l * l_;
        float m_dS2 = 6.0 * k_m * k_m * m_;
        float s_dS2 = 6.0 * k_s * k_s * s_;

        float f = wl * l + wm * m + ws * s;
        float f1 = wl * l_dS + wm * m_dS + ws * s_dS;
        float f2 = wl * l_dS2 + wm * m_dS2 + ws * s_dS2;

        S = S - f * f1 / (f1 * f1 - 0.5 * f * f2);
    }

    return S;
}

vec2 find_cusp(float a, float b) {
    // First, find the maximum saturation (saturation S = C/L)
    float S_cusp = compute_max_saturation(a, b);

    // Convert to linear sRGB to find the first point where at least one of r,g or b >= 1:
    vec3 rgb_at_max = oklab_to_linear_srgb(1, S_cusp * a, S_cusp * b);
    float L_cusp = pow(1.0 / max(max(rgb_at_max.x, rgb_at_max.y), rgb_at_max.z), 1.0 / 3.0);
    float C_cusp = L_cusp * S_cusp;

    return vec2(L_cusp, C_cusp);
}

float find_gamut_intersection(float a, float b, float L1, float C1, float L0, vec2 cusp) {
    // Find the intersection for upper and lower half seprately
    float t;
    if (((L1 - L0) * cusp.y - (cusp.x - L0) * C1) <= 0.0) {
        // Lower half

        t = cusp.y * L0 / (C1 * cusp.x + cusp.y * (L0 - L1));
    } else {
        // Upper half

        // First intersect with triangle
        t = cusp.y * (L0 - 1.0) / (C1 * (cusp.x - 1.0) + cusp.y * (L0 - L1));

        // Then one step Halley's method
        {
            float dL = L1 - L0;
            float dC = C1;

            float k_l = +0.3963377774 * a + 0.2158037573 * b;
            float k_m = -0.1055613458 * a - 0.0638541728 * b;
            float k_s = -0.0894841775 * a - 1.2914855480 * b;

            float l_dt = dL + dC * k_l;
            float m_dt = dL + dC * k_m;
            float s_dt = dL + dC * k_s;

            // If higher accuracy is required, 2 or 3 iterations of the following block can be used:
            {
                float L = L0 * (1.0 - t) + t * L1;
                float C = t * C1;

                float l_ = L + C * k_l;
                float m_ = L + C * k_m;
                float s_ = L + C * k_s;

                float l = l_ * l_ * l_;
                float m = m_ * m_ * m_;
                float s = s_ * s_ * s_;

                float ldt = 3.0 * l_dt * l_ * l_;
                float mdt = 3.0 * m_dt * m_ * m_;
                float sdt = 3.0 * s_dt * s_ * s_;

                float ldt2 = 6.0 * l_dt * l_dt * l_;
                float mdt2 = 6.0 * m_dt * m_dt * m_;
                float sdt2 = 6.0 * s_dt * s_dt * s_;

                float r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s - 1.0;
                float r1 = 4.0767416621 * ldt - 3.3077115913 * mdt + 0.2309699292 * sdt;
                float r2 = 4.0767416621 * ldt2 - 3.3077115913 * mdt2 + 0.2309699292 * sdt2;

                float u_r = r1 / (r1 * r1 - 0.5 * r * r2);
                float t_r = -r * u_r;

                float g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s - 1.0;
                float g1 = -1.2684380046 * ldt + 2.6097574011 * mdt - 0.3413193965 * sdt;
                float g2 = -1.2684380046 * ldt2 + 2.6097574011 * mdt2 - 0.3413193965 * sdt2;

                float u_g = g1 / (g1 * g1 - 0.5 * g * g2);
                float t_g = -g * u_g;

                float b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s - 1.0;
                float b1 = -0.0041960863 * ldt - 0.7034186147 * mdt + 1.7076147010 * sdt;
                float b2 = -0.0041960863 * ldt2 - 0.7034186147 * mdt2 + 1.7076147010 * sdt2;

                float u_b = b1 / (b1 * b1 - 0.5 * b * b2);
                float t_b = -b * u_b;

                t_r = u_r >= 0.0 ? t_r : 10e5;
                t_g = u_g >= 0.0 ? t_g : 10e5;
                t_b = u_b >= 0.0 ? t_b : 10e5;

                t += min(t_r, min(t_g, t_b));
            }
        }
    }

    return t;
}

vec2 get_ST_max(float a_, float b_, vec2 cusp) {
    float L = cusp.x;
    float C = cusp.y;
    return vec2(C / L, C / (1.0 - L));
}

vec2 get_ST_mid(float a_, float b_) {
    float S = 0.11516993
        + 1.0
            / (+7.44778970 + 4.15901240 * b_
               + a_
                   * (-2.19557347 + 1.75198401 * b_
                      + a_
                          * (-2.13704948 - 10.02301043 * b_
                             + a_ * (-4.24894561 + 5.38770819 * b_ + 4.69891013 * a_))));

    float T = 0.11239642
        + 1.0
            / (+1.61320320 - 0.68124379 * b_
               + a_
                   * (+0.40370612 + 0.90148123 * b_
                      + a_
                          * (-0.27087943 + 0.61223990 * b_
                             + a_ * (+0.00299215 - 0.45399568 * b_ - 0.14661872 * a_))));

    return vec2(S, T);
}

vec3 get_Cs(float L, float a_, float b_) {
    vec2 cusp = find_cusp(a_, b_);

    float C_max = find_gamut_intersection(a_, b_, L, 1, L, cusp);
    vec2 ST_max = get_ST_max(a_, b_, cusp);

    float S_mid = 0.11516993
        + 1.0
            / (+7.44778970 + 4.15901240 * b_
               + a_
                   * (-2.19557347 + 1.75198401 * b_
                      + a_
                          * (-2.13704948 - 10.02301043 * b_
                             + a_ * (-4.24894561 + 5.38770819 * b_ + 4.69891013 * a_))));

    float T_mid = 0.11239642
        + 1.0
            / (+1.61320320 - 0.68124379 * b_
               + a_
                   * (+0.40370612 + 0.90148123 * b_
                      + a_
                          * (-0.27087943 + 0.61223990 * b_
                             + a_ * (+0.00299215 - 0.45399568 * b_ - 0.14661872 * a_))));

    float k = C_max / min((L * ST_max.x), (1.0 - L) * ST_max.y);

    float C_mid;
    {
        float C_a = L * S_mid;
        float C_b = (1.0 - L) * T_mid;

        C_mid =
            0.9 * k * sqrt(sqrt(1.0 / (1.0 / (C_a * C_a * C_a * C_a) + 1.0 / (C_b * C_b * C_b * C_b))));
    }

    float C_0;
    {
        float C_a = L * 0.4;
        float C_b = (1.0 - L) * 0.8;

        C_0 = sqrt(1.0 / (1.0 / (C_a * C_a) + 1.0 / (C_b * C_b)));
    }

    return vec3(C_0, C_mid, C_max);
}

vec3 okhsl_to_srgb(float h, float s, float l) {
    if (l == 1.0) {
        return vec3(1.0, 1.0, 1.0);
    } else if (l == 0.0) {
        return vec3(0.0, 0.0, 0.0);
    }

    float a_ = cos(2.0 * PI * h);
    float b_ = sin(2.0 * PI * h);
    float L = toe_inv(l);

    vec3 Cs = get_Cs(L, a_, b_);
    float C_0 = Cs.x;
    float C_mid = Cs.y;
    float C_max = Cs.z;

    float C, t, k_0, k_1, k_2;
    if (s < 0.8) {
        t = 1.25 * s;
        k_0 = 0.0;
        k_1 = 0.8 * C_0;
        k_2 = (1.0 - k_1 / C_mid);
    } else {
        t = 5.0 * (s - 0.8);
        k_0 = C_mid;
        k_1 = 0.2 * C_mid * C_mid * 1.25 * 1.25 / C_0;
        k_2 = (1.0 - (k_1) / (C_max - C_mid));
    }

    C = k_0 + t * k_1 / (1.0 - k_2 * t);

    // If we would only use one of the Cs:
    //C = s*C_0;
    //C = s*1.25*C_mid;
    //C = s*C_max;

    vec3 rgb = oklab_to_linear_srgb(L, C * a_, C * b_);
    return vec3(
        srgb_transfer_function(rgb.x),
        srgb_transfer_function(rgb.y),
        srgb_transfer_function(rgb.z)
    );
}

vec3 srgb_to_okhsl(float r, float g, float b) {
    vec3 lab = linear_srgb_to_oklab(
        srgb_transfer_function_inv(r),
        srgb_transfer_function_inv(g),
        srgb_transfer_function_inv(b)
    );

    float C = sqrt(lab.y * lab.y + lab.z * lab.z);
    float a_ = lab.y / C;
    float b_ = lab.z / C;

    float L = lab.x;
    float h = 0.5 + 0.5 * atan(-lab.z, -lab.y) / PI;

    vec3 Cs = get_Cs(L, a_, b_);
    float C_0 = Cs.x;
    float C_mid = Cs.y;
    float C_max = Cs.z;

    float s;
    if (C < C_mid) {
        float k_0 = 0.0;
        float k_1 = 0.8 * C_0;
        float k_2 = (1.0 - k_1 / C_mid);

        float t = (C - k_0) / (k_1 + k_2 * (C - k_0));
        s = t * 0.8;
    } else {
        float k_0 = C_mid;
        float k_1 = 0.2 * C_mid * C_mid * 1.25 * 1.25 / C_0;
        float k_2 = (1.0 - (k_1) / (C_max - C_mid));

        float t = (C - k_0) / (k_1 + k_2 * (C - k_0));
        s = 0.8 + 0.2 * t;
    }

    float l = toe(L);
    return vec3(h, s, l);
}

vec3 okhsv_to_srgb(float h, float s, float v) {
    float a_ = cos(2.0 * PI * h);
    float b_ = sin(2.0 * PI * h);

    vec2 ST_max = get_ST_max(a_, b_, find_cusp(a_, b_));
    float S_max = ST_max.x;
    float S_0 = 0.5;
    float T = ST_max.y;
    float k = 1.0 - S_0 / S_max;

    float L_v = 1.0 - s * S_0 / (S_0 + T - T * k * s);
    float C_v = s * T * S_0 / (S_0 + T - T * k * s);

    float L = v * L_v;
    float C = v * C_v;

    // to present steps along the way
    //L = v;
    //C = v*s*S_max;
    //L = v*(1 - s*S_max/(S_max+T));
    //C = v*s*S_max*T/(S_max+T);

    float L_vt = toe_inv(L_v);
    float C_vt = C_v * L_vt / L_v;

    float L_new = toe_inv(L); // * L_v/L_vt;
    C = C * L_new / L;
    L = L_new;

    vec3 rgb_scale = oklab_to_linear_srgb(L_vt, a_ * C_vt, b_ * C_vt);
    float scale_L = pow(1.0 / (max(rgb_scale.x, max(rgb_scale.y, max(rgb_scale.z, 0.0f)))), 1.0 / 3.0);

    // remove to see effect without rescaling
    L = L * scale_L;
    C = C * scale_L;

    vec3 rgb = oklab_to_linear_srgb(L, C * a_, C * b_);
    return vec3(
        srgb_transfer_function(rgb.x),
        srgb_transfer_function(rgb.y),
        srgb_transfer_function(rgb.z)
    );
}

vec3 srgb_to_okhsv(float r, float g, float b) {
    vec3 lab = linear_srgb_to_oklab(
        srgb_transfer_function_inv(r),
        srgb_transfer_function_inv(g),
        srgb_transfer_function_inv(b)
    );

    float C = sqrt(lab.y * lab.y + lab.z * lab.z);
    float a_ = lab.y / C;
    float b_ = lab.z / C;

    float L = lab.x;
    float h = 0.5 + 0.5 * atan(-lab.z, -lab.y) / PI;

    vec2 ST_max = get_ST_max(a_, b_, find_cusp(a_, b_));
    float S_max = ST_max.x;
    float S_0 = 0.5;
    float T = ST_max.y;
    float k = 1.0 - S_0 / S_max;

    float t = T / (C + L * T);
    float L_v = t * L;
    float C_v = t * C;

    float L_vt = toe_inv(L_v);
    float C_vt = C_v * L_vt / L_v;

    vec3 rgb_scale = oklab_to_linear_srgb(L_vt, a_ * C_vt, b_ * C_vt);
    float scale_L = pow(1.0 / (max(rgb_scale.x, max(rgb_scale.y, max(rgb_scale.z, 0.0f)))), 1.0 / 3.0);

    L = L / scale_L;
    C = C / scale_L;

    C = C * toe(L) / L;
    L = toe(L);

    float v = L / L_v;
    float s = (S_0 + T) * C_v / ((T * S_0) + T * k * C_v);

    return vec3(h, s, v);
}