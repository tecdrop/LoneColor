//----------------------------------------------------------------------------------------------------------------------
//
// <copyright file="ClipboardParser.cs" company="Appgramming">
// Copyright (c) 2012-2017 Appgramming
// https://www.appgramming.com/
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
// </copyright>
//
// Description: Gets a valid color value from the Clipboard.
//
//----------------------------------------------------------------------------------------------------------------------

namespace LoneColor
{
    using System;
    using System.Drawing;
    using System.Windows.Forms;

    /// <summary>
    /// Contains a static method that gets a valid color value from the Clipboard.
    /// </summary>
    public static class ClipboardParser
    {
        /// <summary>
        /// Returns a valid color value from the Clipboard.
        /// </summary>
        /// <returns>A color value.</returns>
        public static Color GetColor()
        {
            try
            {
                if (Clipboard.ContainsText())
                {
                    string text = Clipboard.GetText();

                    Color color = GetValidColor(text);
                    if (!color.IsEmpty)
                    {
                        return color;
                    }
                    else
                    {
                        color = GetValidColor("#" + text);
                        if (!color.IsEmpty)
                        {
                            return color;
                        }
                    }
                }
            }
            catch (Exception)
            {
            }

            return Color.Empty;
        }

        private static Color GetValidColor(string parameter)
        {
            try
            {
                Color color = ColorTranslator.FromHtml(parameter);
                return color;
            }
            catch (Exception)
            {
            }

            return Color.Empty;
        }
    }
}