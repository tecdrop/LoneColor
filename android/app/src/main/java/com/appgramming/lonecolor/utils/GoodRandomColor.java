/*
 * Copyright (C) 2014-2022 Tecdrop
 * https://www.tecdrop.com
 *
 * Use of this source code is governed by an MIT-style license that can be
 * found in the LICENSE file.
 */

package com.appgramming.lonecolor.utils;

import android.graphics.Color;

import java.util.Random;

/**
 * Utility class to create good pseudo-random RGB colors.
 */
public final class GoodRandomColor {

    /**
     * The maximum value (minus one) of a RGB value.
     */
    private static final int MAX_RGB = 256;

    /**
     * Mask used to access the alpha component of a color int.
     */
    private static final int FULL_ALPHA = 0xFF000000;

    /**
     * The random generator used to create random colors. It is static so it will not be
     * recreated too often by sequent runs of the app.
     */
    private static final Random sRandom = new Random();

    private GoodRandomColor() {
    }

    /**
     * Return a pseudo-random RGB color value, created from a raw integer value.
     */
    private static int nextRawColor() {
        return FULL_ALPHA | sRandom.nextInt();
    }

    /**
     * Return a pseudo-random RGB color value, created from random red, green and blue values.
     */
    private static int nextRGBColor() {
        final int red = sRandom.nextInt(MAX_RGB);
        final int green = sRandom.nextInt(MAX_RGB);
        final int blue = sRandom.nextInt(MAX_RGB);
        return Color.rgb(red, green, blue);
    }

    /**
     * Return a pseudo-random RGB color value mixed with the specified color.
     *
     * @param mixColor The color to use for mixing.
     */
    private static int nextMixedColor(int mixColor) {
        final int red = (sRandom.nextInt(MAX_RGB) + Color.red(mixColor)) / 2;
        final int green = (sRandom.nextInt(MAX_RGB) + Color.green(mixColor)) / 2;
        final int blue = (sRandom.nextInt(MAX_RGB) + Color.blue(mixColor)) / 2;
        return Color.rgb(red, green, blue);
    }

    /**
     * Return a pseudo-random RGB color value, created by various methods for better results.
     */
    public static int nextColor() {

        int color = Color.BLACK;

        // Select a random method
        final int selector = sRandom.nextInt(10);
        switch (selector) {
            case 0:
                color = nextRawColor();
                break;
            case 1:
                color = nextRGBColor();
                break;
            case 2:
                color = nextMixedColor(Color.BLACK);
                break;
            case 3:
                color = nextMixedColor(Color.WHITE);
                break;
            case 4:
                color = nextMixedColor(Color.RED);
                break;
            case 5:
                color = nextMixedColor(Color.GREEN);
                break;
            case 6:
                color = nextMixedColor(Color.BLUE);
                break;
            case 7:
                color = nextMixedColor(Color.YELLOW);
                break;
            case 8:
                color = nextMixedColor(Color.CYAN);
                break;
            case 9:
                color = nextMixedColor(Color.MAGENTA);
                break;
        }

        return color;
    }
}