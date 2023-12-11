//----------------------------------------------------------------------------------------------------------------------
//
// <copyright file="ApplicationStartupShortcut.cs" company="Tecdrop">
// Copyright (c) 2012-2023 Tecdrop
// https://www.tecdrop.com/
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
// </copyright>
//
// Description: Gets the path and name of the shortcut file (.lnk) that the user invoked to start the application.
//
//----------------------------------------------------------------------------------------------------------------------

namespace LoneColor
{
    using System;
    using System.Diagnostics.CodeAnalysis;
    using System.Runtime.InteropServices;

    /// <summary>
    /// Gets the path and name of the shortcut file (.LNK) that the user invoked to start the application.
    /// </summary>
    internal static class ApplicationStartupShortcut
    {
        /// <summary>
        /// Returns the path and name of the shortcut file (.LNK) that the user invoked to start the
        /// application, or an empty string if the application was not started by using a shortcut file.
        /// </summary>
        /// <returns>The path and name of the shortcut file.</returns>
        public static string GetShortcutPath()
        {
            NativeMethods.STARTUPINFO_I si = new NativeMethods.STARTUPINFO_I();
            NativeMethods.GetStartupInfo(si);

            if ((si.dwFlags & NativeMethods.STARTF_TITLEISLINKNAME) == NativeMethods.STARTF_TITLEISLINKNAME)
            {
                return Marshal.PtrToStringAuto(si.lpTitle);
            }

            return string.Empty;
        }

        /// <summary>
        /// Returns the path and name of the shortcut file (.LNK) that the user invoked to start the
        /// application, or the specified default string value if the application was not started by
        /// using a shortcut file.
        /// </summary>
        /// <param name="defaultValue">A default string value.</param>
        /// <returns>The path and name of the shortcut file.</returns>
        public static string GetShortcutPath(string defaultValue)
        {
            string shortcutPath = ApplicationStartupShortcut.GetShortcutPath();
            return !string.IsNullOrEmpty(shortcutPath) ? shortcutPath : defaultValue;
        }

        /// <summary>
        /// Contains required Windows API native methods, structures and constants.
        /// </summary>
        private static class NativeMethods
        {
            /// <summary>
            /// Indicated that the Title member of the STARTUPINFO_I structure contains the path of
            /// the shortcut file (.LNK) that the user invoked to start this process.
            /// </summary>
            public const int STARTF_TITLEISLINKNAME = 0x00000800;

            /// <summary>
            /// Retrieves the contents of the STARTUPINFO structure that was specified when the
            /// calling process was created.
            /// </summary>
            /// <param name="startupinfo_i">A pointer to a STARTUPINFO structure that receives the
            /// startup information.</param>
            [DllImport("kernel32.dll", CharSet = CharSet.Auto)]
            public static extern void GetStartupInfo([In, Out] NativeMethods.STARTUPINFO_I startupinfo_i);

            /// <summary>
            /// Specifies the window station, desktop, standard handles, and appearance of the main
            /// window for a process at creation time.
            /// </summary>
            [SuppressMessage("StyleCop.CSharp.MaintainabilityRules", "SA1401:FieldsMustBePrivate", Justification = "Reviewed.")]
            [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Auto)]
            public class STARTUPINFO_I
            {
                public int cb = 0;
                public IntPtr lpReserved = IntPtr.Zero;
                public IntPtr lpDesktop = IntPtr.Zero;
                public IntPtr lpTitle = IntPtr.Zero;
                public int dwX = 0;
                public int dwY = 0;
                public int dwXSize = 0;
                public int dwYSize = 0;
                public int dwXCountChars = 0;
                public int dwYCountChars = 0;
                public int dwFillAttribute = 0;
                public int dwFlags = 0;
                public short wShowWindow = 0;
                public short cbReserved2 = 0;
                public IntPtr lpReserved2 = IntPtr.Zero;
                public IntPtr hStdInput = IntPtr.Zero;
                public IntPtr hStdOutput = IntPtr.Zero;
                public IntPtr hStdError = IntPtr.Zero;
            }
        }
    }
}