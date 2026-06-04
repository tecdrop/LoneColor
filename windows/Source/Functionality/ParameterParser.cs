//----------------------------------------------------------------------------------------------------------------------
//
// <copyright file="ParameterParser.cs" company="Tecdrop">
// Copyright (c) 2012-2023 Tecdrop
// https://www.tecdrop.com/
// Licensed under the MIT License. See LICENSE file in the project root for full license information.
// </copyright>
//
// Description: Parses the parameters from the command line and/or from the file name.
//
//----------------------------------------------------------------------------------------------------------------------

namespace LoneColor
{
    using System;
    using System.Collections.Generic;
    using System.Drawing;
    using System.Globalization;

    /// <summary>
    /// Parses the parameters from the command line and/or from the file name.
    /// </summary>
    internal class ParameterParser
    {
        /// <summary>
        /// Keeps a copy of the initial duplicate color parameter.
        /// </summary>
        private string duplicateColor;

        /// <summary>
        /// Initializes a new instance of the <see cref="ParameterParser"/> class.
        /// </summary>
        public ParameterParser()
        {
            // Set the default parameter values
            this.Silent = false;
            this.Color = Color.Empty;
            this.duplicateColor = string.Empty;
        }

        /// <summary>
        /// Gets a value indicating whether the Silent parameter (that disables audio success or error
        /// notifications) is on.
        /// </summary>
        public bool Silent { get; private set; }

        /// <summary>
        /// Gets the value of the Color parameter.
        /// </summary>
        public Color Color { get; private set; }

        /// <summary>
        /// Gets a value indicating whether the Choose parameter (that opens Desktop Wallpaper in
        /// Control Panel) is on.
        /// </summary>
        public bool Choose { get; private set; }

        /// <summary>
        /// Parses the parameters from the specified file name.
        /// </summary>
        /// <param name="fileName">The file name.</param>
        public void Parse(string fileName)
        {
            List<string> nameParameters = new List<string>(fileName.Split(' '));
            nameParameters.RemoveAt(0);

            for (int i = 0; i < nameParameters.Count; i++)
            {
                string parameter = nameParameters[i].Trim();

                if (string.IsNullOrEmpty(parameter))
                {
                    continue;
                }

                if (this.Silent = this.TestSwitch(parameter, Properties.Resources.LongSilentParameter, Properties.Resources.ShortSilentParameter))
                {
                    continue;
                }

                if (this.Choose = this.TestSwitch(parameter, Properties.Resources.LongChooseParameter, Properties.Resources.ShortChooseParameter))
                {
                    continue;
                }

                if (this.TestForColor(parameter))
                {
                    continue;
                }

                // If we are here, we must have encountered an invalid parameter
                throw new ArgumentException(string.Format(
                    CultureInfo.CurrentCulture,
                    Properties.Resources.InvalidParameterMessage,
                    parameter));
            }
        }

        /// <summary>
        /// Checks for duplicate color parameters.
        /// </summary>
        /// <param name="parameter">The parameter value.</param>
        private void CheckForDuplicates(string parameter)
        {
            if (!string.IsNullOrEmpty(this.duplicateColor))
            {
                throw new ArgumentException(string.Format(
                    CultureInfo.CurrentCulture,
                    Properties.Resources.DuplicateColorMessage,
                    this.duplicateColor,
                    parameter));
            }
            else
            {
                this.duplicateColor = parameter;
            }
        }

        /// <summary>
        /// Tests if a parameter is a specified switch.
        /// </summary>
        /// <param name="parameter">The parameter value.</param>
        /// <param name="longSwitch">The long switch value.</param>
        /// <param name="shortSwitch">The short switch value.</param>
        /// <returns>True if the parameter is the specified switch, false otherwise.</returns>
        private bool TestSwitch(string parameter, string longSwitch, string shortSwitch)
        {
            return parameter.Equals(longSwitch, StringComparison.OrdinalIgnoreCase) ||
                parameter.Equals(shortSwitch, StringComparison.OrdinalIgnoreCase);
        }

        /// <summary>
        /// Test for the number of guids parameter.
        /// </summary>
        /// <param name="parameter">The parameter value.</param>
        /// <returns>True if the parameter is a count value, false otherwise.</returns>
        private bool TestForColor(string parameter)
        {
            Color color = Color.Empty;
            try
            {
                color = ColorTranslator.FromHtml(parameter);
            }
            catch (Exception)
            {
            }

            if (color != Color.Empty)
            {
                this.CheckForDuplicates(parameter);
                this.Color = color;
                return true;
            }

            return false;
        }
    }
}
