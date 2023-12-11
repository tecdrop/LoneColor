//----------------------------------------------------------------------------------------------------------------------
//
// <copyright file="ControlPanelOpener.cs" company="Tecdrop">
// Copyright (c) 2012-2023 Tecdrop
// https://www.tecdrop.com/
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
// </copyright>
//
// Description: Provides a method to open the Control Panel Desktop Background / Wallpaper section.
//
//----------------------------------------------------------------------------------------------------------------------

namespace LoneColor
{
    using System;
    using System.Diagnostics;
    using System.IO;

    /// <summary>
    /// Provides a method to open the Control Panel Desktop Background / Wallpaper section.
    /// </summary>
    public static class ControlPanelOpener
    {
        /// <summary>
        /// The Control Panel shell command.
        /// </summary>
        private const string ControlPanelCommand = "control.exe";

        /// <summary>
        /// The arguments used to open the Desktop Background / Wallpaper Control Panel section on Windows XP.
        /// </summary>
        private const string ControlPanelParametersXP = "desk.cpl,Desktop,@Desktop";

        /// <summary>
        /// The arguments used to open the Desktop Background / Wallpaper Control Panel section on Windows Vista and above.
        /// </summary>
        private const string ControlPanelParametersVista = "/name Microsoft.Personalization /page pageWallpaper";

        /// <summary>
        /// Opens the Desktop Background / Wallpaper Control Panel section.
        /// </summary>
        public static void OpenDesktopBackground()
        {
            Process.Start(
                Path.Combine(Environment.SystemDirectory, ControlPanelOpener.ControlPanelCommand),
                Environment.OSVersion.Version.Major >= 6 ? ControlPanelOpener.ControlPanelParametersVista : ControlPanelOpener.ControlPanelParametersXP);
        }
    }
}
