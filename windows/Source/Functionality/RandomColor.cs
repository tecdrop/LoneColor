//----------------------------------------------------------------------------------------------------------------------
//
// <copyright file="RandomColor.cs" company="Tecdrop">
// Copyright (c) 2012-2023 Tecdrop
// https://www.tecdrop.com/
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
// </copyright>
//
// Description: Provides a method that generate more true random Colors.
//
//----------------------------------------------------------------------------------------------------------------------

namespace LoneColor
{
    using System;
    using System.Collections.Generic;
    using System.Drawing;
    using System.Security.Cryptography;

    /// <summary>
    /// Provides a method that generate more true random Colors.
    /// </summary>
    public static class RandomColor
    {
        /// <summary>
        /// Returns a random Color.
        /// </summary>
        /// <returns>The random Color value.</returns>
        public static Color GetOne()
        {
            RNGCryptoServiceProvider rngRandom = new RNGCryptoServiceProvider();

            byte[] randomRGB = new byte[3];
            rngRandom.GetBytes(randomRGB);

            return Color.FromArgb(randomRGB[0], randomRGB[1], randomRGB[2]);
        }
    }
}
