/*
 * Copyright (c) 2014-2026 Tecdrop SRL
 * https://www.tecdrop.com
 *
 * Use of this source code is governed by an MIT-style license that can be
 * found in the LICENSE file.
 */


package com.appgramming.lonecolor.utils;

import android.content.ClipData;
import android.content.ClipboardManager;
import android.content.Context;
import android.content.Intent;

import com.appgramming.lonecolor.R;

/**
 * Static utility methods.
 */
public final class Utils {

    private Utils() {
    }

    /**
     * Converts an integer color value to a hexadecimal string.
     */
    public static String colorToHex(int color) {
        return String.format("#%06X", 0xFFFFFF & color);
    }

    /**
     * Copies a text to clipboard.
     */
    public static void copyText(Context context, CharSequence text) {
        final ClipboardManager clipboard = (ClipboardManager) context.getSystemService(Context.CLIPBOARD_SERVICE);
        final ClipData clip = ClipData.newPlainText(context.getString(R.string.app_name), text);
        clipboard.setPrimaryClip(clip);
    }

    /**
     * Goes home.
     */
    public static void goHome(Context context) {
        Intent startMain = new Intent(Intent.ACTION_MAIN);
        startMain.addCategory(Intent.CATEGORY_HOME);
        startMain.setFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        context.startActivity(startMain);
    }
}
