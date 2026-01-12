/*
 * Copyright (c) 2014-2026 Tecdrop SRL
 * https://www.tecdrop.com
 *
 * Use of this source code is governed by an MIT-style license that can be
 * found in the LICENSE file.
 */


package com.appgramming.lonecolor.utils;

import android.app.WallpaperManager;
import android.content.Context;
import android.graphics.Bitmap;
import android.os.Build;

import java.io.IOException;

/**
 * Utility class for setting a color wallpaper.
 */

public final class ColorWallpaper {

    /**
     * The minimum safe wallpaper pixel size.
     */
    private static final int MIN_SAFE_SIZE = 100;

    private ColorWallpaper() {
    }

    /**
     * Create a color filled bitmap and changes the current system wallpaper to this bitmap.
     */
    public static void setColorWallpaper(Context context, int color) throws IOException {

        // Get the Wallpaper Manager
        final WallpaperManager wpManager = WallpaperManager.getInstance(context);

        // Create the color bitmap
        final Bitmap colorBitmap = createColorBitmap(color, MIN_SAFE_SIZE, MIN_SAFE_SIZE);

        // Set the wallpaper
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
            // On Android N and above use the new API to set both the general system wallpaper and
            // the lock-screen-specific wallpaper
            wpManager.setBitmap(colorBitmap, null, true, WallpaperManager.FLAG_SYSTEM | WallpaperManager.FLAG_LOCK);
        } else {
            wpManager.setBitmap(colorBitmap);
        }
    }

    /**
     * Returns a mutable bitmap filled with the specified color.
     */
    @SuppressWarnings("SameParameterValue")
    private static Bitmap createColorBitmap(int color, int width, int height) {
        final Bitmap colorBitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888);
        colorBitmap.eraseColor(color);
        return colorBitmap;
    }
}
