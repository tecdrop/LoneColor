//----------------------------------------------------------------------------------------------------------------------
//
// <copyright file="WallpaperColorChanger.cs" company="Appgramming">
// Copyright (c) 2012-2017 Appgramming
// https://www.appgramming.com/
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
// </copyright>
//
// Description: Contains a method that sets the current Desktop Background (Wallpaper) to a solid color.
//
//----------------------------------------------------------------------------------------------------------------------

namespace LoneColor
{
    using System;
    using System.Drawing;
    using System.Drawing.Imaging;
    using System.Globalization;
    using System.IO;
    using System.Runtime.InteropServices;
    using Microsoft.Win32;

    /// <summary>
    /// Contains a method that sets the current Desktop Background (Wallpaper) to a solid color.
    /// </summary>
    public static class WallpaperColorChanger
    {
        /// <summary>
        /// Sets the current Desktop Background (Wallpaper) to the specified solid color.
        /// </summary>
        /// <param name="color">The color to set.</param>
        public static void SetColor(Color color)
        {
            WallpaperColorChanger.SetColorBackground(color);
            WallpaperColorChanger.RemoveCurrentWallpaper(color);
            ////WallpaperColorChanger.SetColorBackground(color);
            WallpaperColorChanger.PersistColorBackground(color);
        }

        /// <summary>
        /// Removes the current wallpaper.
        /// </summary>
        /// <param name="color">A color value.</param>
        private static void RemoveCurrentWallpaper(Color color)
        {
            // Create a temporary bitmap in order to disable the Windows 7 Slideshow
            string tempBitmapFile = string.Empty;
            tempBitmapFile = Path.GetTempFileName();
            Path.ChangeExtension(tempBitmapFile, ".BMP");
            Bitmap tempBitmap = new Bitmap(100, 100);
            using (Graphics graphics = Graphics.FromImage(tempBitmap))
            {
                Brush brush = new SolidBrush(color);
                graphics.FillRectangle(brush, 0, 0, tempBitmap.Width, tempBitmap.Height);
                brush.Dispose();
            }

            tempBitmap.Save(tempBitmapFile, ImageFormat.Bmp);

            try
            {
                // Remove the current wallpaper, and make sure Windows 7 SlideShow feature gets disabled
                NativeMethods.SystemParametersInfo(
                    NativeMethods.SPI_SETDESKWALLPAPER,
                    0,
                    string.Empty,
                    NativeMethods.SPIF_UPDATEINIFILE | NativeMethods.SPIF_SENDWININICHANGE);
                NativeMethods.SystemParametersInfo(
                    NativeMethods.SPI_SETDESKWALLPAPER,
                    0,
                    tempBitmapFile,
                    NativeMethods.SPIF_UPDATEINIFILE | NativeMethods.SPIF_SENDWININICHANGE);
                NativeMethods.SystemParametersInfo(
                    NativeMethods.SPI_SETDESKWALLPAPER,
                    0,
                    string.Empty,
                    NativeMethods.SPIF_UPDATEINIFILE | NativeMethods.SPIF_SENDWININICHANGE);
            }
            finally
            {
                // Make sure the temporary bitmap file gets deleted
                File.Delete(tempBitmapFile);
            }
        }

        /// <summary>
        /// Sets the new desktop solid color for the current session.
        /// </summary>
        /// <param name="color">A color value.</param>
        private static void SetColorBackground(Color color)
        {
            int[] elements = { NativeMethods.COLOR_DESKTOP };
            int[] colors = { System.Drawing.ColorTranslator.ToWin32(color) };
            NativeMethods.SetSysColors(elements.Length, elements, colors);
        }

        /// <summary>
        /// Saves the color value in registry so that it will persist.
        /// </summary>
        /// <param name="color">A color value.</param>
        private static void PersistColorBackground(Color color)
        {
            RegistryKey key = Registry.CurrentUser.OpenSubKey("Control Panel\\Colors", true);
            key.SetValue(@"Background", string.Format(CultureInfo.InvariantCulture, "{0} {1} {2}", color.R, color.G, color.B));
        }

        /// <summary>
        /// Required native (WinAPI) methods and constants.
        /// </summary>
        private static class NativeMethods
        {
            /// <summary>
            /// Tell SetSysColors to set the color of the desktop.
            /// </summary>
            public const int COLOR_DESKTOP = 1;

            /// <summary>
            /// Tell SystemParametersInfo to set the desktop wallpaper.
            /// </summary>
            public const int SPI_SETDESKWALLPAPER = 20;

            /// <summary>
            /// Tell SystemParametersInfo to writes the new system-wide parameter setting to the user
            /// profile.
            /// </summary>
            public const int SPIF_UPDATEINIFILE = 0x01;

            /// <summary>
            /// Tell SystemParametersInfo to broadcast the WM_SETTINGCHANGE message after updating the
            /// user profile.
            /// </summary>
            public const int SPIF_SENDWININICHANGE = 0x02;

            [DllImport("user32.dll")]
            public static extern bool SetSysColors(int cElements, int[] lpaElements, int[] lpaRgbValues);

            [DllImport("user32.dll", CharSet = CharSet.Auto, SetLastError = true)]
            public static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);
        }
    }
}
