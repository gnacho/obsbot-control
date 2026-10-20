// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Translatable text for the error types the GUI shows in toasts.
//!
//! `obsbot-core` and the preview pipeline carry English `Display`
//! messages. Those are fine in logs but must not reach the UI of a
//! translated app, so every variant the user can see is mapped to a
//! localised sentence here. Unknown variants (both enums are
//! `#[non_exhaustive]`) fall back to the raw message rather than
//! swallowing the detail.

use obsbot_core::xu::XuError;

use crate::i18n::gettext;

#[cfg(feature = "live-preview")]
use crate::preview::PreviewError;

/// Localised description of an XU transport/command failure.
#[must_use]
pub fn xu_error_text(err: &XuError) -> String {
    match err {
        XuError::Io(io) => {
            gettext("The camera did not answer ({error}).").replace("{error}", &io.to_string())
        }
        XuError::LengthMismatch {
            payload_len,
            selector_len,
            ..
        } => gettext(
            "The command is {payload} bytes long and the camera only accepts {limit} on this channel.",
        )
        .replace("{payload}", &payload_len.to_string())
        .replace("{limit}", &selector_len.to_string()),
        XuError::Decode(decode) => gettext("The camera reported an unrecognised state ({error}).")
            .replace("{error}", &decode.to_string()),
        XuError::InvalidPresetIndex(index) => gettext("Preset index {n} is out of range (0 to 2).")
            .replace("{n}", &index.to_string()),
        XuError::InvalidPresetSlot(slot) => gettext("Preset slot {n} is out of range (1 to 3).")
            .replace("{n}", &slot.to_string()),
        other => other.to_string(),
    }
}

/// Localised description of a live-preview failure.
#[cfg(feature = "live-preview")]
#[must_use]
pub fn preview_error_text(err: &PreviewError) -> String {
    match err {
        PreviewError::MissingElement(element) => {
            gettext("The preview needs the GStreamer element {name}, which is not installed.")
                .replace("{name}", element)
        }
        PreviewError::PipelineStart(detail) => gettext(
            "The preview could not start. Is another application using the camera? ({error})",
        )
        .replace("{error}", detail),
        PreviewError::GstInit(detail) => {
            gettext("GStreamer could not be initialised ({error}).").replace("{error}", detail)
        }
    }
}
