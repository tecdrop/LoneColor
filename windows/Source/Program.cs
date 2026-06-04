//----------------------------------------------------------------------------------------------------------------------
//
// <copyright file="Program.cs" company="Tecdrop">
// Copyright (c) 2012-2023 Tecdrop
// https://www.tecdrop.com/
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
// </copyright>
//
// Description: The main LoneColor program class.
//
//----------------------------------------------------------------------------------------------------------------------

namespace LoneColor
{
    using System;
    using System.Drawing;
    using System.Globalization;
    using System.IO;
    using System.Media;
    using System.Windows.Forms;

    /// <summary>
    /// The main LoneColor program class.
    /// </summary>
    public static class Program
    {
        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        private static void Main()
        {
            ParameterParser parameterParser = new ParameterParser();

            try
            {
                // Extract the color and other parameters from the name of the file name that started
                // the application (a shortcut or an executable file)
                string shortcutPath = ApplicationStartupShortcut.GetShortcutPath(Application.ExecutablePath);
                parameterParser.Parse(Path.GetFileNameWithoutExtension(shortcutPath));

                if (parameterParser.Choose)
                {
                    // Opens Desktop Wallpaper Control Panel
                    ControlPanelOpener.OpenDesktopBackground();
                }
                else
                {
                    Color color = parameterParser.Color;

                    // If no color was specified as parameter
                    if (color.IsEmpty)
                    {
                        // Is there a valid color in the Clipboard?
                        color = ClipboardParser.GetColor();

                        // No? Then get a random color
                        if (color.IsEmpty)
                        {
                            color = RandomColor.GetOne();
                        }
                    }

                    // Save the color to the clipboard
                    // Clipboard.SetText(color.ToString());
                    Clipboard.SetText("Color " + ColorTranslator.ToHtml(color));

                    // Set the Windows Desktop Wallpaper to the solid color
                    WallpaperColorChanger.SetColor(color);
                }
            }
            catch (Exception e)
            {
                // If an exception occurs, let the user know by playing the system error sound.
                if (!parameterParser.Silent)
                {
                    SystemSounds.Hand.Play();
                }

                Clipboard.SetText(string.Format(
                    CultureInfo.CurrentCulture,
                    Properties.Resources.ErrorMessage,
                    Application.ProductName,
                    e.Message));
            }
        }
    }
}
